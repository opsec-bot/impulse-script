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
    │   ├── xmod_state.rs   # X-axis modification state tracking
    │   └── logger.rs       # Debug logging system
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
├── instructions/
│   └── RustC.instructions.md # Rust programming copilot instructions
└── workflows/
    └── windows-build-release.yml # Automated Windows build and release
```

## Key Components

### Input Module

- **Multi-backend Support**: GFCK (universal) and GHub (Logitech-specific) drivers
- **Hardware-level Input**: Low-level mouse movement simulation via DLL interfaces
- **Command Pattern**: Abstracted mouse commands (Move, Click, Down, Up) for consistent interface
- **Runtime Switching**: Dynamic switching between mouse input methods

### Core Module  

- **Recoil Control**: Threaded recoil compensation with DPI/sensitivity scaling and timing
- **Hotkey System**: Global hotkey detection with customizable bindings (Exit, Toggle, Hide, Weapon selection)
- **Process Stealth**: Hide from Alt+Tab switcher and screen capture tools using Windows API
- **State Management**: X-axis modification patterns (-1 alternating, 0 one-time, custom multipliers)
- **Debug Logging**: Comprehensive logging system with file output to temp directory

### UI Module

- **ImGui Interface**: Tabbed interface with Recoil Control, Hotkeys, Settings, and Extras tabs
- **Real-time Updates**: Live weapon configuration with decimal precision sliders
- **Key Capture**: Interactive hotkey binding system with visual feedback
- **Window Management**: Resizable window support with dynamic frame rate optimization

### Config Module

- **Auto-import**: Reads Rainbow Six Siege GameSettings.ini automatically from user documents
- **Weapon Database**: Pre-configured weapon profiles with RPM data for all R6S weapons
- **Persistence**: INI-based configuration with automatic backup and recovery
- **Sensitivity Scaling**: Automatic recoil adjustment when in-game sensitivity changes
- **ACOG Support**: Separate recoil patterns for ACOG (2.5x) scope configurations

## Data Flow

```
Game Settings Import → Settings I/O → Control Logic → Mouse Input → Hardware
                ↓
           Configuration UI ← User Input ← Hotkey Handler ← Global Key Detection
                ↓
            Debug Logger → File Output (impulseDebug.txt)
```

## Configuration System

- **config.ini**: Main configuration file with game settings, weapon data, and hotkeys
- **Automatic Detection**: Finds most recent GameSettings.ini from R6S installation
- **Default Weapons**: 100+ pre-configured weapons with accurate RPM values
- **Weapon Classes**: Organized by type (AR, SMG, LMG, DMR, MP, SG, Handgun, etc.)

## Threading Architecture

- **Main Thread**: UI rendering and event handling
- **Control Thread**: Recoil compensation loop with mouse button detection
- **Channel Communication**: Message passing between threads for commands and mouse movements

## Build Artifacts

- `impulse.exe`: Main executable (Windows release build)
- `config.ini`: Auto-generated configuration file
- `lib/`: Required DLL dependencies for mouse input
- `impulseDebug.txt`: Debug log file in system temp directory

## Features

- **Decimal Precision**: Slider values support decimals (0.5, 1.2, etc.) for fine-tuned control
- **Dynamic Frame Rate**: 100 FPS when active, 30 FPS when idle for performance optimization
- **Ghost Mode**: Complete stealth from Alt+Tab and screen capture detection
- **Auto-Sensitivity Scaling**: Proportional recoil adjustment when game sensitivity changes
- **Comprehensive Weapon Support**: All Rainbow Six Siege weapons with accurate timing data
