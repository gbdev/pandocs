
# VRAM Tile Maps

The Game Boy contains two 32×32 tile maps in VRAM at
the memory areas `$9800-$9BFF` and `$9C00-$9FFF`. Any of these maps can be used to
display the Background or the Window.

## Tile Indexes

Each tile map contains the 1-byte indexes of the
tiles to be displayed.

Tiles are obtained from the Tile Data Table using either of the two
addressing modes (described in [VRAM Tile Data](<#VRAM Tile Data>)), which
can be selected via [the LCDC register](<#FF40 — LCDC: LCD control>).

Since one tile has 8×8 pixels, each map holds a 256×256 pixels picture.
Only 160×144 of those pixels are displayed on the LCD at any given time.

## BG Map Attributes (CGB Mode only)

In CGB Mode, an additional map of 32×32 bytes is stored in VRAM Bank 1
(each byte defines attributes for the corresponding tile-number map
entry in VRAM Bank 0, that is, 1:9800 defines the attributes for the tile at
0:9800):

{{#bits 8 > 
	"BG attributes"  7:"Priority" 6:"Y flip" 5:"X flip" 3:"Bank" 2-0:"Color palette";
}}

- **Priority**: `0` = No; `1` = [Color indices](<#Data format>) 1–3 of the corresponding BG/Window tile are drawn over OBJ, regardless of [OBJ priority](<#Byte 3 — Attributes/Flags>)
- **Y flip**: `0` = Normal; `1` = Tile is drawn vertically mirrored
- **X flip**: `0` = Normal; `1` = Tile is drawn horizontally mirrored
- **Bank**: `0` = Fetch tile from VRAM bank 0; `1` = Fetch tile from VRAM bank 1
- **Color palette**: Which of BGP0–7 to use

Bit 4 is ignored by the hardware, but can be written to and read from normally.

Note that, for example, if the byte at `0:9800` is \$2A, the attribute at `1:9800` doesn't define properties for ALL tiles \$2A on-screen, but only the one at `0:9800`!

### BG-to-OBJ Priority in CGB Mode

In CGB Mode, the priority between the BG (and window) layer and the OBJ layer is declared in three different places:
- [BG Map Attribute bit 7](<#BG Map Attributes (CGB Mode only)>)
- [LCDC bit 0](<#LCDC.0 — BG and Window enable/priority>)
- [OAM Attributes bit 7](<#Byte 3 — Attributes/Flags>)

We can infer the following rules from the table below:
* If the BG color index is 0, the OBJ will always have priority;
* Otherwise, if LCDC bit 0 is clear, the OBJ will always have priority;
* Otherwise, if both the BG Attributes and the OAM Attributes have bit 7 clear, the OBJ will have priority;
* Otherwise, BG will have priority.

The following table shows the relations between the 3 flags:

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

:::warning

Keep in mind that:
* OAM Attributes bit 7 will grant OBJ priority when **clear**, not when **set**.
* Priority between all OBJs is resolved **before** priority with the BG layer is considered.
  Please refer [to this page](<#Drawing priority>) for more details.

:::


