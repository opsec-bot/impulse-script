use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };
use std::time::Duration;

use crate::modules::input::raw_mouse;
use crate::modules::core::logger::log_debug;

#[cfg(windows)]
use winapi::um::winuser::GetAsyncKeyState;

#[derive(Clone)]
pub struct TraceSnapshot {
    /// Number of holes marked in the current (in-progress) trace.
    pub current_marks: usize,
    /// Per-shot recoil deltas for the current trace: `hole[i] - hole[i-1]`.
    /// One entry per mark *after* the first (the first mark is the origin).
    pub current: Vec<(i32, i32)>,
    /// Completed traces, each a list of per-shot deltas.
    pub samples: Vec<Vec<(i32, i32)>>,
}

struct TraceState {
    running: bool,
    active: bool,
    mark_vk: i32,
    // Cursor position accumulated from raw mouse deltas, relative to hole 1.
    cursor_x: i32,
    cursor_y: i32,
    // Position of the previously marked hole, relative to hole 1.
    last_mark_x: i32,
    last_mark_y: i32,
    // How many holes have been marked in the current trace (includes hole 1).
    current_marks: usize,
    // Per-shot recoil deltas (one entry per mark after the first).
    current: Vec<(i32, i32)>,
    samples: Vec<Vec<(i32, i32)>>,
    prev_mark: bool,
}

pub struct TraceRecorder {
    thread: Option<JoinHandle<()>>,
    state: Arc<Mutex<TraceState>>,
}

impl TraceRecorder {
    pub fn new(mark_vk: i32) -> Self {
        log_debug("Starting TraceRecorder");
        raw_mouse::init();

        let state = Arc::new(
            Mutex::new(TraceState {
                running: true,
                active: false,
                mark_vk,
                cursor_x: 0,
                cursor_y: 0,
                last_mark_x: 0,
                last_mark_y: 0,
                current_marks: 0,
                current: Vec::new(),
                samples: Vec::new(),
                prev_mark: false,
            })
        );

        let thread_state = Arc::clone(&state);
        let handle = thread::spawn(move || {
            loop {
                // Always drain the raw-input accumulator so it never grows
                // unbounded; only fold it into the cursor while tracing.
                let (dx, dy) = raw_mouse::take_delta();

                {
                    let mut s = thread_state.lock().unwrap();
                    if !s.running {
                        break;
                    }

                    if s.active {
                        s.cursor_x += dx;
                        s.cursor_y += dy;
                    }

                    #[cfg(windows)]
                    let mark_down = unsafe { GetAsyncKeyState(s.mark_vk) < 0 };
                    #[cfg(not(windows))]
                    let mark_down = false;

                    let mark_just_pressed = mark_down && !s.prev_mark;
                    s.prev_mark = mark_down;

                    if s.active && mark_just_pressed {
                        if s.current_marks == 0 {
                            // First mark = hole 1 = origin. Establish the
                            // reference point; this is NOT a recoil step.
                            s.cursor_x = 0;
                            s.cursor_y = 0;
                            s.last_mark_x = 0;
                            s.last_mark_y = 0;
                        } else {
                            // Subsequent hole: the per-shot recoil vector is the
                            // movement since the previous hole was marked.
                            let mark_dx = s.cursor_x - s.last_mark_x;
                            let mark_dy = s.cursor_y - s.last_mark_y;
                            s.current.push((mark_dx, mark_dy));
                            s.last_mark_x = s.cursor_x;
                            s.last_mark_y = s.cursor_y;
                        }
                        s.current_marks += 1;
                    }
                }

                thread::sleep(Duration::from_millis(2));
            }
        });

        Self { thread: Some(handle), state }
    }

    pub fn start_trace(&self) {
        let mut s = self.state.lock().unwrap();
        s.active = true;
        s.cursor_x = 0;
        s.cursor_y = 0;
        s.last_mark_x = 0;
        s.last_mark_y = 0;
        s.current_marks = 0;
        s.current.clear();
        s.prev_mark = false;
        // Discard any movement accumulated before the trace started.
        let _ = raw_mouse::take_delta();
    }

    pub fn finish_sample(&self) {
        let mut s = self.state.lock().unwrap();
        // Only keep traces that actually captured recoil steps (>= 2 holes).
        if !s.current.is_empty() {
            let copy = s.current.clone();
            s.samples.push(copy);
        }
        s.active = false;
        s.current.clear();
        s.cursor_x = 0;
        s.cursor_y = 0;
        s.last_mark_x = 0;
        s.last_mark_y = 0;
        s.current_marks = 0;
        s.prev_mark = false;
    }

    pub fn set_mark_key(&self, mark_vk: i32) {
        let mut s = self.state.lock().unwrap();
        s.mark_vk = mark_vk;
        s.prev_mark = false;
    }

    pub fn clear(&self) {
        let mut s = self.state.lock().unwrap();
        s.active = false;
        s.samples.clear();
        s.current.clear();
        s.cursor_x = 0;
        s.cursor_y = 0;
        s.last_mark_x = 0;
        s.last_mark_y = 0;
        s.current_marks = 0;
        s.prev_mark = false;
    }

    pub fn snapshot(&self) -> TraceSnapshot {
        let s = self.state.lock().unwrap();
        TraceSnapshot {
            current_marks: s.current_marks,
            current: s.current.clone(),
            samples: s.samples.clone(),
        }
    }

    pub fn stop(&mut self) {
        {
            let mut s = self.state.lock().unwrap();
            s.running = false;
        }
        if let Some(h) = self.thread.take() {
            let _ = h.join();
        }
    }
}

impl Drop for TraceRecorder {
    fn drop(&mut self) {
        self.stop();
    }
}
