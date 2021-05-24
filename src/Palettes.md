
# Palettes

## LCD Monochrome Palettes

### FF47 - BGP (BG Palette Data) (R/W) - Non CGB Mode Only

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

In CGB Mode the Color Palettes are taken from [CGB Palette Memory](<#LCD Color Palettes (CGB only)>)
instead.

### FF48 - OBP0 (Object Palette 0 Data) (R/W) - Non CGB Mode Only

This register assigns gray shades to the color indexes of the OBJs that use this palette. It works exactly
like BGP (FF47), except that the lower two bits are ignored because
sprite index 00 means transparent.

### FF49 - OBP1 (Object Palette 1 Data) (R/W) - Non CGB Mode Only

This register assigns gray shades to the color indexes of the OBJs that use this palette. It works exactly
like BGP (FF47), except that the lower two bits are ignored because
sprite index 00 means transparent.

## LCD Color Palettes (CGB only)

### FF68 - BCPS/BGPI (Background Color Palette Specification or Background Palette Index) - CGB Mode Only

This register is used to address a byte in the CGBs Background Palette
Memory. Each two byte in that memory define a color value. The first 8
bytes define Color 0-3 of Palette 0 (BGP0), and so on for BGP1-7.

```
Bit 7     Auto Increment  (0=Disabled, 1=Increment after Writing)
Bit 5-0   Index (00-3F)
```

Data can be read/written to/from the specified index address through
Register FF69. When the Auto Increment bit is set then the index is
automatically incremented after each **write** to FF69. Auto Increment has
no effect when **reading** from FF69, so the index must be manually
incremented in that case. Writing to FF69 during rendering still causes
auto-increment to occur.

Unlike the following, this register can be accessed outside VBlank and
HBlank.

### FF69 - BCPD/BGPD (Background Color Palette Data or Background Palette Data) - CGB Mode Only

This register allows to read/write data to the CGBs Background Palette
Memory, addressed through Register FF68. Each color is defined by two
bytes (Bit 0-7 in first byte).

```
Bit 0-4   Red Intensity   (00-1F)
Bit 5-9   Green Intensity (00-1F)
Bit 10-14 Blue Intensity  (00-1F)
```

Much like VRAM, data in Palette Memory cannot be read/written during the
time when the LCD Controller is reading from it. (That is when the STAT
register indicates Mode 3). Note: All background colors are initialized
as white by the boot ROM, but it's a good idea to initialize at least
one color yourself (for example if you include a soft-reset mechanic).

### FF6A - OCPS/OBPI (Object Color Palette Specification or Sprite Palette Index), FF6B - OCPD/OBPD (Object Color Palette Data or Sprite Palette Data) - Both CGB Mode Only

These registers are used to initialize the Sprite Palettes OBP0-7,
identically as described above for Background Palettes. Note that four
colors may be defined for each OBP Palettes - but only Color 1-3 of each
Sprite Palette can be displayed, Color 0 is always transparent, and can
be initialized to a don't care value or plain never initialized.

Note: All sprite colors are left uninitialized by the boot ROM, and are
somewhat random.

### RGB Translation by CGBs

![sRGB versus CGB color mixing](imgs/VGA_versus_CGB.png)

When developing graphics on PCs, note that the RGB values will have
different appearance on CGB displays as on VGA/HDMI monitors calibrated
to sRGB color. Because the GBC is not lit, the highest intensity will
produce Light Gray color rather than White. The intensities are not
linear; the values 10h-1Fh will all appear very bright, while medium and
darker colors are ranged at 00h-0Fh.

The CGB display's pigments aren't perfectly saturated. This means the
colors mix quite oddly; increasing intensity of only one R,G,B color
will also influence the other two R,G,B colors. For example, a color
setting of 03EFh (Blue=0, Green=1Fh, Red=0Fh) will appear as Neon Green
on VGA displays, but on the CGB it'll produce a decently washed out
Yellow. See the image above.

### RGB Translation by GBAs

Even though GBA is described to be compatible to CGB games, most CGB
games are completely unplayable on older GBAs because most colors are
invisible (black). Of course, colors such like Black and White will
appear the same on both CGB and GBA, but medium intensities are arranged
completely different. Intensities in range 00h..07h are invisible/black
(unless eventually under best sunlight circumstances, and when gazing at
the screen under obscure viewing angles), unfortunately, these
intensities are regularly used by most existing CGB games for medium and
darker colors.

Newer CGB games may avoid this effect by changing palette data when
detecting GBA hardware ([see
how](<#Detecting CGB (and GBA) functions>)).
Based on measurement of GBC and GBA palettes using the [144p Test
Suite](https://github.com/pinobatch/240p-test-mini/tree/master/gameboy) ROM, a fairly close approximation is GBA = GBC \* 3/4 + 8h for
each R,G,B intensity. The result isn't quite perfect, and it may turn
out that the color mixing is different also; anyways, it'd be still
ways better than no conversion.

This problem with low brightness levels does not affect later GBA SP
units and Game Boy Player. Thus ideally, the player should have control
of this brightness correction.
