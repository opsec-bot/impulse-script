use crate::modules::input::gfck::InputMethodGFCK;
use crate::modules::input::ghub::InputMethodGhubMouse;
use std::ffi::c_int;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub enum InputMethodEnum {
    GFCK,
    GHUB,
}

#[allow(dead_code)]
pub struct MouseInput {
    gfck: InputMethodGFCK,
    ghub: InputMethodGhubMouse,
    current: InputMethodEnum,
}

impl<'a> MouseInput {
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
        Ok(Self {
            gfck: unsafe {
                InputMethodGFCK::new(gfck_dll)?
            },
            ghub: unsafe {
                InputMethodGhubMouse::new(ghub_dll)?
            },
            current: InputMethodEnum::GFCK,
        })
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
        println!(
            "[MouseInput] move_relative called with x: {}, y: {}, method: {}",
            x,
            y,
            self.get_current_name()
        );
        match self.current {
            InputMethodEnum::GFCK => self.gfck.move_relative(x, y),
            InputMethodEnum::GHUB => self.ghub.move_relative(x, y),
        }
    }
}