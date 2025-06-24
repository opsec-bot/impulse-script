use libloading::{Library, Symbol};
use std::{ffi::c_int, path::PathBuf, thread::sleep, time::Duration};

pub struct InputMethodGhubMouse<'a> {
    _dll: Library, // Keep alive
    press: Symbol<'a, unsafe extern "C" fn(c_int) -> c_int>,
    release: Symbol<'a, unsafe extern "C" fn() -> c_int>,
    move_r: Symbol<'a, unsafe extern "C" fn(c_int, c_int) -> c_int>,
    #[allow(dead_code)]
    mouse_close: Symbol<'a, unsafe extern "C" fn() -> c_int>,
}

impl<'a> InputMethodGhubMouse<'a> {
    pub unsafe fn new(dll_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !dll_path.exists() {
            return Err(format!("GHUB DLL not found at {:?}", dll_path).into());
        }
        unsafe {
            let dll = Library::new(dll_path)?;
            // Use transmute to extend the lifetime of the symbols to match the struct's lifetime
            use std::mem::transmute;
            let press = dll.get::<unsafe extern "C" fn(c_int) -> c_int>(b"press")?;
            let release = dll.get::<unsafe extern "C" fn() -> c_int>(b"release")?;
            let move_r = dll.get::<unsafe extern "C" fn(c_int, c_int) -> c_int>(b"moveR")?;
            let mouse_open = dll.get::<unsafe extern "C" fn() -> c_int>(b"mouse_open")?;
            let mouse_close = dll.get::<unsafe extern "C" fn() -> c_int>(b"mouse_close")?;

            // SAFETY: The symbols do not outlive the library, which is stored in the struct.
            let press = transmute::<_, Symbol<'a, unsafe extern "C" fn(c_int) -> c_int>>(press);
            let release = transmute::<_, Symbol<'a, unsafe extern "C" fn() -> c_int>>(release);
            let move_r = transmute::<_, Symbol<'a, unsafe extern "C" fn(c_int, c_int) -> c_int>>(move_r);
            let mouse_open = transmute::<_, Symbol<'a, unsafe extern "C" fn() -> c_int>>(mouse_open);
            let mouse_close = transmute::<_, Symbol<'a, unsafe extern "C" fn() -> c_int>>(mouse_close);

            if (mouse_open)() != 1 {
                eprintln!("Failed to open GHUB mouse interface");
            }

            Ok(Self {
                _dll: dll,
                press,
                release,
                move_r,
                mouse_close,
            })
        }
    }

    pub fn down(&self, button: c_int) {
        unsafe { (self.press)(button) };
    }

    pub fn up(&self, _button: c_int) {
        unsafe { (self.release)() };
    }

    pub fn click(&self, button: c_int) {
        self.down(button);
        sleep(Duration::from_millis(100));
        self.up(button);
    }

    pub fn move_relative(&self, x: c_int, y: c_int) {
        unsafe { (self.move_r)(x, y) };
    }

    pub fn name(&self) -> &'static str {
        "GhubMouse"
    }
}