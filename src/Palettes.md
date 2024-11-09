
# Palettes

## LCD Monochrome Palettes

### FF47 — BGP (Non-CGB Mode only): BG palette data

This register assigns gray shades to the [color IDs](./Tile_Data.md) of the BG and Window tiles.

{{#bits 8 >
  "Color for..."  7-6:"ID 3" 5-4:"ID 2" 3-2:"ID 1" 1-0:"ID 0";
}}

Each of the two-bit values map to a color thusly:

Value | Color
------|-------
  0   | White
  1   | Light gray
  2   | Dark gray
  3   | Black

In CGB Mode the color palettes are taken from [CGB palette memory](<#LCD Color Palettes (CGB only)>)
instead.

### FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data

These registers assigns gray shades to the color indexes of the OBJs that use the corresponding palette.
They work exactly like [`BGP`](<#FF47 — BGP (Non-CGB Mode only): BG palette data>), except that the lower two bits are ignored because color index 0 is transparent for OBJs.

## LCD Color Palettes (CGB only)

The CGB has a small amount of RAM used to store its color palettes. Unlike most
of the hardware interface, palette RAM (or *CRAM* for *Color RAM*) is not
accessed directly, but instead through the following registers:

### FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index

This register is used to address a byte in the CGB's background palette RAM.
Since there are 8 palettes, 8 palettes × 4 colors/palette × 2 bytes/color = 64 bytes
can be addressed.

First comes BGP0 color number 0, then BGP0 color number 1, BGP0 color number 2, BGP0 color number 3,
BGP1 color number 0, and so on. Thus, address $03 allows accessing the second (upper)
byte of BGP0 color #1 via BCPD, which contains the color's blue and upper green bits.

{{#bits 8 >
  "BCPS / OCPS"  7:"Auto-increment" 5-0:"Address";
}}

- **Auto-increment**: `0` = Disabled; `1` = Increment "Address" field after **writing** to
  [`BCPD`](<#FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data>) /
  [`OCPD`](<#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>)
  (even during [Mode 3](<#PPU modes>), despite the write itself failing), reads *never* cause an increment
- **Address**: Specifies which byte of BG Palette Memory can be accessed through
  [`BCPD`](<#FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data>)

Unlike BCPD, this register can be accessed outside VBlank and HBlank.

### FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data

This register allows to read/write data to the CGBs background palette memory, addressed through [BCPS/BGPI](<#FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index>).
Each color is stored as little-endian RGB555:

{{#bits 16 <
  "One color"  0-4:"Red intensity" 5-9:"Green intensity" 10-14:"Blue intensity";
}}

Much like VRAM, data in palette memory cannot be read or written during the time
when the PPU is reading from it, that is, [Mode 3](<#PPU modes>).

:::tip NOTE

All background colors are initialized as white by the boot ROM, however it is a
good idea to initialize all colors yourself, e.g. if implementing
a soft-reset mechanic.

:::

### FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data

These registers function exactly like BCPS and BCPD respectively; the 64 bytes
of OBJ palette memory are entirely separate from Background palette memory, but
function the same.

Note that while 4 colors are stored per OBJ palette, color #0 is never used, as
it's always transparent. It's thus fine to write garbage values, or even leave
color #0 uninitialized.

:::tip NOTE

In CGB mode, the boot ROM leaves all object colors uninitialized (and thus somewhat random/unreliable),
aside from setting the first byte of OBJ0 color #0 to $00, which is unused.

In DMG compatibility mode, the boot ROM sets the first 2 object palettes which are
used by OBP0/OBP1, [as explained here](<#Compatibility palettes>).

:::

### RGB Translation by CGBs

![sRGB versus CGB color mixing](imgs/VGA_versus_CGB.png)

When developing graphics on PCs, note that the RGB values will have
different appearance on CGB displays as on VGA/HDMI monitors calibrated
to sRGB color. Because the GBC is not lit, the highest intensity will
produce light gray rather than white. The intensities are not
linear; the values $10-$1F will all appear very bright, while medium and
darker colors are ranged at $00-0F.

The CGB display's pigments aren't perfectly saturated. This means the
colors mix quite oddly: increasing the intensity of only one R/G/B color
will also influence the other two R/G/B colors. For example, a color
setting of $03EF (Blue=$00, Green=$1F, Red=$0F) will appear as Neon Green
on VGA displays, but on the CGB it'll produce a decently washed out
Yellow. See the image above.

### RGB Translation by GBAs

Even though GBA is described to be compatible to CGB games, most CGB
games are completely unplayable on older GBAs because most colors are
invisible (black). Of course, colors such like Black and White will
appear the same on both CGB and GBA, but medium intensities are arranged
completely different. Intensities in range $00–07 are invisible/black
(unless eventually under best sunlight circumstances, and when gazing at
the screen under obscure viewing angles), unfortunately, these
intensities are regularly used by most existing CGB games for medium and
darker colors.

:::tip WORKAROUND

Newer CGB games may avoid this effect by changing palette data when
detecting GBA hardware ([see how](<#Detecting CGB (and GBA) functions>)).
Based on measurements of GBC and GBA palettes using the
[144p Test Suite](https://github.com/pinobatch/240p-test-mini/tree/master/gameboy),
a fairly close approximation is `GBA = GBC × 3/4 + $08` for each R/G/B
component. The result isn't quite perfect, and it may turn
out that the color mixing is different also; anyways, it'd be still
ways better than no conversion.

:::

This problem with low brightness levels does not affect later GBA SP
units and Game Boy Player. Thus ideally, the player should have control
of this brightness correction.
