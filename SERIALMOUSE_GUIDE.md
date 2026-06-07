# SerialMouse Backend Integration Guide

## Overview

The **SerialMouse** backend is a new reusable mouse input method for rust-macro that connects to hardware-based mouse controllers via serial/CDC connections. It was designed specifically for your ESP32+Pico setup but can work with any device that accepts JSON-based mouse commands over serial.

## Features

- ✅ Full mouse movement support via relative positioning
- ✅ Button control (down, up, click)
- ✅ Cross-platform serial communication (Windows, macOS, Linux)
- ✅ JSON-based command protocol (human-readable, easy to debug)
- ✅ Non-blocking error handling with logging
- ✅ Reusable across multiple projects

## Using SerialMouse in rust-macro

### 1. Basic Setup

The new `MouseInput::new_with_serial()` method initializes and connects the SerialMouse backend:

```rust
use crate::modules::input::MouseInput;

// Initialize and connect
let mouse = MouseInput::new_with_serial("COM3", 115200)?;

// Or switch to SerialMouse after initialization
mouse.set_current("SerialMouse");
```

### 2. Connection Management

```rust
// Check if connected
if mouse.is_serial_connected() {
    println!("SerialMouse is connected");
}

// Disconnect gracefully
mouse.disconnect_serial();

// Reconnect
mouse.connect_serial()?;
```

In your `config.ini` or settings system, add:

```ini
[input]
method = SerialMouse
serial_port = COM3
baud_rate = 115200
```

Then load it with:

```rust
let port = config.get("input", "serial_port").unwrap_or("COM3");
let baud = config.get("input", "baud_rate").unwrap_or("115200").parse().unwrap_or(115200u32);
let mouse = MouseInput::new_with_serial(&port, baud)?;
```

### 3. Using the mouse backend

Once initialized, use it like any other backend:

```rust
// Move mouse relative
mouse.move_relative(10, -5);  // Right 10, up 5

// Button control
mouse.down(1);      // Press left button
mouse.up(1);        // Release left button
mouse.click(1);     // Full click

// Switch methods at runtime
mouse.set_current("SerialMouse");
mouse.set_current("GFCK");
mouse.set_current("GhubMouse");

// Connection lifecycle (SerialMouse only)
mouse.is_serial_connected();    // Check connection status
mouse.disconnect_serial();      // Close port gracefully
mouse.connect_serial()?;        // Reopen port (e.g., after hotplug)
```

## Connection Lifecycle API

The SerialMouse backend supports explicit connection management:

### `connect(&self) -> Result<(), String>`
Opens the serial port. Called automatically by `new_with_serial()` but can be called again after disconnection or hotplug events.

```rust
match mouse.connect_serial() {
    Ok(_) => println!("Connected!"),
    Err(e) => eprintln!("Failed: {}", e),
}
```

### `disconnect(&self)`
Closes the serial port gracefully. Port will not accept commands until `connect()` is called again.

```rust
mouse.disconnect_serial();  // Always succeeds
```

### `is_connected(&self) -> bool`
Checks current connection state without attempting to reconnect.

```rust
if mouse.is_serial_connected() {
    mouse.move_relative(10, 5);  // Safe to use
} else {
    mouse.connect_serial()?;     // Reconnect if needed
}
```

## Serial Protocol Specification

The SerialMouse backend sends JSON commands, one per line, terminated with `\n`:

### Command Format

```json
{"cmd": "move", "dx": 10, "dy": -5}
{"cmd": "down", "btn": 1}
{"cmd": "up"}
{"cmd": "click", "btn": 1}
```

### Button Codes

- `1` = Left mouse button
- `2` = Right mouse button  
- `4` = Middle mouse button

### Examples

```json
{"cmd": "move", "dx": 0, "dy": 0}           // No movement
{"cmd": "move", "dx": 100, "dy": -50}       // Right 100px, up 50px
{"cmd": "down", "btn": 1}                   // Left button press
{"cmd": "up"}                               // Any button release
{"cmd": "click", "btn": 2}                  // Right-click
```

## Reusing in Other Projects

### Option 1: Copy the backend file

The SerialMouse backend is designed to be **project-independent**. To reuse it:

1. Copy `src/modules/input/serial_mouse.rs` to your project
2. Add `serialport` and `serde_json` to your `Cargo.toml`
3. Adapt the logger imports if your logging system differs

### Option 2: Extract as a standalone crate

To make it a publishable crate:

```bash
cargo new serial-mouse --lib
# Copy serial_mouse.rs as src/lib.rs
# Update logger imports to use your logging system
# Publish to crates.io
```

### Option 3: Minimal integration example

If you don't need rust-macro's full infrastructure:

