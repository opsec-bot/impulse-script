use std::hint::spin_loop;
use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };
use std::time::{ Duration, Instant };

#[cfg(windows)]
use winapi::um::winuser::{ GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON };

use crate::modules::input::MouseInput;
use crate::modules::core::logger::{ log_debug };

// Playback tick for pattern mode. Finer than the per-shot interval so each
// shot's recoil step is spread into several small, smooth movements.
const PATTERN_TICK: Duration = Duration::from_micros(2500);

// Captured patterns usually cover only the first few shots. Past the last
// captured shot we keep pulling at the pattern's steady-state velocity instead
// of freezing, so the whole magazine stays compensated. `TAIL_SHOTS` is how many
// trailing segments we average to estimate that velocity; `MAX_EXTRAPOLATED_SHOTS`
// caps how far the pull continues so holding fire on an empty mag can't drag the
// aim arbitrarily far down.
const TAIL_SHOTS: usize = 3;
const MAX_EXTRAPOLATED_SHOTS: f32 = 60.0;

fn precise_sleep(duration: Duration) {
    let start = Instant::now();

    if duration > Duration::from_millis(2) {
        thread::sleep(duration - Duration::from_millis(1));
    }

    while start.elapsed() < duration {
        spin_loop();
    }
}

struct ControlState {
    stop: bool,
    running: bool,
    active: bool,
    move_x: i32,
    move_y: i32,
    move_x_modifier: f32,
    timing: f32,
    x_flip: i32,
    x_once_done: bool,

    sensitivity: i32,
    dpi: i32,
    raw_movement_x: f32,
    raw_movement_y: f32,
    // Sub-pixel residuals carried between shots for precision.
    residual_x: f32,
    residual_y: f32,
    // Global calibration multiplier applied to all compensation (Recoil Scale).
    recoil_scale: f32,

    // --- Pattern playback ---
    // Cumulative compensation curve in output mouse counts. `pattern[0]` is
    // always (0, 0) (shot 1, gun at rest); `pattern[k]` is the total counts to
    // have moved by shot k+1. Empty => fall back to constant-pull mode.
    pattern: Vec<(f32, f32)>,
    // Milliseconds between shots (60000 / RPM).
    shot_ms: f32,
    // Start of the current trigger burst; `None` until the first active tick.
    burst_start: Option<Instant>,
    // Counts already emitted this burst (so we only ever send the delta needed
    // to reach the interpolated cumulative target).
    emitted_x: f32,
    emitted_y: f32,
}
pub struct Control {
    thread: Option<JoinHandle<()>>,
    state: Arc<Mutex<ControlState>>,
    mouse_input: Option<Arc<MouseInput>>,
}

impl Control {
    pub fn new() -> Self {
        log_debug("Creating new Control instance");
        Control {
            thread: None,
            state: Arc::new(
                Mutex::new(ControlState {
                    stop: false,
                    running: false,
                    active: false,
                    move_x: 0,
                    move_y: 0,
                    move_x_modifier: 1.0,
                    timing: 0.0,
                    x_flip: 1,
                    x_once_done: false,
                    sensitivity: 0,
                    dpi: 800,
                    raw_movement_x: 0.0,
                    raw_movement_y: 0.0,
                    residual_x: 0.0,
                    residual_y: 0.0,
                    recoil_scale: 1.0,
                    pattern: Vec::new(),
                    shot_ms: 0.0,
                    burst_start: None,
                    emitted_x: 0.0,
                    emitted_y: 0.0,
                })
            ),
            mouse_input: None,
        }
    }

    pub fn set_mouse_input(&mut self, mouse_input: Arc<MouseInput>) {
        self.mouse_input = Some(mouse_input);
    }

