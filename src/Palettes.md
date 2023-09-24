
# Palettes

## LCD Monochrome Palettes

### FF47 — BGP (Non-CGB Mode only): BG palette data

This register assigns gray shades to the color indexes of the BG and
Window tiles.

```
Bit 7-6 - Color for index 3
Bit 5-4 - Color for index 2
Bit 3-2 - Color for index 1
Bit 1-0 - Color for index 0
```

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
They work exactly like BGP, except that the lower two bits are ignored because color index 0 is transparent for OBJs.

## LCD Color Palettes (CGB only)

The CGB has a small amount of RAM used to store its color palettes. Unlike most
of the hardware interface, palette RAM (or *CRAM* for *Color RAM*) is not
accessed directly, but instead through the following registers:

### FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index

This register is used to address a byte in the CGB's background palette RAM.
Since there are 8 palettes, 8 palettes × 4 colors/palette × 2 bytes/color = 64 bytes
can be addressed.

```
Bit 7     Auto Increment  (0=Disabled, 1=Increment after Writing)
Bit 5-0   Address ($00-3F)
```

First comes BGP0 color number 0, then BGP0 color number 1, BGP0 color number 2, BGP0 color number 3,
BGP1 color number 0, and so on. Thus, address $03 allows accessing the second (upper)
byte of BGP0 color #1 via BCPD, which contains the color's blue and upper green bits.

data can be read from or written to the specified CRAM address through
BCPD/BGPD. If the Auto Increment bit is set, the index gets
incremented after each **write** to BCPD. Auto Increment has
no effect when **reading** from BCPD, so the index must be manually
incremented in that case. Writing to BCPD during rendering still causes
auto-increment to occur, despite the write being blocked.

Unlike BCPD, this register can be accessed outside VBlank and HBlank.

### FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data

This register allows to read/write data to the CGBs background palette memory,
addressed through BCPS/BGPI. Each color is stored as little-endian RGB555:

```
Bit 0-4   Red Intensity   ($00-1F)
Bit 5-9   Green Intensity ($00-1F)
Bit 10-14 Blue Intensity  ($00-1F)
```

Much like VRAM, data in palette memory cannot be read or written during the time
when the PPU is reading from it, that is, [Mode 3](<#PPU modes>).

::: tip NOTE

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

::: tip NOTE

The boot ROM leaves all object colors uninitialized (and thus somewhat random),
aside from setting the first byte of OBJ0 color #0 to $00, which is unused.

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

::: tip WORKAROUND

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
