use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{ Arc, Mutex };
use winapi::um::winuser::{ GetAsyncKeyState, VK_END, VK_F1, VK_F2 };

pub enum HotkeyCommand {
    Exit,
    ToggleRcs,
    HideToggle,
    SelectWeapon(String),
}

pub struct HotkeyHandler {
    weapon_bindings: Arc<Mutex<HashMap<i32, String>>>,
    exit_key: i32,
    toggle_key: i32,
    hide_key: i32,
    sender: Option<Sender<HotkeyCommand>>,
    prev_states: HashMap<i32, bool>,
}

impl HotkeyHandler {
    pub fn new() -> Self {
        Self {
            weapon_bindings: Arc::new(Mutex::new(HashMap::new())),
            exit_key: VK_END,
            toggle_key: VK_F1,
            hide_key: VK_F2,
            sender: None,
            prev_states: HashMap::new(),
        }
    }

    pub fn set_sender(&mut self, sender: Sender<HotkeyCommand>) {
        self.sender = Some(sender);
    }

    pub fn set_exit_key(&mut self, key_code: i32) {
        self.exit_key = key_code;
    }

    pub fn set_toggle_key(&mut self, key_code: i32) {
        self.toggle_key = key_code;
    }

    pub fn set_hide_key(&mut self, key_code: i32) {
        self.hide_key = key_code;
    }

    pub fn bind_weapon(&mut self, key_code: i32, weapon_name: String) {
        self.weapon_bindings.lock().unwrap().insert(key_code, weapon_name);
    }

    pub fn unbind_weapon(&mut self, key_code: i32) {
        self.weapon_bindings.lock().unwrap().remove(&key_code);
    }

    pub fn check_hotkeys(&mut self) {
        if let Some(sender) = self.sender.clone() {
            let sender_clone = sender.clone();
            self.check_key_press(self.exit_key, move || {
                let _ = sender_clone.send(HotkeyCommand::Exit);
            });

            let sender_clone = sender.clone();
            self.check_key_press(self.toggle_key, move || {
                let _ = sender_clone.send(HotkeyCommand::ToggleRcs);
            });

            let sender_clone = sender.clone();
            self.check_key_press(self.hide_key, move || {
                let _ = sender_clone.send(HotkeyCommand::HideToggle);
            });

            let bindings = self.weapon_bindings.lock().unwrap().clone();
            for (key_code, weapon_name) in bindings {
                let sender_clone = sender.clone();
                self.check_key_press(key_code, move || {
                    let _ = sender_clone.send(HotkeyCommand::SelectWeapon(weapon_name));
                });
            }
        }
    }

    fn check_key_press<F>(&mut self, key_code: i32, callback: F) where F: FnOnce() {
        let is_pressed = unsafe { GetAsyncKeyState(key_code) < 0 };
        let was_pressed = self.prev_states.get(&key_code).copied().unwrap_or(false);

        if is_pressed && !was_pressed {
            callback();
        }

        self.prev_states.insert(key_code, is_pressed);
    }
}

pub fn key_name_to_vk_code(key_name: &str) -> Option<i32> {
    match key_name.to_uppercase().as_str() {
        "NONE" => None,
        "END" => Some(0x23),
        "INSERT" => Some(0x2D),
        "DELETE" => Some(0x2E),
        "HOME" => Some(0x24),
        "PAGEUP" => Some(0x21),
        "PAGEDOWN" => Some(0x22),
        "F1" => Some(0x70),
        "F2" => Some(0x71),
        "F3" => Some(0x72),
        "F4" => Some(0x73),
        "F5" => Some(0x74),
        "F6" => Some(0x75),
        "F7" => Some(0x76),
        "F8" => Some(0x77),
        "F9" => Some(0x78),
        "F10" => Some(0x79),
        "F11" => Some(0x7A),
        "F12" => Some(0x7B),
        "A" => Some(0x41),
        "B" => Some(0x42),
        "C" => Some(0x43),
        "D" => Some(0x44),
        "E" => Some(0x45),
        "F" => Some(0x46),
        "G" => Some(0x47),
        "H" => Some(0x48),
        "I" => Some(0x49),
        "J" => Some(0x4A),
        "K" => Some(0x4B),
        "L" => Some(0x4C),
        "M" => Some(0x4D),
        "N" => Some(0x4E),
        "O" => Some(0x4F),
        "P" => Some(0x50),
        "Q" => Some(0x51),
        "R" => Some(0x52),
        "S" => Some(0x53),
        "T" => Some(0x54),
        "U" => Some(0x55),
        "V" => Some(0x56),
        "W" => Some(0x57),
        "X" => Some(0x58),
        "Y" => Some(0x59),
        "Z" => Some(0x5A),
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),
        _ => None,
    }
}
