
# VRAM Background Maps

The Game Boy contains two 32x32 tile maps in VRAM at
addresses `$9800-$9BFF` and `$9C00-$9FFF`. They are known as background tile maps. Any of these maps can be used to
display the Background or the Window.

## BG Map Tile Indexes

Each background tile map contains the 1-byte indexes of the
tiles to be displayed.

Tiles are obtained from the Tile Data Table using either of the two
addressing modes (described [above](<#VRAM Tile Data>)), which
can be selected via LCDC register.

As one background tile has a size of 8x8 pixels, the BG maps may hold a
picture of 256x256 pixels, and an area of 160x144 pixels of this picture
can be displayed on the LCD screen.

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

The [SCY and SCX](<#FF42 - SCY (Scroll Y) (R/W), FF43 - SCX (Scroll X) (R/W)>) registers can be
used to scroll the background, allowing to select the origin of the visible
160x144 pixel area within the total 256x256 pixel background map.
The Background visible area wraps around the Background map (that is, when part of
the visible area goes beyond the map edge, it starts displaying the opposite side of the map).

Whether the background is displayed can be toggled using
[LCDC bit 0](<#LCDC.0 - BG and Window enable/priority>), except on CGB in CGB Mode,
where it's always drawn.

## Window

Besides the Background, there is also a Window overlaying it.
The content of the Window is not scrollable; it is always
displayed starting at the top left tile of its tilemap. The only way to adjust the Window
is by modifying its position on the screen, which is done via the WX and WY registers. The screen
coordinates of the top left corner of the Window are (WX-7,WY). The tiles
for the Window are stored in the Tile Data Table. Both the Background
and the Window share the same Tile Data Table.

Whether the Window is displayed can be toggled using
[LCDC bit 5](<#LCDC.5 - Window enable>). Enabling the Window makes
[Mode 3](#<LCD Status Register>) slightly longer on scanlines where it's visible.
(See [above](<#FF4A - WY (Window Y Position) (R/W), FF4B - WX (Window X Position + 7) (R/W)>)
for the definition of "Window visibility".)

::: tip Window Internal Line Counter

The window keeps an internal line counter that's functionally similar to `LY`, and increments alongside it. However, it only gets incremented when the window is visible, as described [here](<#FF4A - WY (Window Y Position) (R/W), FF4B - WX (Window X Position + 7) (R/W)>).

This line counter determines what window line is to be rendered on the current scanline.

:::
