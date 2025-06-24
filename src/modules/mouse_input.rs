use crate::modules::input_method_gfck::InputMethodGFCK;
use crate::modules::input_method_ghub::InputMethodGhubMouse;
use std::ffi::c_int;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum InputMethodEnum {
    GFCK,
    GHUB,
}

#[allow(dead_code)]
pub struct MouseInput<'a> {
    gfck: InputMethodGFCK,
    ghub: InputMethodGhubMouse<'a>,
    current: InputMethodEnum,
}

#[allow(dead_code)]
impl<'a> MouseInput<'a> {
    pub unsafe fn new(
        gfck_dll: PathBuf,
        ghub_dll: PathBuf
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if !gfck_dll.exists() {
            return Err(format!("GFCK DLL not found at {:?}", gfck_dll).into());
        }
        if !ghub_dll.exists() {
            return Err(format!("GHUB DLL not found at {:?}", ghub_dll).into());
        }
        unsafe {
            Ok(Self {
                gfck: InputMethodGFCK::new(gfck_dll)?,
                ghub: InputMethodGhubMouse::new(ghub_dll)?,
                current: InputMethodEnum::GFCK, // GFCK is default
            })
        }
    }

    pub fn set_current(&mut self, method_name: &str) {
        self.current = match method_name {
            "GFCK" => InputMethodEnum::GFCK,
            "GhubMouse" => InputMethodEnum::GHUB,
            _ => self.current,
        };
    }

    pub fn get_current_name(&self) -> &str {
        match self.current {
            InputMethodEnum::GFCK => self.gfck.name(),
            InputMethodEnum::GHUB => self.ghub.name(),
        }
    }

    pub fn down(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => self.gfck.down(button),
            InputMethodEnum::GHUB => self.ghub.down(button),
        }
    }

    pub fn up(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => self.gfck.up(button),
            InputMethodEnum::GHUB => self.ghub.up(button),
        }
    }

    pub fn click(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => self.gfck.click(button),
            InputMethodEnum::GHUB => self.ghub.click(button),
        }
    }

    pub fn move_relative(&self, x: c_int, y: c_int) {
        match self.current {
            InputMethodEnum::GFCK => self.gfck.move_relative(x, y),
            InputMethodEnum::GHUB => self.ghub.move_relative(x, y),
        }
    }
}
