
# VRAM Tile Maps

The Game Boy contains two 32x32 tile maps in VRAM at
the memory areas `$9800-$9BFF` and `$9C00-$9FFF`. Any of these maps can be used to
display the Background or the Window.

## Tile Indexes

Each tile map contains the 1-byte indexes of the
tiles to be displayed.

Tiles are obtained from the Tile Data Table using either of the two
addressing modes (described in [VRAM Tile Data](<#VRAM Tile Data>)), which
can be selected via [the LCDC register](<#FF40 — LCDC: LCD control>).

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

Note that, if the map entry at `0:9800` is tile \$2A, the attribute at
`1:9800` doesn't define properties for ALL tiles \$2A on-screen, but only
the one at `0:9800`!

### BG-to-OBJ Priority in CGB Mode

In CGB Mode, the priority between the BG (and window) layer and the OBJ layer is declared in three different places:
1. [BG Map Attribute bit 7](#bg-map-attributes-cgb-mode-only) - BG-to-OAM Priority (CGB Mode)
2. [LCDC bit 0](LCDC.md#lcdc0--bg-and-window-enablepriority) - BG and Window Priority (CGB Mode)
3. [OAM Attributes bit 7](OAM.md#byte-3--attributesflags) - BG and Window over OBJ

We can infer the following rules from the table below:
* When the BG color is 0 the OBJ will always have priority (ignoring the flags)
* When LCDC bit 0 is clear OBJ will always have priority (ignoring the rest of the flags)
* In order to grant the BG priority (color 1-3) LCDC bit 0 must be set and if OAM attributes bit 7 is clear BG Map attributes bit 7 must be set to override it

::: tip NOTE

OAM Attributes bit 7 will grant OBJ priority when clear and not when set

:::

The following table visualize the relations between the 3 flags

LCDC bit 0 | OAM attr bit 7 | BG attr bit 7 | Priority
:---------:|:--------------:|:-------------:|---------
0          | 0              | 0             | OBJ
0          | 0              | 1             | OBJ
0          | 1              | 0             | OBJ
0          | 1              | 1             | OBJ
1          | 0              | 0             | OBJ
1          | 0              | 1             | BG color 1–3, otherwise OBJ
1          | 1              | 0             | BG color 1–3, otherwise OBJ
1          | 1              | 1             | BG color 1–3, otherwise OBJ

[This test ROM](https://github.com/alloncm/MagenTests) can be used to observe the above.

## Background (BG)

The [SCY and SCX](<#FF42–FF43 — SCY, SCX: Viewport Y position, X position>)
registers can be used to scroll the Background, specifying the origin of the visible
160x144 pixel area within the total 256x256 pixel Background map.
The visible area of the Background wraps around the Background map (that is, when part of
the visible area goes beyond the map edge, it starts displaying the opposite side of the map).

In Non-CGB mode, the Background (and the Window) can be disabled using
[LCDC bit 0](<#LCDC.0 — BG and Window enable/priority>).

## Window

Besides the Background, there is also a Window overlaying it.
The content of the Window is not scrollable; it is always
displayed starting at the top left tile of its tile map. The only way to adjust the Window
is by modifying its position on the screen, which is done via the WX and WY registers. The screen
coordinates of the top left corner of the Window are (WX-7,WY). The tiles
for the Window are stored in the Tile Data Table. Both the Background
and the Window share the same Tile Data Table.

Whether the Window is displayed can be toggled using
[LCDC bit 5](<#LCDC.5 — Window enable>). But in Non-CGB mode this bit is only
functional as long as [LCDC bit 0](<#LCDC.0 — BG and Window enable/priority>) is set.
Enabling the Window makes
[Mode 3](<#LCD Status Register>) slightly longer on scanlines where it's visible.
(See [WX and WY](<#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>)
for the definition of "Window visibility".)

::: tip Window Internal Line Counter

The window keeps an internal line counter that's functionally similar to `LY`, and increments alongside it. However, it only gets incremented when the window is visible, as described [here](<#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>).

This line counter determines what window line is to be rendered on the current scanline.

:::
