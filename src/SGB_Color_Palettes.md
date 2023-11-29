# Color Palettes Overview

## Available SNES Palettes

The SGB/SNES provides 8 palettes of 16 colors each, each color may be
defined out of a selection of 32768 colors (15 bit). Palettes 0-3 are
used to colorize the gamescreen, only the first four colors of each of
these palettes are used. Palettes 4-7 are used for the SGB Border, all
16 colors of each of these palettes may be used.

## Color format

Colors are encoded as 16-bit RGB numbers, in the following way:

{{#bits 16 >
  "" 15:"Ignored" 14-10:"Blue" 9-5:"Green" 4-0:"Red"
}}

The palettes are encoded **little-endian**, thus, the Red+Green byte comes
first in memory.

:::tip

This is the same format as [Game Boy Color palettes](<#LCD Color Palettes (CGB only)>).
However, the same color will be displayed differently by SGB and CGB due to the different screen gamma!

:::

Here's a formula to convert 24-bit RGB into SNES format:
`(color & 0xF8) << 7 | (color & 0xF800) >> 6 | (color & 0xF80000) >> 19`

## Color 0 Restriction

Color 0 of each of the eight palettes is transparent, causing the
backdrop color to be displayed instead. The backdrop color is typically
defined by the most recently color being assigned to Color 0 (regardless
of the palette number being used for that operation). Effectively,
gamescreen palettes can have only three custom colors each, and SGB
border palettes only 15 colors each, additionally, color 0 can be used
for for all palettes, which will then all share the same color though.

## Translation of Grayshades into Colors

Because the SGB/SNES reads out the Game Boy video controllers display
signal, it translates the different grayshades from the signal into SNES
colors as such:

GB color   | SNES palette **index**
-----------|-----------------------
White      | Color #0
Light gray | Color #1
Dark gray  | Color #2
Black      | Color #3

Note that Game Boy colors 0-3 are assigned to user-selectable grayshades
by the Game Boy's BGP, OBP0, and OBP1 registers. There is thus no fixed
relationship between Game Boy colors 0-3 and SNES colors 0-3.

### Using Game Boy BGP/OBP Registers

A direct translation of GB color 0-3 into SNES color 0-3 may be produced
by setting BGP/OBPx registers to a value of $E4 each. However, in case
that your program uses black background for example, then you may
internally assign background as "White" at the Game Boy side by BGP/OBP
registers (which is then interpreted as SNES color 0, which is shared
for all SNES palettes). The advantage is that you may define Color 0 as
Black at the SNES side, and may assign custom colors for Colors 1-3 of
each SNES palette.

## System Color Palette Memory

Beside for the actually visible palettes, up to 512 palettes of 4 colors
each may be defined in SNES RAM. The palettes are just stored in RAM
without any relationship to the displayed picture; however, these
pre-defined colors may be transferred to actually visible palettes
slightly faster than when transferring palette data by separate command
packets.
