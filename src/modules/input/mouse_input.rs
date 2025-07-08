use crate::modules::core::logger::{ log_debug, log_error, log_warning };
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
    gfck: Option<InputMethodGFCK>,
    ghub: Option<InputMethodGhubMouse>,
    current: InputMethodEnum,
}

impl<'a> MouseInput {
    pub unsafe fn new(
        gfck_dll: PathBuf,
        ghub_dll: PathBuf
    ) -> Result<Self, Box<dyn std::error::Error>> {
        log_debug("Initializing mouse input systems");

        let gfck_result = unsafe { InputMethodGFCK::new(gfck_dll.clone()) };
        let ghub_result = unsafe { InputMethodGhubMouse::new(ghub_dll.clone()) };

        match (&gfck_result, &ghub_result) {
            (Ok(_), Ok(_)) => {
                log_debug("Both GFCK and GHub mouse systems loaded successfully");
            }
            (Ok(_), Err(e)) => log_warning(&format!("GFCK loaded, GHub failed: {}", e)),
            (Err(e), Ok(_)) => log_warning(&format!("GHub loaded, GFCK failed: {}", e)),
            (Err(e1), Err(e2)) => {
                log_error(&format!("Both mouse systems failed - GFCK: {}, GHub: {}", e1, e2));
                return Err("No mouse input methods available".into());
            }
        }

        Ok(Self {
            gfck: gfck_result.ok(),
            ghub: ghub_result.ok(),
            current: InputMethodEnum::GFCK,
        })
    }

    pub fn set_current(&mut self, method_name: &str) {
        log_debug(&format!("Switching mouse input method to: {}", method_name));
        match method_name {
            "GFCK" => {
                if self.gfck.is_some() {
                    self.current = InputMethodEnum::GFCK;
                    log_debug("Successfully switched to GFCK");
                } else {
                    log_error("Cannot switch to GFCK - not available");
                }
            }
            "GhubMouse" => {
                if self.ghub.is_some() {
                    self.current = InputMethodEnum::GHUB;
                    log_debug("Successfully switched to GhubMouse");
                } else {
                    log_error("Cannot switch to GhubMouse - not available");
                }
            }
            _ => {
                log_warning(&format!("Unknown mouse input method: {}", method_name));
            }
        }
    }

    pub fn get_current_name(&self) -> &str {
        match self.current {
            InputMethodEnum::GFCK =>
                self.gfck
                    .as_ref()
                    .map(|g| g.name())
                    .unwrap_or("Unavailable"),
            InputMethodEnum::GHUB =>
                self.ghub
                    .as_ref()
                    .map(|g| g.name())
                    .unwrap_or("Unavailable"),
        }
    }

    pub fn down(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => {
                if let Some(gfck) = &self.gfck {
                    gfck.down(button)
                }
            }
            InputMethodEnum::GHUB => {
                if let Some(ghub) = &self.ghub {
                    ghub.down(button)
                }
            }
        }
    }

    pub fn up(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => {
                if let Some(gfck) = &self.gfck {
                    gfck.up(button)
                }
            }
            InputMethodEnum::GHUB => {
                if let Some(ghub) = &self.ghub {
                    ghub.up(button)
                }
            }
        }
    }

    pub fn click(&self, button: c_int) {
        match self.current {
            InputMethodEnum::GFCK => {
                if let Some(gfck) = &self.gfck {
                    gfck.click(button)
                }
            }
            InputMethodEnum::GHUB => {
                if let Some(ghub) = &self.ghub {
                    ghub.click(button)
                }
            }
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
            InputMethodEnum::GFCK => {
                if let Some(gfck) = &self.gfck {
                    gfck.move_relative(x, y)
                }
            }
            InputMethodEnum::GHUB => {
                if let Some(ghub) = &self.ghub {
                    ghub.move_relative(x, y)
                }
            }
        }
    }
}
