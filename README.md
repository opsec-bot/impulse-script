# Rust Rust Macro - Recoil Control System

Recoil control script built in Rust with ImGui for Rainbow Six Siege X

## Features

- **ACOG Toggle**: Separate recoil patterns for ACOG (2.5x)
- **Global Hotkeys**: Bind to default functions + wep selection
- **Ghost Mode**: Hide from Alt+Tab and screen capture
- **Auto-Sensitivity Scaling**: Automatically adjusts recoil values when sensitivity changes
- **Multiple Input Methods**: GFCK driver (universal) and GHub (Logitech only)
- **Add weapons**: If a weapon isn't already added you can add it

## Usage

1. Run rust-macro.exe
2. Select weapon
3. Configure X/Y recoil values (-2 to 2 for X, 1 to 10 for Y)
4. Enable RCS and test

## Configuration

- **X**: Horizontal recoil compensation
- **Y**: Vertical recoil compensation  
- **Xmod**: Special modifiers (-1 alternating, 0 one-time, 1 standard)
- **RPM**: Weapon rate of fire for timing calculations [(Fetch New Values Here)](https://rainbowsix.fandom.com/wiki/Category:Weapons_of_Tom_Clancy%27s_Rainbow_Six_Siege)

## Configuration Files

The application automatically creates and manages configuration files:

- `settings.ini`: Main configuration (sensitivity, DPI, weapon data)
- `hotkeys.ini`: Hotkey bindings
- Automatic backup and recovery system

## Advanced Features

### Auto-Sensitivity Scaling

When game sensitivity changes, the application automatically:

1. Detects sensitivity modification
2. Calculates scaling factor
3. Updates all weapon recoil values proportionally
4. Maintains relative weapon balance

### Ghost Mode

- Hide from Alt+Tab switcher
- Invisible to screen capture software
- Process name obfuscation
- Memory protection techniques

## Troubleshooting

### Common Issues

**DLL Not Found**

- Ensure lib folder is in same directory as executable
- Check antivirus hasn't quarantined the files

**Hotkeys Not Working**

- Check for conflicting hotkeys with other applications
- Ensure hotkeys are properly bound in the Hotkeys tab

**Mouse Input Not Responding**

- Switch between GFCK and GhubMouse methods in Hotkeys tab
- GFCK is recommended for most users (default option)
- GhubMouse only works with compatible Logitech G-series mice
- Verify mouse drivers are properly installed
- Check DPI settings match your mouse configuration

**RCS Not Accurate**

- Verify game sensitivity matches Settings tab values
- Check DPI setting is correct
- Ensure weapon RPM values are accurate
- Test with different Xmod values

## Building from Source

### Prerequisites

- Rust 1.70+
- Windows SDK
- MSVC Build Tools

### Build Steps

```bash
git clone https://github.com/opsec-bot/rust-macro.git
cd rust-macro
cargo build --release
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Disclaimer

This software is intended for educational and personal use only. Users are responsible for ensuring compliance with game terms of service and applicable laws. The developers are not responsible for any consequences resulting from the use of this software.

## Credits

- [Villageslayer](https://www.unknowncheats.me/forum/members/3298005.html) — for his public project & Data collection:
  - [Mouse NoRecoil 1.0 (helpy, auto config)](https://www.unknowncheats.me/forum/rainbow-six-siege/620039-mouse-norecoil-1-0-recoil-helpy-auto-config-probably-ud-universal.html)
  - [R6 Recoil Info / Data Collection (Operators & Attachments)](https://www.unknowncheats.me/forum/rainbow-six-siege/681544-r6-recoil-info-data-collection-operator-attachments.html)

---