    pub fn run_threaded(&mut self) {
        let state = Arc::clone(&self.state);
        let mouse_input = match self.mouse_input.clone() {
            Some(mouse_input) => mouse_input,
            None => return,
        };
        {
            let mut s = state.lock().unwrap();
            s.running = true;
        }
        self.thread = Some(
            thread::spawn(move || {
                // What to do this tick, decided under the lock then executed
                // after releasing it (mouse DLL calls / sleeps are slow).
                enum Action {
                    Quit,
                    Idle,
                    Move(i32, i32, Duration),
                }

                loop {
                    let action = {
                        let mut s = state.lock().unwrap();

                        if !s.running {
                            Action::Quit
                        } else {
                            s.check_status();

                            if !s.active || s.stop {
                                Action::Idle
                            } else if !s.pattern.is_empty() {
                                // --- Pattern playback (shot-synced + smoothed) ---
                                let now = Instant::now();
                                let start = *s.burst_start.get_or_insert(now);
                                let elapsed_ms =
                                    now.duration_since(start).as_secs_f32() * 1000.0;

                                let (tx, ty) = s.pattern_target(elapsed_ms);
                                let out_x = (tx - s.emitted_x).round() as i32;
                                let out_y = (ty - s.emitted_y).round() as i32;
                                s.emitted_x += out_x as f32;
                                s.emitted_y += out_y as f32;

                                Action::Move(out_x, out_y, PATTERN_TICK)
                            } else {
                                // --- Constant-pull (legacy) ---
                                let timing = s.timing;
                                let (mut x, y) = s.calculate_dpi_adjusted_movement();

                                match s.move_x_modifier as i32 {
                                    -1 => {
                                        x *= s.x_flip;
                                        s.x_flip *= -1;
                                    }
                                    0 => {
                                        if s.x_once_done {
                                            x = 0;
                                        } else {
                                            s.x_once_done = true;
                                        }
                                    }
                                    1 => {}
                                    _ => {
                                        x = ((x as f32) * s.move_x_modifier).round() as i32;
                                    }
                                }

                                Action::Move(x, y, Duration::from_secs_f32(timing))
                            }
                        }
                    };

                    match action {
                        Action::Quit => {
                            break;
                        }
                        Action::Idle => {
                            precise_sleep(Duration::from_millis(1));
                        }
                        Action::Move(x, y, sleep) => {
                            if x != 0 || y != 0 {
                                mouse_input.move_relative(x, y);
                            }
                            precise_sleep(sleep);
                        }
                    }
                }
            })
        );
    }

    pub fn reset(&mut self) {
        log_debug("Resetting control state");
        let mut s = self.state.lock().unwrap();
        s.stop = true;
        s.active = false;
        s.move_x = 0;
        s.move_y = 0;
        s.timing = 0.0;
        s.move_x_modifier = 1.0;
        s.x_flip = 1;
        s.x_once_done = false;
        s.raw_movement_x = 0.0;
        s.raw_movement_y = 0.0;
        s.residual_x = 0.0;
        s.residual_y = 0.0;
        s.pattern.clear();
        s.shot_ms = 0.0;
        s.burst_start = None;
        s.emitted_x = 0.0;
        s.emitted_y = 0.0;
    }

    pub fn set_dpi(&mut self, dpi: i32) {
        log_debug(&format!("Setting DPI to: {}", dpi));
        let mut state = self.state.lock().unwrap();
        state.dpi = dpi;
    }

    pub fn set_sensitivity(&mut self, sensitivity: i32) {
        log_debug(&format!("Setting sensitivity to: {}", sensitivity));
        let mut state = self.state.lock().unwrap();
        state.sensitivity = sensitivity;
    }

    pub fn set_recoil_scale(&mut self, scale: f32) {
        log_debug(&format!("Setting recoil scale to: {:.2}", scale));
        let mut state = self.state.lock().unwrap();
        state.recoil_scale = scale;
    }

    /// Switch the control thread into pattern-playback mode. `comp` is the
    /// cumulative compensation curve in output mouse counts (already DPI / sens /
    /// per-axis-scale adjusted): `comp[0]` should be (0, 0) and `comp[k]` the
    /// total counts to have moved by shot k+1. `shot_ms` is the inter-shot
    /// interval (60000 / RPM). The global Recoil Scale is still applied on top.
    pub fn set_pattern(&mut self, comp: Vec<(f32, f32)>, shot_ms: f32) {
        let mut s = self.state.lock().unwrap();

        // Re-applying the identical pattern (the UI does this every frame while a
        // slider is dragged, and on every enable) must not yank an in-progress
        // burst back to shot 1 — only reset playback when the pattern truly changes.
        let unchanged =
            s.pattern == comp && (s.shot_ms - shot_ms).abs() < 0.001 && !s.pattern.is_empty();
        if unchanged {
            return;
        }

        log_debug(
            &format!("Loading recoil pattern: {} shots, {:.2}ms/shot", comp.len(), shot_ms)
        );
        s.stop = true;
        s.pattern = comp;
        s.shot_ms = shot_ms;
        s.burst_start = None;
        s.emitted_x = 0.0;
        s.emitted_y = 0.0;
        s.x_flip = 1;
        s.x_once_done = false;
        s.stop = false;
    }

    /// Leave pattern mode and fall back to constant-pull.
    pub fn clear_pattern(&mut self) {
        let mut s = self.state.lock().unwrap();
        s.pattern.clear();
        s.shot_ms = 0.0;
        s.burst_start = None;
        s.emitted_x = 0.0;
        s.emitted_y = 0.0;
    }

