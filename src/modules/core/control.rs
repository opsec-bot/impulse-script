use std::hint::spin_loop;
use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };
use std::time::{ Duration, Instant };

#[cfg(windows)]
use winapi::um::winuser::{ GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON };

use crate::modules::input::MouseInput;
use crate::modules::core::logger::{ log_debug };

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
                loop {
                    let (running, active, stop, timing, x, y) = {
                        let mut s = state.lock().unwrap();

                        if !s.running {
                            (false, false, false, 0.0, 0, 0)
                        } else {
                            s.check_status();

                            let active = s.active;
                            let stop = s.stop;
                            let timing = s.timing;

                            if active && !stop {
                                // Recompute per shot so live DPI / sensitivity changes
                                // apply without waiting for the next update() call.
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
                                        x = ((x as f32) * s.move_x_modifier) as i32;
                                    }
                                }

                                (true, active, stop, timing, x, y)
                            } else {
                                (true, active, stop, timing, 0, 0)
                            }
                        }
                    };

                    if !running {
                        break;
                    }

                    if !active || stop {
                        precise_sleep(Duration::from_millis(1));
                        continue;
                    }

                    mouse_input.move_relative(x, y);
                    precise_sleep(Duration::from_secs_f32(timing));
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

    pub fn update(&mut self, x: i32, y: i32, t: i32, x_mod: f32) {
        log_debug(
            &format!("Updating recoil values: X={}, Y={}, Timing={}ms, Xmod={}", x, y, t, x_mod)
        );
        let mut s = self.state.lock().unwrap();
        s.stop = true;
        s.active = false;
        s.move_x = 0;
        s.move_y = 0;
        s.x_flip = 1;
        s.x_once_done = false;
        s.raw_movement_x = x as f32;
        s.raw_movement_y = y as f32;
        s.timing = (t as f32) / 1000.0;
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

        self.active = is_active;
    }

    fn calculate_dpi_adjusted_movement(&self) -> (i32, i32) {
        let base_sensitivity = 30.0;

        if self.dpi == 0 {
            return (self.raw_movement_x as i32, self.raw_movement_y as i32);
        }

        let sens_scale = if self.sensitivity > 0 {
            base_sensitivity / (self.sensitivity as f32)
        } else {
            1.0
        };

        let adjusted_x = self.raw_movement_x * sens_scale;
        let adjusted_y = self.raw_movement_y * sens_scale;

        (adjusted_x.round() as i32, adjusted_y.round() as i32)
    }

}
