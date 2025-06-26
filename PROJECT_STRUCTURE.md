# Project Structure

## Directory Layout

```bash
src/
├── main.rs                 # Application entry point with ImGui main loop
├── lib.rs                  # Library root (module exports)
└── modules/
    ├── mod.rs              # Module declarations and re-exports
    ├── input/              # Mouse input handling and DLL interfaces
    │   ├── mod.rs          # Input module exports
    │   ├── gfck.rs         # GFCK DLL interface implementation
    │   ├── ghub.rs         # Logitech G HUB DLL interface
    │   ├── mouse_input.rs  # Multi-backend mouse input abstraction
    │   └── mouse_command.rs # Mouse command enum definitions
    ├── core/               # Core recoil control and system logic
    │   ├── mod.rs          # Core module exports
    │   ├── control.rs      # Main recoil control thread and logic
    │   ├── hotkey_handler.rs # Global hotkey detection and handling
    │   ├── process_ghost.rs # Window hiding and stealth features
    │   └── xmod_state.rs   # X-axis modification state tracking
    ├── ui/                 # User interface components
    │   ├── mod.rs          # UI module exports
    │   ├── support.rs      # ImGui initialization and window management
    │   └── keybinds.rs     # Key code conversion utilities
    └── config/             # Configuration management
        ├── mod.rs          # Config module exports
        ├── settings.rs     # Generic INI settings handler
        ├── settings_io.rs  # Application-specific settings I/O
        ├── setup_class.rs  # Game settings auto-import from R6S
        └── weapon_data.rs  # Default weapon definitions and classes

lib/                        # External DLL dependencies
├── GFCK.dll               # Hardware mouse driver (universal)
└── ghub_mouse.dll         # Logitech G-series mouse driver

.github/
└── workflows/
    └── windows-build-release.yml # Automated Windows build and release
```

## Key Components

### Input Module

- **Multi-backend Support**: GFCK (universal) and GHub (Logitech-specific) drivers
- **Hardware-level Input**: Low-level mouse movement simulation
- **Command Pattern**: Abstracted mouse commands for consistent interface

### Core Module  

- **Recoil Control**: Threaded recoil compensation with DPI/sensitivity scaling
- **Hotkey System**: Global hotkey detection that works when game is focused
- **Process Stealth**: Hide from Alt+Tab and screen capture tools
- **State Management**: X-axis modification patterns and weapon switching

### UI Module

- **ImGui Interface**: Tabbed interface for configuration and control
- **Real-time Updates**: Live weapon configuration and testing
- **Key Capture**: Interactive hotkey binding system

### Config Module

- **Auto-import**: Reads Rainbow Six Siege settings automatically
- **Weapon Database**: Pre-configured weapon profiles with RPM data
- **Persistence**: INI-based configuration with automatic backup
- **Sensitivity Scaling**: Automatic recoil adjustment when sensitivity changes

## Data Flow

```
Game Settings Import → Settings I/O → Control Logic → Mouse Input → Hardware
                ↓
           Configuration UI ← User Input ← Hotkey Handler
```

## Build Artifacts

- `rust-macro.exe`: Main executable (Windows release build)
- `config.ini`: Auto-generated configuration file
- `lib/`: Required DLL dependencies for mouse input
