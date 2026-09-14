# Putting images on the OTG board

How images get from a file on the PC onto the **ESP32-S3-USB-OTG**'s 1.3" ST7789
(240×240) screen, via the pocket meme-viewer firmware. Companion to the board
reference [[ESP32-S3-USB-OTG-DOCS]]. Source project: `Desktop/funnymemeESp32`.

> Note: this is a **separate project** from the mouse pass-through — the OTG board
> was retired as a USB host there ([[inventory]]), but it's a fine little image
> viewer. Images live in **onboard flash (LittleFS)**, not on the SD card.

---

## The pipeline in one line

`prepare_images.py` → `data/images/*.jpg` → `pio run -t uploadfs` → flash
(`littlefs` partition) → firmware scans `/images` on boot.

There is **no runtime upload** — you re-flash the filesystem whenever the image set
changes. The screen only ever reads what's baked into flash.

---

## 1. Prep the images (PC side)

The firmware wants exactly **240×240**, so normalize sources first:

```powershell
python tools/prepare_images.py path/to/pics --output data/images
```

- Accepts a single file **or** a directory (jpg/jpeg/png/bmp/gif/webp/tif).
- Auto-trims baked-in black letterbox bars, then resizes, writes `<name>.jpg`.
- `--mode fill` (default) = scale to cover + crop overflow → edge-to-edge, no bars
  (crops the long edge of a non-square image).
- `--mode fit` = letterbox → nothing cropped, remainder padded black.
- Other flags: `--size W H` (default `240 240`), `--quality 1-100` (default `88`).
- Needs Pillow (`pip install pillow`).

> Run it on the **original full-res** image, not an already-shrunk copy — sharper.

## 2. Where they land

- `data/` is the LittleFS image root; `data/images/` → `/images` on the device.
- So a prepped `data/images/shrek.jpg` shows up to the firmware as
  `/images/shrek.jpg`.
- Both **JPG and PNG** render (JPG via TJpg_Decoder, PNG via PNGdec). The prep
  script emits JPG.

## 3. Flash the filesystem

```powershell
pio run -t buildfs                        # build the image only (safe dry check)
pio run -t uploadfs --upload-port COM12   # flash it to the littlefs partition
```

- Flash over **COM12** (the CP210x micro-USB bridge), same port as firmware +
  serial. See [[notes]] for the COM-port map.
- `uploadfs` writes to offset `0x510000` per `partitions.csv`. This is a **separate**
  step from `pio run -t upload` (firmware) — changing images needs only `uploadfs`.

## 4. Device side (what the firmware does)

- Mounts LittleFS, scans `IMAGE_ROOT` = `/images`, sorts, and displays.
- UP / DOWN browse; MENU toggles slideshow (auto-advance every 3 s).
- Empty/missing `/images` → clear "no images" error screen instead of a hang.

---

## Gotchas worth remembering

**LittleFS partition label.** `partitions.csv` names the data partition `littlefs`
(subtype `spiffs`), but `LittleFS.begin()` defaults to hunting for a partition named
`spiffs` → `partition "spiffs" could not be found`, mount error 261. The firmware
passes the label explicitly:
```cpp
LittleFS.begin(true, "/littlefs", 10, "littlefs");
```

**JPEG scaler steps by powers of two only.** TJpg_Decoder downscales by 1/2, 1/4,
1/8 — nothing in between. A 256-px source halves to 128 and renders as a small
centered block. **Pre-size to exactly 240×240** so it draws at scale 1, full screen.
This is the whole reason step 1 exists.

**Flash budget.** The `littlefs` partition is `0x2F0000` ≈ **2.94 MB** (8 MB flash,
5 MB app). At ~10–15 KB per 240×240 JPEG (quality 88) that's a couple hundred images
before it's full. `buildfs` fails loudly if the set overflows.

**Color, if a freshly flashed image looks wrong.** Green↔blue swap = the RGB565
double byte-swap bug; negative colors = panel inversion. Both are display-config, not
image-prep — see [[ESP32-S3-USB-OTG-DOCS]] and the source
`docs/ESP32-S3-USB-OTG-NOTES.md`.

---

## Quick checklist — "my new image isn't showing"

1. Did you run `uploadfs` after changing `data/images/`? (firmware upload alone won't do it)
2. Is the file actually in `data/images/` and a supported type?
3. Is it 240×240? (ran through `prepare_images.py`?)
4. Serial on COM12 @ 115200 — does the boot log list the file under `/images`?
5. Mount fail (err 261) → LittleFS label is `littlefs`, not `spiffs`.

---
> Part of the OTG image-viewer project (`Desktop/funnymemeESp32`). See also
> [[ESP32-S3-USB-OTG-DOCS]] · [[inventory]] · [[notes]] · [[index]].