```rust
use serde_json::json;
use std::io::Write;

// Create instance (disconnected)
let mouse = InputMethodSerialMouse::new("COM3", 115200);

// Connect when ready
mouse.connect()?;

// Use it
mouse.move_relative(10, 5);

// Disconnect when done
mouse.disconnect();

// Minimal example without wrapper
fn send_mouse_command(port: &mut Box<dyn serialport::SerialPort>, dx: i32, dy: i32) {
    let cmd = json!({ "cmd": "move", "dx": dx, "dy": dy });
    let line = format!("{}\n", cmd.to_string());
    let _ = port.write_all(line.as_bytes());
    let _ = port.flush();
}
```

## Pico/ESP32 Firmware Compatibility

Your Pico firmware (from Desktop/New folder) already supports this protocol. Ensure:

1. **Baud Rate**: CDC runs at 115200 (verify in firmware)
2. **JSON Format**: Commands are one JSON object per line with `\n` terminator
3. **DTR/RTS**: PC-side serial code asserts DTR/RTS so Pico sends data back

If using `serialport-rs`, DTR/RTS are asserted in the `InputMethodSerialMouse::new()` method automatically (if needed).

## Debugging

### Enable serial logging

The SerialMouse backend logs all commands sent and errors:

```rust
// In your logger setup:
log_debug("Initializing SerialMouse on COM3 @ 115200");
log_debug("SerialMouse port opened successfully");
log_debug("SerialMouse sent: {\"cmd\":\"move\",\"dx\":10,\"dy\":-5}");
log_warning("SerialMouse move failed: serial write failed");
```

### Verify port communication

Use a serial monitor (e.g., PuTTY at 115200 baud) to confirm:
- Port opens without errors
- Commands arrive as valid JSON lines
- Device responds appropriately

### Check port availability

```rust
use serialport;

let ports = serialport::available_ports().unwrap();
for p in ports {
    println!("Found: {}", p.port_name);
}
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Port not found" | Verify device is connected, check COM port number in Device Manager |
| "Permission denied" | Ensure no other application is using the serial port |
| "Connection timeout" | Check baud rate matches device (115200 for Pico) |
| "JSON not received" | Verify DTR/RTS assertions; check firmware serial init |
| "Commands arrive but no movement" | Check Pico firmware command parsing matches protocol |
| "Cannot reconnect" | Ensure `disconnect()` was called before reconnecting |

## Architecture Decisions

1. **JSON-based**: Human-readable, language-agnostic, extensible
2. **Synchronous**: Fire-and-forget commands; device handles buffering
3. **No state tracking**: Backend doesn't track button state (firmware does)
4. **Modular logging**: All operations logged for debugging
5. **Separate from network**: Uses only serial ports, no TCP/Bluetooth

## Future Extensions

Possible enhancements (not yet implemented):

- Gesture sequences (multi-command batches)
- Response verification (wait for ACK)
- Latency measurement and reporting
- Firmware version detection
- Over-the-air updates

## File Structure

```
rust-macro/
├── src/modules/input/
│   ├── serial_mouse.rs      ← New backend (standalone reusable)
│   ├── mouse_input.rs       ← Updated abstraction layer
│   ├── mod.rs               ← Updated exports
│   ├── gfck.rs              ← Existing backend
│   └── ghub.rs              ← Existing backend
├── Cargo.toml               ← Updated (added serialport dependency)
└── ...
```

## Code Example: Full Integration

```rust
// In main.rs or initialization code
use crate::modules::input::MouseInput;

fn init_mouse_input(config: &ConfigParser) -> Result<MouseInput, Box<dyn std::error::Error>> {
    let method = config.get("input", "method").unwrap_or("SerialMouse".to_string());
    
    match method.as_str() {
        "SerialMouse" => {
            let port = config.get("input", "serial_port").unwrap_or("COM3".to_string());
            let baud: u32 = config.get("input", "baud_rate")
                .and_then(|b| b.parse().ok())
                .unwrap_or(115200);
            MouseInput::new_with_serial(&port, baud)
        }
        "GFCK" => {
            let gfck_dll = PathBuf::from("path/to/gfck.dll");
            let ghub_dll = PathBuf::from("path/to/ghub.dll");
            unsafe { MouseInput::new(gfck_dll, ghub_dll) }
        }
        _ => Err("Unknown input method".into())
    }
}

// Usage with connection management
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let mouse = init_mouse_input(&config)?;
    
    // Check connection status
    if mouse.is_serial_connected() {
        println!("Mouse connected!");
    }
    
    // Move mouse
    mouse.move_relative(50, -25);
    
    // Graceful disconnect
    mouse.disconnect_serial();
    
    // Can reconnect later
    mouse.connect_serial()?;
    mouse.move_relative(10, 10);
    
    Ok(())
}
```

## License

This backend follows the same license as rust-macro. It's designed to be copied and adapted freely.

---

**Questions?** Check the [PROGRESS.md](../PROGRESS.md) for implementation details or review the source code in [serial_mouse.rs](src/modules/input/serial_mouse.rs).
