//! Serial-based mouse input via JSON protocol (e.g., ESP32 + Pico hardware).
//!
//! This module provides a reusable interface to control a mouse connected via
//! a serial port using a JSON-based command protocol. It's designed to work with
//! microcontroller setups like:
//! - ESP32-S3 (USB host) + Raspberry Pi Pico (USB device)
//! - Any device that accepts JSON mouse commands over serial
//!
//! # Protocol
//!
//! Commands are JSON objects (one per line, `\n`-terminated):
//! - `{"cmd": "move", "dx": i32, "dy": i32}` - Relative movement
//! - `{"cmd": "down", "btn": u8}` - Button press (1=left, 2=right, 4=middle)
//! - `{"cmd": "up"}` - Button release
//! - `{"cmd": "click", "btn": u8}` - Full click (down + up)
//!
//! # Example
//!
//! ```ignore
//! let mouse = InputMethodSerialMouse::new("COM3", 115200)?;
//! mouse.move_relative(10, -5);  // Move right 10, up 5
//! mouse.down(1);                 // Left mouse button down
//! mouse.up(1);                   // Left mouse button up
//! ```

use serde_json::json;
use std::ffi::c_int;
use std::io::Write;
use std::sync::Mutex;
use std::time::Duration;

use crate::modules::core::logger::{ log_debug, log_error, log_warning };

/// Button codes compatible with common HID standards.
const BUTTON_LEFT: u8 = 1;
const BUTTON_RIGHT: u8 = 2;
const BUTTON_MIDDLE: u8 = 4;

/// Maps c_int button codes (Windows VK_* style) to HID button codes.
fn map_button_code(button: c_int) -> u8 {
    match button {
        1 => BUTTON_LEFT,        // VK_LBUTTON or generic left
        2 => BUTTON_RIGHT,       // VK_RBUTTON or generic right
        4 => BUTTON_MIDDLE,      // VK_MBUTTON or generic middle
        _ => BUTTON_LEFT,        // Default to left
    }
}

pub struct InputMethodSerialMouse {
    port_name: String,
    baud_rate: u32,
    port: Mutex<Option<Box<dyn serialport::SerialPort>>>,
}

impl InputMethodSerialMouse {
    /// Create a new SerialMouse instance (disconnected state).
    ///
    /// # Arguments
    /// * `port_name` - Serial port name (e.g., "COM3", "/dev/ttyUSB0")
    /// * `baud_rate` - Baud rate (typically 115200 for Pico CDC)
    ///
    /// # Note
    /// This creates the instance but does not open the port. Call `connect()` to establish connection.
    pub fn new(port_name: &str, baud_rate: u32) -> Self {
        log_debug(&format!("Creating SerialMouse instance for {} @ {}", port_name, baud_rate));
        Self {
            port_name: port_name.to_string(),
            baud_rate,
            port: Mutex::new(None),
        }
    }

    /// Connect to the serial port.
    ///
    /// # Errors
    /// Returns an error if the port cannot be opened (already in use, doesn't exist, etc.)
    pub fn connect(&self) -> Result<(), String> {
        log_debug(&format!("Connecting to {} @ {}", self.port_name, self.baud_rate));

        let port = serialport::new(&self.port_name, self.baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| format!("Failed to open serial port: {}", e))?;

        match self.port.lock() {
            Ok(mut p) => {
                *p = Some(port);
                log_debug("SerialMouse connected successfully");
                Ok(())
            }
            Err(_) => {
                log_error("Failed to lock port mutex during connect");
                Err("Failed to lock port mutex".to_string())
            }
        }
    }

    /// Disconnect from the serial port.
    pub fn disconnect(&self) {
        match self.port.lock() {
            Ok(mut p) => {
                if p.is_some() {
                    *p = None;
                    log_debug("SerialMouse disconnected");
                }
            }
            Err(_) => {
                log_warning("Failed to lock port mutex during disconnect");
            }
        }
    }

    /// Check if the serial port is currently connected.
    pub fn is_connected(&self) -> bool {
        self.port
            .lock()
            .map(|p| p.is_some())
            .unwrap_or(false)
    }

    /// Send a command as a JSON line.
    fn send_command(&self, cmd: &serde_json::Value) -> Result<(), String> {
        let json_line = format!("{}\n", cmd.to_string());
        
        match self.port.lock() {
            Ok(mut port_guard) => {
                match port_guard.as_mut() {
                    Some(port) => {
                        port.write_all(json_line.as_bytes())
                            .map_err(|e| format!("serial write failed: {}", e))?;
                        port.flush()
                            .map_err(|e| format!("serial flush failed: {}", e))?;
                        log_debug(&format!("SerialMouse sent: {}", cmd.to_string()));
                        Ok(())
                    }
                    None => Err("SerialMouse not connected".to_string()),
                }
            }
            Err(_) => Err("failed to lock serial port".to_string()),
        }
    }

    pub fn down(&self, button: c_int) {
        let btn = map_button_code(button);
        let cmd = json!({ "cmd": "down", "btn": btn });
        
        if let Err(e) = self.send_command(&cmd) {
            log_warning(&format!("SerialMouse down failed: {}", e));
        }
    }

    pub fn up(&self, _button: c_int) {
        let cmd = json!({ "cmd": "up" });
        
        if let Err(e) = self.send_command(&cmd) {
            log_warning(&format!("SerialMouse up failed: {}", e));
        }
    }

    pub fn click(&self, button: c_int) {
        let btn = map_button_code(button);
        let cmd = json!({ "cmd": "click", "btn": btn });
        
        if let Err(e) = self.send_command(&cmd) {
            log_warning(&format!("SerialMouse click failed: {}", e));
        }
    }

    pub fn move_relative(&self, x: c_int, y: c_int) {
        let cmd = json!({ "cmd": "move", "dx": x, "dy": y });
        
        if let Err(e) = self.send_command(&cmd) {
            log_warning(&format!("SerialMouse move failed: {}", e));
        }
    }

    pub fn name(&self) -> &'static str {
        "SerialMouse (Hardware)"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_mapping() {
        assert_eq!(map_button_code(1), BUTTON_LEFT);
        assert_eq!(map_button_code(2), BUTTON_RIGHT);
        assert_eq!(map_button_code(4), BUTTON_MIDDLE);
        assert_eq!(map_button_code(99), BUTTON_LEFT); // Unknown defaults to left
    }

    #[test]
    fn test_json_command_format() {
        let move_cmd = json!({ "cmd": "move", "dx": 10, "dy": -5 });
        assert_eq!(move_cmd["cmd"], "move");
        assert_eq!(move_cmd["dx"], 10);

        let click_cmd = json!({ "cmd": "click", "btn": 1 });
        assert_eq!(click_cmd["cmd"], "click");
        assert_eq!(click_cmd["btn"], 1);
    }
}
