use libloading::Library;
use std::{ ffi::c_int, path::PathBuf, thread::sleep, time::Duration };

pub struct InputMethodGFCK {
    _dll: Library, // Keep alive
    mouse_move: unsafe extern "C" fn(c_int, c_int, c_int, c_int),
}

impl InputMethodGFCK {
    pub unsafe fn new(dll_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !dll_path.exists() {
            return Err(format!("GFCK DLL not found at {:?}", dll_path).into());
        }
        unsafe {
            let dll = Library::new(dll_path)?;
            let mouse_move_symbol = dll.get::<unsafe extern "C" fn(c_int, c_int, c_int, c_int)>(
                b"mouse_move"
            )?;
            let mouse_move = *mouse_move_symbol;

            Ok(Self {
                _dll: dll,
                mouse_move,
            })
        }
    }

    pub fn down(&self, button: c_int) {
        unsafe { (self.mouse_move)(button, 0, 0, 0) }
    }

    pub fn up(&self, _button: c_int) {
        unsafe { (self.mouse_move)(0, 0, 0, 0) }
    }

    pub fn click(&self, button: c_int) {
        self.down(button);
        sleep(Duration::from_millis(100));
        self.up(button);
    }

    pub fn move_relative(&self, x: c_int, y: c_int) {
        unsafe { (self.mouse_move)(0, x, y, 0) }
    }

    pub fn name(&self) -> &'static str {
        "GFCK"
    }
}