    pub fn update(&mut self, x: i32, y: i32, t_ms: f32, x_mod: f32) {
        log_debug(
            &format!("Updating recoil values: X={}, Y={}, Timing={:.3}ms, Xmod={}", x, y, t_ms, x_mod)
        );
        let mut s = self.state.lock().unwrap();
        s.stop = true;
        s.active = false;
        // Switching to constant-pull supersedes any loaded pattern.
        s.pattern.clear();
        s.shot_ms = 0.0;
        s.burst_start = None;
        s.emitted_x = 0.0;
        s.emitted_y = 0.0;
        s.move_x = 0;
        s.move_y = 0;
        s.x_flip = 1;
        s.x_once_done = false;
        s.raw_movement_x = x as f32;
        s.raw_movement_y = y as f32;
        s.residual_x = 0.0;
        s.residual_y = 0.0;
        s.timing = t_ms / 1000.0;
        s.move_x_modifier = x_mod;

        let (adjusted_x, adjusted_y) = s.calculate_dpi_adjusted_movement();
        s.move_x = adjusted_x;
        s.move_y = adjusted_y;

        s.stop = false;
    }

}

impl ControlState {
    fn check_status(&mut self) {
        #[cfg(windows)]
        let is_active = unsafe {
            GetAsyncKeyState(VK_RBUTTON) < 0 && GetAsyncKeyState(VK_LBUTTON) < 0
        };

        #[cfg(not(windows))]
        let is_active = false;

        // Rising edge: trigger just pulled. Reset per-burst state.
        if is_active && !self.active {
            self.x_flip = 1;
            self.x_once_done = false;
            self.residual_x = 0.0;
            self.residual_y = 0.0;
            // Restart pattern playback from shot 1.
            self.burst_start = None;
            self.emitted_x = 0.0;
            self.emitted_y = 0.0;
        }

        self.active = is_active;
    }

    /// Cumulative compensation target (in output counts) at `elapsed_ms` into
    /// the burst, linearly interpolated between shots and scaled by recoil_scale.
    fn pattern_target(&self, elapsed_ms: f32) -> (f32, f32) {
        let n = self.pattern.len();
        if n == 0 {
            return (0.0, 0.0);
        }
        let scale = self.recoil_scale;

        // No timing info: just hold the final cumulative compensation.
        if self.shot_ms <= 0.0 {
            let last = self.pattern[n - 1];
            return (last.0 * scale, last.1 * scale);
        }

        let pos = elapsed_ms / self.shot_ms; // fractional shot index

        // Past the last captured shot: keep pulling at the pattern's steady-state
        // per-shot velocity instead of freezing, so the rest of the magazine stays
        // compensated even when only the first few shots were captured.
        if pos >= (n as f32) - 1.0 {
            let (lx, ly) = self.pattern[n - 1];
            let (vx, vy) = self.tail_velocity();
            let extra = (pos - ((n as f32) - 1.0)).min(MAX_EXTRAPOLATED_SHOTS);
            return ((lx + vx * extra) * scale, (ly + vy * extra) * scale);
        }

        let seg = pos.floor() as usize;
        let frac = pos - (seg as f32);
        let (ax, ay) = self.pattern[seg];
        let (bx, by) = self.pattern[seg + 1];
        let tx = ax + (bx - ax) * frac;
        let ty = ay + (by - ay) * frac;
        (tx * scale, ty * scale)
    }

    /// Average per-shot velocity (output counts per shot) over the tail of the
    /// pattern, used to extrapolate compensation past the last captured shot.
    /// For short patterns this collapses to the whole-pattern average slope.
    fn tail_velocity(&self) -> (f32, f32) {
        let n = self.pattern.len();
        if n < 2 {
            return (0.0, 0.0);
        }
        let segs = (n - 1).min(TAIL_SHOTS);
        let (sx, sy) = self.pattern[n - 1];
        let (px, py) = self.pattern[n - 1 - segs];
        ((sx - px) / (segs as f32), (sy - py) / (segs as f32))
    }

    fn calculate_dpi_adjusted_movement(&mut self) -> (i32, i32) {
        const BASE_DPI: f32 = 800.0;
        const BASE_SENSITIVITY: f32 = 30.0;

        if self.dpi == 0 {
            return (
                (self.raw_movement_x * self.recoil_scale) as i32,
                (self.raw_movement_y * self.recoil_scale) as i32,
            );
        }

        let sens_scale = if self.sensitivity > 0 {
            BASE_SENSITIVITY / (self.sensitivity as f32)
        } else {
            1.0
        };

        let dpi_scale = BASE_DPI / (self.dpi as f32);

        let target_x =
            self.raw_movement_x * self.recoil_scale * sens_scale * dpi_scale + self.residual_x;
        let target_y =
            self.raw_movement_y * self.recoil_scale * sens_scale * dpi_scale + self.residual_y;

        let out_x = target_x.round() as i32;
        let out_y = target_y.round() as i32;

        self.residual_x = target_x - out_x as f32;
        self.residual_y = target_y - out_y as f32;

        (out_x, out_y)
    }

}
