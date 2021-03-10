
# VRAM Tile Maps

The Game Boy contains two 32x32 tile maps in VRAM at
the memory areas `$9800-$9BFF` and `$9C00-$9FFF`. Any of these maps can be used to
display the Background or the Window.

## Tile Indexes

Each tile map contains the 1-byte indexes of the
tiles to be displayed.

Tiles are obtained from the Tile Data Table using either of the two
addressing modes (described in [VRAM Tile Data](<#VRAM Tile Data>)), which
can be selected via [the LCDC register](<#FF40 - LCDC (LCD Control) (R/W)>).

Since one tile has 8x8 pixels, each map holds a 256x256 pixels picture.
Only 160x144 of those pixels are displayed on the LCD at any given time.

## BG Map Attributes (CGB Mode only)

In CGB Mode, an additional map of 32x32 bytes is stored in VRAM Bank 1
(each byte defines attributes for the corresponding tile-number map
entry in VRAM Bank 0, that is, 1:9800 defines the attributes for the tile at
0:9800):

```
Bit 7    BG-to-OAM Priority         (0=Use OAM Priority bit, 1=BG Priority)
Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)
Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)
Bit 4    Not used
Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
Bit 2-0  Background Palette number  (BGP0-7)
```

When Bit 7 is set, the corresponding BG tile will have priority above
all OBJs (regardless of the priority bits in OAM memory). There's also
a Master Priority flag in LCDC register Bit 0 which overrides all other
priority bits when cleared.

Note that, if the map entry at `0:9800` is tile \$2A, the attribute at
`1:9800` doesn't define properties for ALL tiles \$2A on-screen, but only
the one at `0:9800`!

## Background (BG)

The [SCY and SCX](<#FF42 - SCY (Scroll Y) (R/W), FF43 - SCX (Scroll X) (R/W)>)
registers can be used to scroll the Background, specifying the origin of the visible
160x144 pixel area within the total 256x256 pixel Background map.
The visible area of the Background wraps around the Background map (that is, when part of
the visible area goes beyond the map edge, it starts displaying the opposite side of the map).

In Non-CGB mode, the Background (and the Window) can be disabled using
[LCDC bit 0](<#LCDC.0 - BG and Window enable/priority>).

## Window

Besides the Background, there is also a Window overlaying it.
The content of the Window is not scrollable; it is always
displayed starting at the top left tile of its tile map. The only way to adjust the Window
is by modifying its position on the screen, which is done via the WX and WY registers. The screen
coordinates of the top left corner of the Window are (WX-7,WY). The tiles
for the Window are stored in the Tile Data Table. Both the Background
and the Window share the same Tile Data Table.

Whether the Window is displayed can be toggled using
[LCDC bit 5](<#LCDC.5 - Window enable>). But in Non-CGB mode this bit is only
functional as long as [LCDC bit 0](<#LCDC.0 - BG and Window enable/priority>) is set.
Enabling the Window makes
[Mode 3](<#LCD Status Register>) slightly longer on scanlines where it's visible.
(See [WX and WY](<#FF4A - WY (Window Y Position) (R/W), FF4B - WX (Window X Position + 7) (R/W)>)
for the definition of "Window visibility".)

::: tip Window Internal Line Counter

The window keeps an internal line counter that's functionally similar to `LY`, and increments alongside it. However, it only gets incremented when the window is visible, as described [here](<#FF4A - WY (Window Y Position) (R/W), FF4B - WX (Window X Position + 7) (R/W)>).

This line counter determines what window line is to be rendered on the current scanline.

:::
