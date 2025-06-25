use std::sync::{ Arc, Mutex, mpsc::Sender };
use std::thread::{ self, JoinHandle };
use std::time::Duration;

use winapi::um::winuser::{ GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON };

use crate::modules::mouse_command::MouseCommand;
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
                })
            ),
            sender: None,
        }
    }

    pub fn set_sender(&mut self, sender: Sender<MouseCommand>) {
        self.sender = Some(sender);
    }

    #[allow(unused_variables)]
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
                                let x = s.move_x;
                                let y = s.move_y;
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
        s.move_x = 0;
        s.move_y = 0;
        s.timing = 0.0;
        s.move_x_modifier = 1.0;
    }

    /// Call this when a weapon/hotkey is selected to set the current recoil profile.
    pub fn update(&mut self, x: i32, y: i32, t: i32, x_mod: f32) {
        self.reset();
        let mut s = self.state.lock().unwrap();
        // X = Horizontal Amount (>0 right, <0 left)
        // Y = Vertical Amount (use calculator)
        // Xmod = Modifier applied to X every iteration:
        //   -1: Flips X direction each iteration
        //    0: Moves horizontal once, then stops
        //    1: No modification (X stays the same)
        s.move_x = x;
        s.move_y = y;
        s.timing = (t as f32) / 1000.0;
        s.move_x_modifier = x_mod;
        s.x_flip = 1;
        s.x_once_done = false;
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
        let is_active = unsafe {
            GetAsyncKeyState(VK_RBUTTON) < 0 && GetAsyncKeyState(VK_LBUTTON) < 0
        };
        self.active = is_active;
    }

    fn current(&self, debug: bool) -> (i32, i32, f32, f32) {
        if debug {
            println!(
                "current values: ({}, {}, {}, {})",
                self.move_x,
                self.move_y,
                self.timing,
                self.move_x_modifier
            );
        }
        (self.move_x, self.move_y, self.timing, self.move_x_modifier)
    }
}
