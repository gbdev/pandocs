# LCD Status Register

::: tip TERMINOLOGY

A *dot* is the shortest period over which the PPU can output one pixel: is it equivalent to 1 T-state on DMG or on CGB single-speed mode or 2 T-states on CGB double-speed mode. On each dot during mode 3, either the PPU outputs a pixel or the fetcher is stalling the [FIFOs](<#Pixel FIFO>).

:::

## FF41 - STAT (LCD Status) (R/W)

```
Bit 6 - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
Bit 1-0 - Mode Flag                          (Mode 0-3, see below) (Read Only)
          0: In HBlank
          1: In VBlank
          2: Searching OAM
          3: Transferring Data to LCD Controller
```

The two lower STAT bits show the current status of the PPU.

Bit 2 is set when [LY](<#FF44 - LY (LCD Y Coordinate) (R)>) contains the same value as [LYC](<#FF45 - LYC (LY Compare) (R/W)>).
It is constantly updated.

Bits 3-6 select which sources are used for [the STAT interrupt](<#INT 48 - STAT Interrupt>).

The LCD controller operates on a 2^22 Hz = 4.194 MHz dot clock. An
entire frame is 154 scanlines, 70224 dots, or 16.74 ms. On scanlines 0
through 143, the PPU cycles through modes 2, 3, and 0 once
every 456 dots. Scanlines 144 through 153 are mode 1.

The following sequence is typical when the display is enabled:

```
Mode 2  2_____2_____2_____2_____2_____2___________________2____
Mode 3  _33____33____33____33____33____33__________________3___
Mode 0  ___000___000___000___000___000___000________________000
Mode 1  ____________________________________11111111111111_____
```

When the PPU is reading a particular part of video memory,
that memory is inaccessible to the CPU.

- During modes 2 and 3, the CPU cannot access OAM (FE00h-FE9Fh).
- During mode 3, the CPU cannot access VRAM or CGB Palette Data
  (FF69,FF6B).

Mode | Action                                                           | Duration                                                           | Accessible video memory
-----|------------------------------------------------------------------|--------------------------------------------------------------------|-------------------------
  2  | Searching OAM for OBJs whose (X,Y) coordinates overlap this line | 80 dots (19 µs)                                                    | VRAM, CGB palettes
  3  | Reading OAM and VRAM to generate the picture                     | 168 to 291 dots (40 to 60 µs) depending on sprite count            | None
  0  | Horizontal blanking                                              | 85 to 208 dots (20 to 49 µs) depending on previous mode 3 duration | VRAM, OAM, CGB palettes
  1  | Vertical blanking                                                | 4560 dots (1087 µs, 10 scanlines)                                  | VRAM, OAM, CGB palettes

## Properties of STAT modes

Unlike most game consoles, the Game Boy can pause the dot clock briefly,
adding dots to mode 3's duration. It routinely takes a 6 to 11 dot
break to fetch sprite patterns between background tile pattern fetches.
On DMG and GBC in DMG mode, mid-scanline writes to [`BGP`](<#FF47 - BGP (BG Palette Data) (R/W) - Non CGB Mode Only>)
allow observing this behavior, as a sprite delay shifts the effect of a
write to the left by that many dots.

Three things are known to pause the dot clock:

- Background scrolling: If `SCX mod 8` is not zero at the start of the scanline, rendering is paused for that many dots while the shifter discards that many pixels from the leftmost tile.
- Window: An active window pauses for at least 6 dots, as the background fetching mechanism starts over at the left side of the window.
- Sprites: Each sprite usually pauses for `11 - min(5, (x + SCX) mod 8)` dots. Because sprite fetch waits for background fetch to finish, a sprite's cost depends on its position relative to the left side of the background tile under it. It's greater if a sprite is directly aligned over the background tile, less if the sprite is to the right. If the sprite's left side is over the window, use `255 - WX` for `SCX` in this formula.

::: warning TO BE VERIFIED

The exact pause duration for window start is
not confirmed; it may have the same background fetch finish delay as a
sprite. If two sprites' left sides are over the same background or
window tile, the second may pause for fewer dots.

:::

A hardware quirk in the monochrome Game Boy makes the LCD interrupt
sometimes trigger when writing to STAT (including writing \$00) during
OAM scan, HBlank, VBlank, or LY=LYC. It behaves as if \$FF were
written for one cycle, and then the written value were written the next
cycle. Because the GBC in DMG mode does not have this quirk, two games
that depend on this quirk (Ocean's *Road Rash* and Vic Tokai's *Xerd
no Densetsu*) will not run on a GBC.
