use libloading::Library;
use std::{ ffi::c_int, path::PathBuf, thread::sleep, time::Duration };

pub struct InputMethodGhubMouse {
    _dll: Library, // Keep alive
    press: unsafe extern "C" fn(c_int) -> c_int,
    release: unsafe extern "C" fn() -> c_int,
    move_r: unsafe extern "C" fn(c_int, c_int) -> c_int,
}

impl InputMethodGhubMouse {
    pub unsafe fn new(dll_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !dll_path.exists() {
            return Err(format!("GHUB DLL not found at {:?}", dll_path).into());
        }
        unsafe {
            let dll = Library::new(dll_path)?;

            let press_symbol = dll.get::<unsafe extern "C" fn(c_int) -> c_int>(b"press")?;
            let release_symbol = dll.get::<unsafe extern "C" fn() -> c_int>(b"release")?;
            let move_r_symbol = dll.get::<unsafe extern "C" fn(c_int, c_int) -> c_int>(b"moveR")?;
            let mouse_open = dll.get::<unsafe extern "C" fn() -> c_int>(b"mouse_open")?;

            if mouse_open() != 1 {
                eprintln!("Failed to open GHUB mouse interface");
            }

            let press_fn = *press_symbol;
            let release_fn = *release_symbol;
            let move_r_fn = *move_r_symbol;

            Ok(Self {
                _dll: dll,
                press: press_fn,
                release: release_fn,
                move_r: move_r_fn,
            })
        }
    }

    pub fn down(&self, button: c_int) {
        unsafe { (self.press)(button); }
    }

    pub fn up(&self, _button: c_int) {
        unsafe { (self.release)(); }
    }

    pub fn click(&self, button: c_int) {
        self.down(button);
        sleep(Duration::from_millis(100));
        self.up(button);
    }

    pub fn move_relative(&self, x: c_int, y: c_int) {
        unsafe { (self.move_r)(x, y); }
    }

    pub fn name(&self) -> &'static str {
        "GhubMouse"
    }
}
