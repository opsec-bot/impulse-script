# Project Structure

## Directory Layout

```bash
src/
├── main.rs                 # Application entry point
├── lib.rs                  # Library root
└── modules/
    ├── mod.rs              # Module declarations and re-exports
    ├── input/              # Input handling (mouse, DLLs)
    │   ├── mod.rs
    │   ├── gfck.rs         # GFCK DLL interface
    │   ├── ghub.rs         # GHUB DLL interface
    │   ├── mouse_input.rs  # Mouse input abstraction
    │   └── mouse_command.rs # Mouse command types
    ├── core/               # Core application logic
    │   ├── mod.rs
    │   ├── control.rs      # Recoil control logic
    │   ├── calculator.rs   # Sensitivity calculations
    │   └── xmod_state.rs   # X-modification state
    ├── ui/                 # User interface
    │   ├── mod.rs
    │   ├── support.rs      # ImGui support functions
    │   └── keybinds.rs     # Keybinding utilities
    └── config/             # Configuration and settings
        ├── mod.rs
        ├── settings.rs     # Settings file handling
        ├── settings_io.rs  # Settings I/O operations
        └── setup_class.rs  # Game settings import
```

## Key Components

- **Input Module**: Handles all mouse input through different DLL backends
- **Core Module**: Contains the main recoil control logic and calculations
- **UI Module**: ImGui interface and user interaction
- **Config Module**: Settings management and game configuration import
