# PC HID mouse pass-through — system reference

The connective doc for the three boards. The individual board files
([[ESP32-S3-USB-OTG-DOCS]], [[esp32-s3-n32r16v]], [[Pi-Pico-H-DOCS]]) cover the
silicon; this one covers what the boards **do together** and, specifically, what the
**PC** sees at the end of the chain.

Source of truth: the working project at `C:\Users\tav08\Desktop\New folder`
(`mouse-passthrough`) — its `README.md`, `PROGRESS.md`, and `HANDOFF.md`. Keep this
file in sync with that project's `PROGRESS.md`.

This is an input-remapping / accessibility / automation device, **not** a game cheat.

## The chain

```
[Logitech G Pro X Superlight] ~RF~ [Lightspeed dongle]
     -> USB host  -> ESP32-S3-DevKitC-1-N32R16V (parses HID)
     -> UART 1Mbaud (GPIO47->GP1, GP0->GPIO48, shared GND)
     -> Raspberry Pi Pico H (TinyUSB device)
     -> USB       -> PC : HID mouse  (cursor moves)
                       -> PC : CDC serial (control app)
```

The physical mouse never talks to the PC directly. The Pico is the only thing the PC
enumerates, and it presents as **two interfaces at once** (a composite device).

## What the PC sees (the "PC HID pass-through" end)

The Pico H enumerates on the PC as a single composite USB device with two roles:

| Interface | What it is | PC-side handle |
|---|---|---|
| **USB HID mouse** | Standard boot-compatible mouse. Replays the dongle's decoded reports (buttons, dx/dy int16, wheel, hwheel). This is what moves the cursor. | Windows mouse driver — no driver install, no vendor software |
| **USB CDC serial** | A virtual COM port for the control app. Carries the §9 JSON-line protocol. | **COM15** on this machine (see note below) |

- **USB identity: `046D:C547` — "Logitech USB Receiver"** (set on the Pico via
  TinyUSB `setID/​setManufacturerDescriptor/​setProductDescriptor` + `detach()/attach()`).
  This is a generic Logitech receiver ID, chosen so the wired device reads as an
  ordinary mouse. Default TinyUSB `239A:CAFE` is no longer used.
  - Allowed per the project's hard rules (generic vendor IDs OK; cheat-device IDs
    like KMBox/Cronus/Titan are **banned**). See `HANDOFF.md` §11.
- **The HID report descriptor is unchanged** by the identity swap, so pass-through and
  remap behave identically before/after Phase 4.

### COM-port gotcha (identity change renumbers the port)
Changing VID/PID makes Windows treat the Pico as a **new** device, so its CDC port
renumbered **COM14 → COM15**. The control app auto-detects **both** the old
(`239A:CAFE`) and new (`046D:C547`) IDs, so selection still works either way. A stray
`303A:1001` (Espressif native USB) may also appear — that's ESP-side, ignore it.

## The PC-side control app

A **Tauri v2 desktop app** ("Superlight Control", Rust backend + React/Tailwind/shadcn,
in `rust-client/`) talks to the Pico over the CDC port. It does **not** touch the HID
interface — cursor pass-through runs on its own regardless of whether the app is open.

Capabilities over CDC:
- **Query** live mouse state / device status (1 Hz heartbeat).
- **Inject** programmatic movement, clicks, scroll (`move` / `click` / `scroll`).
- **Watch** — live stream of incoming mouse events as JSON.
- **Remap** per-button rules (button→button, or "none" to disable). The app sends the
  rule to the Pico, which relays it to the ESP32, which rewrites the buttons bitfield
  **at the source**. Keystroke remaps (e.g. Mouse4→Ctrl+C) need a keyboard-HID
  interface on the Pico and are deferred.

Rust-side rule: blocking `serialport` on a dedicated `std::thread`, **no tokio/async**
(project hard rule §11.6). One important fix: **CDC device→host writes are gated on
DTR** (`tud_cdc_connected()`); `serialport-rs` leaves DTR low on open, so the app must
assert `write_data_terminal_ready(true)` (+ RTS) right after opening the port, or all
status/event/ack lines silently vanish. (pyserial asserts DTR by default, which is why
early Python tests worked and the Rust app first didn't.)

## Protocols

### Pico <-> PC (USB CDC, JSON lines)
- **PC -> Pico:** `move{dx,dy}`, `click{btn}`, `scroll{wheel}`, `status`, `watch{on}`,
  `remap{from,to}`.
- **Pico -> PC:**
  - `{"type":"ack","what":..}`
  - `{"type":"status","uptime_ms":..,"esp_alive":..,"frames_ok":..,"frames_bad":..,"watching":..}` (1 Hz)
  - `{"type":"evt","kind":"mouse","buttons":..,"dx":..,"dy":..,"wheel":..}` (while watching)
- **Hot-path safety:** the watch stream only writes when
  `Serial.availableForWrite()` has room for the whole line; it drops events rather than
  blocking pass-through, protecting the latency budget.

### ESP32 <-> Pico (internal UART, binary frame)
```
[0xAA][0x55][len:1][type:1][payload:len][crc8:1]
```
- `type=0x01` HID_MOUSE — payload `{buttons:u8, dx:i16, dy:i16, wheel:i8, hwheel:i8}`
- `type=0x02` PC_CMD_TO_ESP — JSON line forwarded from the PC via the Pico (remap rules)
- `type=0x03` ESP_REPLY_TO_PC — reply relayed back out the Pico's CDC (partly deferred)

The decoded dongle mouse report is 13 bytes, no report ID, endpoint 0x81:
`[btn 1-8][btn 9-16][dX i16 LE][dY i16 LE][wheel i8][hwheel i8][vendor x5]`.

## Status

| Phase | What | Status |
|---|---|---|
| 0 | Blink + serial heartbeat, both boards | done |
| 1 | Dump + decode dongle HID reports (`046D:C547`, 13B report) | done |
| 2 | ESP32 -> UART -> Pico -> PC HID mouse; cursor moves end-to-end | done |
| 3a | Pico CDC + Tauri app (move/click/scroll/watch) | done |
| 3b | Button remap (ESP32 intercept, reverse-UART GP0->GPIO48) | done |
| 4 | `046D:C547` "Logitech USB Receiver" identity + remap intercept | done |

**Remaining polish (none blocks a working device):**
- Confirm all 5 buttons + scroll pass through (movement confirmed; buttons pending a
  tick-through).
- Measure added latency — the last open acceptance box. Target ≤2 ms added, ≤5 ms hard
  cap; expected sub-2 ms by construction (1 ms dongle poll + ~0.12 ms UART@1Mbaud +
  ~1 ms Pico USB poll) but unmeasured. See [[notes]] "Future / optional work".
- Keyboard-HID on the Pico to unlock keystroke remaps (composite mouse+keyboard).
- Persist the remap table on the ESP (NVS) so rules survive reboot.
- README end-to-end usage section; tag `v0.1.0`.

## Cross-references
- [[ESP32-S3-USB-OTG-DOCS]] — Phase-1 host board (retired; bare GPIO47/48 pads forced
  the move to the DevKitC).
- [[esp32-s3-n32r16v]] — current host board, build flags (`FlashMode=opi`,
  `USBMode=default`), and host wiring.
- [[Pi-Pico-H-DOCS]] — the PC-facing device board.
- [[inventory]] — bench hardware parts list (boards, cables, breadboard, dongle).
- [[notes]] — lab notebook: COM-port map, compile/upload commands, observations.
- [[index]] — vault map.
