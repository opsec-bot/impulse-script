use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::path::PathBuf;

use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON, VK_RBUTTON};

use crate::modules::mouse_input::MouseInput;

pub struct Control {
    name: &'static str,
    thread: Option<JoinHandle<()>>,
    stop: Arc<Mutex<bool>>,
    running: Arc<Mutex<bool>>,
    active: Arc<Mutex<bool>>,
    move_x: Arc<Mutex<i32>>,
    move_y: Arc<Mutex<i32>>,
    move_x_modifier: Arc<Mutex<f32>>,
    timing: Arc<Mutex<f32>>,
    threaded: bool,
    mouse_input: MouseInput<'static>,
}

impl Control {
    pub fn new() -> Self {
        let gfck_path = PathBuf::from("lib/GFCK.dll");
        let ghub_path = PathBuf::from("lib/ghub_mouse.dll");
        let mouse_input = unsafe {
            MouseInput::new(gfck_path, ghub_path).expect("Failed to load mouse input DLLs")
        };
        Control {
            name: "Control",
            thread: None,
            stop: Arc::new(Mutex::new(false)),
            running: Arc::new(Mutex::new(false)),
            active: Arc::new(Mutex::new(false)),
            move_x: Arc::new(Mutex::new(0)),
            move_y: Arc::new(Mutex::new(0)),
            move_x_modifier: Arc::new(Mutex::new(1.0)),
            timing: Arc::new(Mutex::new(0.0)),
            threaded: false,
            mouse_input,
        }
    }

    pub fn run_threaded(&mut self) {
        self.threaded = true;
        *self.running.lock().unwrap() = true;
        let running = Arc::clone(&self.running);
        let active = Arc::clone(&self.active);
        let stop = Arc::clone(&self.stop);
        let move_x = Arc::clone(&self.move_x);
        let move_y = Arc::clone(&self.move_y);
        let move_x_modifier = Arc::clone(&self.move_x_modifier);
        let timing = Arc::clone(&self.timing);
        let mut mouse_input = self.mouse_input.clone();

        self.thread = Some(thread::spawn(move || {
            while *running.lock().unwrap() {
                if Self::check_status() {
                    *active.lock().unwrap() = true;
                } else {
                    *active.lock().unwrap() = false;
                    thread::sleep(Duration::from_millis(50));
                    continue;
                }

                if !*stop.lock().unwrap() {
                    let x = *move_x.lock().unwrap();
                    let y = *move_y.lock().unwrap();
                    let t = *timing.lock().unwrap();
                    let x_mod = *move_x_modifier.lock().unwrap();

                    mouse_input.move_relative(x, y);
                    thread::sleep(Duration::from_secs_f32(t));
                    *move_x.lock().unwrap() = (x as f32 * x_mod) as i32;
                }
            }
        }));
    }

    pub fn run(&mut self) {
        *self.running.lock().unwrap() = true;
        while *self.running.lock().unwrap() {
            if Self::check_status() {
                *self.active.lock().unwrap() = true;
                self.movement();
            } else {
                *self.active.lock().unwrap() = false;
            }
        }
    }

    fn check_status() -> bool {
        unsafe {
            GetAsyncKeyState(VK_RBUTTON) < 0 && GetAsyncKeyState(VK_LBUTTON) < 0
        }
    }

    pub fn cleanup(&mut self) {
        *self.running.lock().unwrap() = false;
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    pub fn reset(&mut self) {
        *self.stop.lock().unwrap() = true;
        *self.move_x.lock().unwrap() = 0;
        *self.move_y.lock().unwrap() = 0;
        *self.timing.lock().unwrap() = 0.0;
        *self.move_x_modifier.lock().unwrap() = 1.0;
    }

    pub fn update(&mut self, x: i32, y: i32, t: i32, x_mod: f32) {
        self.reset();
        *self.move_x.lock().unwrap() = x;
        *self.move_y.lock().unwrap() = y;
        *self.timing.lock().unwrap() = (t as f32) / 1000.0;
        *self.move_x_modifier.lock().unwrap() = x_mod;
        self.current(true);
        *self.stop.lock().unwrap() = false;
    }

    pub fn current(&self, debug: bool) -> (i32, i32, f32, f32) {
        let x = *self.move_x.lock().unwrap();
        let y = *self.move_y.lock().unwrap();
        let t = *self.timing.lock().unwrap();
        let x_mod = *self.move_x_modifier.lock().unwrap();
        if debug {
            println!("current values: ({}, {}, {}, {})", x, y, t, x_mod);
        }
        (x, y, t, x_mod)
    }

    pub fn movement(&mut self) {
        if !*self.stop.lock().unwrap() {
            let x = *self.move_x.lock().unwrap();
            let y = *self.move_y.lock().unwrap();
            let t = *self.timing.lock().unwrap();
            let x_mod = *self.move_x_modifier.lock().unwrap();
            self.mouse_input.move_relative(x, y);
            thread::sleep(Duration::from_secs_f32(t));
            *self.move_x.lock().unwrap() = (x as f32 * x_mod) as i32;
        }
    }
}
