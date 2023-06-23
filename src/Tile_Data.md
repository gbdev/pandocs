
# VRAM Tile Data

Tile data is stored in VRAM in the memory area at \$8000-$97FF; with each tile
taking 16 bytes, this area defines data for 384 tiles. In CGB Mode,
this is doubled (768 tiles) because of the two VRAM banks.

Each tile (or character) has 8×8 pixels and has a color depth of
2 bits per pixel, allowing each pixel to use one of 4 colors or gray
shades. Tiles can be displayed as part of the Background/Window maps,
and/or as objects (movable sprites).  Color 0 has a special meaning
in objects - it's transparent, allowing the background or other
objects behind it to show through.

There are three "blocks" of 128 tiles each:

<table>
  <thead>
    <tr>
      <th rowspan="2">Block</th>
      <th rowspan="2">VRAM Address</th>
      <th colspan="3">Corresponding Tile IDs</th>
    </tr>
    <tr>
      <td>Objects</td>
      <td>BG/Win if LCDC.4=1</td>
      <td>BG/Win if LCDC.4=0</td>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>0</td>
      <td>$8000&ndash;$87FF</td>
      <td>0&ndash;127</td>
      <td>0&ndash;127</td>
      <td></td>
    </tr>
    <tr>
      <td>1</td>
      <td>$8800&ndash;$8FFF</td>
      <td>128&ndash;255</td>
      <td>128&ndash;255</td>
      <td>
        128&ndash;255 <br />
        (or -128&ndash;-1)
      </td>
    </tr>
    <tr>
      <td>2</td>
      <td>$9000&ndash;$97FF</td>
      <td colspan="2">(Can't use)</td>
      <td>0&ndash;127</td>
    </tr>
  </tbody>
</table>


Tiles are always indexed using an 8-bit integer, but the addressing
method may differ. The "$8000 method" uses \$8000 as its base pointer
and uses an unsigned addressing, meaning that tiles 0-127 are in block
0, and tiles 128-255 are in block 1. The "$8800 method" uses \$9000 as
its base pointer and uses a signed addressing, meaning that tiles 0-127
are in block 2, and tiles -128 to -1 are in block 1, or to put it differently,
"$8800 addressing" takes tiles 0-127 from block 2
and tiles 128-255 from block 1. (You can notice that block 1 is shared
by both addressing methods)

Objects always use "$8000 addressing", but the BG and Window can use either
mode, controlled by [LCDC bit 4](<#LCDC.4 — BG and Window tile data area>).

Each tile occupies 16 bytes, where each line is represented by 2 bytes:

```
Byte 0-1  Topmost Line (Top 8 pixels)
Byte 2-3  Second Line
etc.
```

For each line, the first byte specifies the least significant bit of the color
ID of each pixel, and the second byte specifies the most significant bit. In
both bytes, bit 7 represents the leftmost pixel, and bit 0 the rightmost. For
example, the tile data `$3C $7E $42 $42 $42 $42 $42 $42 $7E $5E $7E $0A $7C $56
$38 $7C` appears as follows:

<figure>
{{#include imgs/sprite.svg:2:}}
<figcaption>Sample tile data</figcaption>
</figure>

For the first row, the values `$3C $7E` are `00111100` and `01111110` in
binary. The leftmost bits are 0 and 0, thus the color ID is binary `00`, or 0.
The next bits are 0 and 1, thus the color ID is binary `10`, or 2 (remember to
flip the order of the bits!). The full eight-pixel row evaluates to 0 2 3 3 3 3
2 0.

A tool for viewing tiles can be found
[here](https://www.huderlem.com/demos/gameboy2bpp.html).

So, each pixel has a color ID of 0 to 3. The color
numbers are translated into real colors (or gray shades) depending on
the current palettes, except that when the tile is used in an OBJ the
color ID 0 means transparent. The palettes are defined through registers
[BGP](<#FF47 — BGP (Non-CGB Mode only): BG palette data>),
[OBP0 and OBP1](<#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>), and
[BCPS/BGPI](<#FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index>),
[BCPD/BGPD](<#FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data>),
[OCPS/OBPI and OCPD/OBPD](<#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>)
(CGB Mode).
