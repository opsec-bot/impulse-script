# Lab notebook

Free-form notes. Append as we go.

## This machine

- OS: Windows 10 Home (19045)
- `arduino-cli` 1.5.1 at `C:\Users\tav08\arduino-cli_1.5.1_Windows_64bit\arduino-cli.exe`
- esp32 core: **3.0.0** installed
- Libraries: EspUsbHost 1.0.1, Adafruit NeoPixel, ESP32 BLE Mouse
- Python 3.13 with pyserial 3.5
- Rust/cargo present

### COM-port map (Phase 1, OTG board — 2026-06-05)

| Port | Device | VID:PID | Role |
|---|---|---|---|
| COM12 | Silicon Labs CP210x USB-UART bridge | 10C4:EA60 | ESP32-S3-USB-OTG micro-USB port — upload + serial |
| COM13 | Espressif native USB CDC | 303A:1001 | OTG board USB_DEV edge connector |
| COM1 | Motherboard serial | — | ignore |

### COM-port map (Phase 2, DevKitC host — 2026-06-07)

OTG board **retired** (Phase 1 reverse-engineering done). Host is now the
**ESP32-S3-DevKitC-1-N32R16V** — its free headers let us attach the UART link to
the Pico without soldering (the OTG board's GPIO47/48 are bare pads). See
[[esp32-s3-n32r16v]] for the full board reference + verified build flags.

| Port | Device | VID:PID | Role |
|---|---|---|---|
| **COM7** | Silicon Labs CP210x (CP2102) | 10C4:EA60 | **DevKitC `UART` port — upload + serial console** |
| **COM14→COM15** | Pico H — USB HID mouse + CDC | 239A:CAFE → **046D:C547** | After Phase 4 the Pico wears the "Logitech USB Receiver" identity, so Windows assigned a **new COM** (was COM14, now COM15). The app auto-detects both IDs. |
| COM1 | Motherboard serial | — | ignore |

> A `303A:1001` (Espressif native USB) may also appear — that's an ESP-side port,
> not our Pico; ignore it for the app.

> Both 10C4:EA60 boards share the same VID:PID; they're distinguished only by USB
> serial number. The DevKitC's native `USB` port (303A:1001) is left **unplugged**
> so GPIO19/20 are free for the dongle.

## Compile / upload (ESP32 host, Phase 1)

> NOTE: the dedicated `esp32s3usbotg` board variant **presets** flash size (8MB),
> PSRAM (off), etc. — so the `FlashSize`/`PSRAM`/`CDCOnBoot` options from HANDOFF
> §5 are INVALID on this FQBN (they only apply to the generic "ESP32S3 Dev
> Module"). The only knob is `USBMode`.
>
> **Use `USBMode=default`, NOT `hwcdc`.** On this board the menu bundles flags:
>   - `USBMode=hwcdc`   -> usb_mode=1, **cdc_on_boot=1** -> `Serial` = native USB
>     CDC. We reroute native USB to the host port (GPIO18 HIGH), so that CDC is
>     physically on the host connector and INVISIBLE to the PC. Result: no serial.
>   - `USBMode=default` -> usb_mode=0, **cdc_on_boot=0** -> `Serial` = UART0 ->
>     CP210x bridge -> COM12. ✓ And the OTG controller stays free for EspUsbHost
>     to claim as host. THIS is what we want.
> (HANDOFF §5's "Hardware CDC and JTAG" advice was for the generic Dev Module and
> backfires here. Verified against esp32 core 3.0.0 boards.txt.)

```powershell
# Compile
arduino-cli compile `
  --fqbn "esp32:esp32:esp32s3usbotg:USBMode=default" `
  ".\firmware\esp32_host"

# Upload (serial/UART mode, via the CP210x bridge on COM12)
arduino-cli upload -p COM12 `
  --fqbn "esp32:esp32:esp32s3usbotg:USBMode=default" `
  ".\firmware\esp32_host"
```

Side effect (harmless, even useful): opening COM12 asserts DTR/RTS, which
auto-resets the ESP32. So every capture start reboots the board -> the dongle
re-enumerates -> the descriptor dump reprints. Good for grabbing descriptors.

## Capture serial

```powershell
python .\scripts\capture_serial.py COM12 .\captures\dongle-descriptors.log
```

`Serial` on this firmware = UART0 -> CP210x -> COM12 (NOT the native USB port).

## Observations

- 2026-06-05: First capture attempt with `USBMode=hwcdc` produced only ROM boot
  text + `E USBH: Device 1 gone` on COM12 — no banner, no reports. Cause: Serial
  was on native USB CDC (rerouted to host port). Fixed by `USBMode=default`.
- The Arduino IDE's `serial-monitor` helper held COM12 (`Access is denied`),
  blocking uploads. Closing the IDE freed it. We're CLI-only now.
- Phase 1 decode results live in `PROGRESS.md`. Short version:
  **046D:C547**, mouse on EP 0x81, 13-byte report, no report ID,
  `[btn8][btn8][dxLE16][dyLE16][wheel8][hwheel8][vendor×5]`.
- Decoded sanity check: L/R/M/back/fwd all map to bits 0–4 of byte 0; wheel
  up=+1, down=−1; dx/dy signed and direction-correct. Max flick seen ±53
  (16-bit width confirmed by descriptor, not by clipping).

## Future / optional work (post-v0.1.0)

Deferred items, roughly by value. Nothing here blocks v0.1.0 — the full chain
(passthrough + control app + remap + Logitech identity) works.

- **Latency measurement.** The last unchecked Phase-2 acceptance box (≤2 ms added,
  ≤5 ms cap). Needs an instrumented test — e.g. a GPIO loopback timed with a logic
  analyzer (dongle report in → Pico HID out), or a documented method in
  `scripts/latency_test.py`. Expected sub-2 ms by construction (1 ms dongle poll +
  ~0.12 ms UART@1Mbaud + ~1 ms Pico USB poll), but unproven by measurement.
- **Keystroke remaps (keyboard HID).** Today remap is button→button / disable only.
  Add a keyboard HID interface to the Pico (composite mouse+keyboard via TinyUSB)
  and a key-combo parser ("ctrl+c", "alt+tab") so the ESP intercept can fire real
  keystrokes. The app's remap "to" field would regain free-text/keychord entry.
- **ESP→PC remap reply (§9 type=0x03).** The app currently shows only the Pico's
  local ack. Have the ESP confirm the applied remap table back through the Pico so
  the UI can reflect actual ESP state (and surface unknown-action rejections).
- **Persist remap table on the ESP** (NVS/Preferences) so rules survive a reboot.
- **App polish:** show the live remap table, a light/dark toggle (currently dark
  only), and a window icon.
- **Status-LED tuning:** brightness/idle-breathe taste; maybe a distinct colour for
  "remap + disconnected".
- **README usage section:** end-to-end build/flash/wire/run for a fresh clone.


---
> Part of the [[pc-hid-passthrough]] mouse HID pass-through project. See also [[inventory]] · [[notes]] · [[index]].
