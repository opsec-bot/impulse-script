use std::sync::{ Arc, Mutex, mpsc::Sender };
use std::thread::{ self, JoinHandle };
use std::time::Duration;

#[cfg(windows)]
use winapi::um::winuser::{ GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON };

use crate::modules::input::MouseCommand;
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
    sender: Option<Sender<MouseCommand>>,
}

impl Control {
    pub fn new() -> Self {
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
            sender: None,
        }
    }

    pub fn set_sender(&mut self, sender: Sender<MouseCommand>) {
        self.sender = Some(sender);
    }

    pub fn run_threaded(&mut self) {
        let state = Arc::clone(&self.state);
        let sender = self.sender.clone();
        {
            let mut s = state.lock().unwrap();
            s.running = true;
        }
        self.thread = Some(
            thread::spawn(move || {
                while state.lock().unwrap().running {
                    {
                        let mut s = state.lock().unwrap();
                        s.check_status();
                        if !s.active {
                            drop(s);
                            thread::sleep(Duration::from_millis(50));
                            continue;
                        }
                        if !s.stop {
                            if let Some(ref sender) = sender {
                                let (x, y) = s.calculate_dpi_adjusted_movement();
                                sender.send(MouseCommand::Move(x, y)).ok();
                            }
                            std::thread::sleep(Duration::from_secs_f32(s.timing));
                        }
                    }
                }
            })
        );
    }

    pub fn reset(&mut self) {
        let mut s = self.state.lock().unwrap();
        s.stop = true;
        s.active = false;
        s.move_x = 0;
        s.move_y = 0;
        s.timing = 0.0;
        s.move_x_modifier = 1.0;
        s.raw_movement_x = 0.0;
        s.raw_movement_y = 0.0;
    }

    pub fn set_dpi(&mut self, dpi: i32) {
        let mut state = self.state.lock().unwrap();
        state.dpi = dpi;
    }

    pub fn set_sensitivity(&mut self, sensitivity: i32) {
        let mut state = self.state.lock().unwrap();
        state.sensitivity = sensitivity;
    }

    pub fn update(&mut self, x: i32, y: i32, t: i32, x_mod: f32) {
        self.reset();
        let mut s = self.state.lock().unwrap();
        s.raw_movement_x = x as f32;
        s.raw_movement_y = y as f32;
        s.timing = (t as f32) / 1000.0;
        s.move_x_modifier = x_mod;
        s.x_flip = 1;
        s.x_once_done = false;

        let (adjusted_x, adjusted_y) = s.calculate_dpi_adjusted_movement();
        s.move_x = adjusted_x;
        s.move_y = adjusted_y;

        s.current(true);
        s.stop = false;
    }

    pub fn current(&self, debug: bool) -> (i32, i32, f32, f32) {
        let s = self.state.lock().unwrap();
        s.current(debug)
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
        if self.sensitivity == 0 || self.dpi == 0 {
            return (self.raw_movement_x as i32, self.raw_movement_y as i32);
        }

        let dpi_scale = 800.0 / (self.dpi as f32);
        let sens_scale = 30.0 / (self.sensitivity as f32);

        let adjusted_x = self.raw_movement_x * dpi_scale * sens_scale;
        let adjusted_y = self.raw_movement_y * dpi_scale * sens_scale;

        (adjusted_x.round() as i32, adjusted_y.round() as i32)
    }

    fn current(&self, debug: bool) -> (i32, i32, f32, f32) {
        if debug {
            println!(
                "current values: ({}, {}, {:.5}, {})",
                self.move_x,
                self.move_y,
                self.timing,
                self.move_x_modifier
            );
        }
        (self.move_x, self.move_y, self.timing, self.move_x_modifier)
    }
}
