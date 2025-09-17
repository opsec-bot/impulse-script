# Impulse - Recoil Control Script

Recoil control script built in Rust with ImGui for Rainbow Six Siege X


Discord Server: https://discord.gg/pfG39Ekq6T

## Features

- **ACOG Toggle**: Separate recoil patterns for ACOG (2.5x)
- **Global Hotkeys**: Bind to default functions + wep selection
- **Ghost Mode**: Hide from Alt+Tab and screen capture
- **Auto-Sensitivity Scaling**: Automatically adjusts recoil values when sensitivity changes
- **Multiple Input Methods**: GFCK driver (universal) and GHub (Logitech only)
- **Add weapons**: If a weapon isn't already added you can add it

## Prerequisites

Doesn't matter if you have a logitech mouse or not

- [Logitech G HUB 2021.11.1775](https://www.unknowncheats.me/forum/downloads.php?do=file&id=39879)
- [How to set up G HUB](https://www.youtube.com/watch?v=FgNEciqHZFE) — YouTube guide for proper installation

## Usage

1. Run impulse.exe
2. Select weapon
3. Configure X/Y recoil values (-2.0 to 2.0 for X, 1.0 to 10.0 for Y)
4. Enable RCS and test

## Configuration

- **X**: Horizontal recoil compensation (-2.0 to 2.0, supports decimals like 0.5)
- **Y**: Vertical recoil compensation (1.0 to 10.0, supports decimals like 1.5)
- **Xmod**: Special modifiers (-1.0 alternating, 0.0 one-time, 1.0+ standard, supports decimals)
- **RPM**: Weapon rate of fire for timing calculations [(Fetch New Values Here)](https://rainbowsix.fandom.com/wiki/Category:Weapons_of_Tom_Clancy%27s_Rainbow_Six_Siege)

## Configuration Files

The application automatically creates and manages configuration files:

- `config.ini`: Main configuration (sensitivity, DPI, weapon data)
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

**Adding more soon**

## Building from Source

### Prerequisites

- Rust 1.70+
- Windows SDK
- MSVC Build Tools

### Build Steps

```bash
git clone https://github.com/opsec-bot/impulse-script.git
cd impulse-script
cargo build --release
```

Ensure that the lib and output executable are in the same folder.

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
