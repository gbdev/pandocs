# Rendering overview

::: tip TERMINOLOGY

All references to a "dot" are meant as dots (one 4 MiHz time unit).
Dots remain the same regardless of whether the CPU is in [double speed](<#FF4D — KEY1 (CGB Mode only): Prepare speed switch>).

:::

<figure><figcaption>

The following diagram shows how a Game Boy frame is decomposed:

</figcaption>

{{#include imgs/ppu_modes_timing.svg:2:}}

</figure>

TODO: high-level description of the above... and implications ("raster effects")

## PPU modes

The PPU operates on a 2<sup>22</sup> Hz = 4.194 MHz clock, called the "dot clock".
An entire frame is 154 scanlines = 70224 dots = 16.74 ms. On scanlines 0
through 143, the PPU cycles through modes 2, 3, and 0 once
every 456 dots. Scanlines 144 through 153 are mode 1.

While the PPU is accessing some video-related memory, [that memory is inaccessible to the CPU](<#Accessing VRAM and OAM>) (writes are ignored, and reads return garbage values, usually $FF).

Mode | Action                                     | Duration                             | Accessible video memory
-----|--------------------------------------------|--------------------------------------|-------------------------
  2  | Searching for OBJs which overlap this line | 80 dots                              | VRAM, CGB palettes
  3  | Sending pixels to the LCD                  | Between 172 and 289 dots, see below  | None
  0  | Waiting until the end of the scanline      | 376 - mode 3's duration              | VRAM, OAM, CGB palettes
  1  | Waiting until the next frame               | 4560 dots (10 scanlines)             | VRAM, OAM, CGB palettes

## Mode 3 length

Unlike most game consoles, the Game Boy does not always output pixels steadily[^crt]: some features cause the rendering process to stall for a couple dots.
Any extra time spent stalling *lengthens* Mode 3; but since scanlines last for a fixed number of dots, Mode 0 is therefore shortened by that same amount of time.

Three things can cause Mode 3 "penalties":

- **Background scrolling**: At the very beginning of Mode 3, rendering is paused for [`SCX`](<#FF42–FF43 — SCY, SCX: Viewport Y position, X position>) % 8 dots while the same number of pixels are discarded from the leftmost tile.
- **Window**: After the last non-window pixel is emitted, a 6-dot penalty is incurred while the BG fetcher is being set up for the window.
- **Objects**: Each object drawn during the scanline (even partially) incurs a 6- to 11-dot penalty ([see below](<#OBJ penalty algorithm>)).

On DMG and GBC in DMG mode, mid-scanline writes to [`BGP`](<#FF47 — BGP (Non-CGB Mode only): BG palette data>)
allow observing this behavior, as the delay from drawing an OBJ shifts the
write's effect to the left by that many dots.

### OBJ penalty algorithm

Only the OBJ's leftmost pixel matters here, transparent or not; it is designated as "The Pixel" in the following.

1. Determine the tile (background or window) that The Pixel is within. (This is affected by horizontal scrolling and/or the window!)
2. If that tile has **not** been considered by a previous OBJ yet:
   1. Count how many of that tile's pixels are to the right of The Pixel.
   2. Subtract 3.
   3. Incur this many dots of penalty, or zero if negative (from waiting for the BG fetch to finish).
3. Incur a flat, 6-dot penalty (from fetching the OBJ's tile).

**Exception**: an OBJ with an OAM X position of 0 (thus, completely off the left side of the screen) always incurs a 11-cycle penalty, regardless of `SCX`.

TODO: a diagram of some examples would probably help this be much clearer! \>_\<

[^crt]: The Game Boy can afford to "take pauses", because it writes to a LCD it fully controls; by contrast, home consoles like the NES or SNES are on a schedule imposed by the screen they are hooked up to. Taking pauses arguably simplified the PPU's design while allowing greater flexibility to game developers.
