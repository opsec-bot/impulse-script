# Bench hardware inventory

Physical parts on hand for the mouse pass-through build. Boards have their own
reference docs; this is the parts list + what each thing is for. See
[[pc-hid-passthrough]] for how they connect.

## Boards

| Item | Key specs | Role | Doc |
|---|---|---|---|
| **ESP32-S3-USB-OTG** (official Espressif) | ESP32-S3-MINI-1-N8, 8 MB flash, no PSRAM | Phase-1 USB host (dongle decode). **Retired** — bare GPIO47/48 pads. | [[ESP32-S3-USB-OTG-DOCS]] |
| **ESP32-S3-DevKitC-1-N32R16V** | WROOM-2, 32 MB OPI flash / 16 MB OPI PSRAM (run as N16R8) | Current USB host (parses dongle HID) | [[esp32-s3-n32r16v]] |
| **Raspberry Pi Pico H** — *Bicool, pre-soldered header* | RP2040 dual Cortex-M0+ @ up to 133 MHz, **264 KB SRAM**, **2 MB** flash, 26 GPIO, USB 1.1 device+host | USB device to the PC (HID mouse + CDC), `046D:C547` identity | [[Pi-Pico-H-DOCS]] |

> The Bicool board is a standard pre-soldered RP2040 ("Pico H" equivalent); its
> castellated + pre-soldered headers are why it's usable on the breadboard without a
> soldering iron. Note the Amazon listing's "264 MB RAM" is a typo — RP2040 has
> **264 KB SRAM**. (If this is a *second/spare* Pico rather than the one already in the
> chain, say so and I'll split it out.)

## Cables & accessories

| Item | Spec | Used for |
|---|---|---|
| **USB-A Female → XH2.54 4-pin converting cable, 30 cm** | USB-A female socket → 4-pin JST-XH (2.54 mm) flying leads | Breaks the Lightspeed dongle's 4 USB lines out to the DevKitC host headers (VBUS/D−/D+/GND → 5V/GPIO19/GPIO20/GND). Wiring + the pin-order caveat are in [[esp32-s3-n32r16v]]. |
| **Amazon Basics USB-A → Micro-USB cable, 10 ft** | USB 2.0, 480 Mbps data-capable, gold-plated, up to 2100 mA charge | Flashing + serial console to the ESP boards via their Micro-USB **`UART`** (CP210x) port. Data-capable (not charge-only), which is what the bridge needs. 10 ft is long but the CP210x link is low-speed UART, so length is a non-issue here. |
| **Busboard BB400 solderless breadboard** | 400 tie-points (300 IC-area + two 50-pt strips = 4 power rails), 21–26 AWG, 36 V / 2 A | Solderless distribution/junction for the inter-board wiring — lets the USB-A XH leads, the DevKitC header pins, and the Pico's UART pins share clean common rails (5V/GND) and get jumpered without soldering. |
| **Micro-USB cable** (Pico's, in box) | data-capable | Pico H → PC (the USB HID + CDC device connection) |
| Jumper wires (F-F, M-F, M-M, assorted) | — | Inter-board UART + breadboard hookups |
| **Logitech G Pro X Superlight + Lightspeed dongle** | dongle `046D:C547`, ~98 mA, 1000 Hz | The mouse being passed through |

## Notes

- The **breadboard is a deliberate addition** — earlier the project assumed no
  breadboard and no soldering iron (`HANDOFF.md` §11.3–4). With the BB400 on hand, the
  clean solderless path is: XH flying leads and both boards' pins land on the
  breadboard, sharing common 5V/GND rails, jumpered to the exact pins per
  [[esp32-s3-n32r16v]]. This also sidesteps the XH-housing-won't-mate-directly issue.
- **Still no soldering iron** — keep to the pre-soldered headers + breadboard + jumper
  approach unless a soldering iron gets added here.

## See also
- [[pc-hid-passthrough]] — how these parts connect · [[esp32-s3-n32r16v]] · [[Pi-Pico-H-DOCS]] · [[ESP32-S3-USB-OTG-DOCS]] · [[notes]] · [[index]]
