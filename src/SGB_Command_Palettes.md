# Palette Commands

## SGB Command $00 — PAL01

Transmit color data for SGB palette 0, color 0-3, and for SGB palette 1,
color 1-3 (without separate color 0).

```
 Byte  Content
 0     Command*8+Length (fixed length=$01)
 1-E   Color Data for 7 colors of 2 bytes (16 bits) each:
         Bit 0-4   - Red Intensity   (0-31)
         Bit 5-9   - Green Intensity (0-31)
         Bit 10-14 - Blue Intensity  (0-31)
         Bit 15    - Not used (zero)
 F     Not used ($00)
```

This is the same RGB5 format as [Game Boy Color palette
entry](<#LCD Color Palettes (CGB only)>), though
without the LCD correction. The value transferred as color 0 will be
applied for all four palettes.

## SGB Command $01 — PAL23

Same as above PAL01, but for Palettes 2 and 3 respectively.

## SGB Command $02 — PAL03

Same as above PAL01, but for Palettes 0 and 3 respectively.

## SGB Command $03 — PAL12

Same as above PAL01, but for Palettes 1 and 2 respectively.

## SGB Command $0A — PAL_SET

Used to copy pre-defined palette data from SGB system color palettes to
actual SNES palettes.

Note: all palette numbers are little-endian.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1-2   System Palette number for SGB Color Palette 0 (0-511)
 3-4   System Palette number for SGB Color Palette 1 (0-511)
 5-6   System Palette number for SGB Color Palette 2 (0-511)
 7-8   System Palette number for SGB Color Palette 3 (0-511)
 9     Attribute File
         Bit 0-5 - Attribute File Number ($00-$2C) (Used only if Bit7=1)
         Bit 6   - Cancel Mask           (0=No change, 1=Yes)
         Bit 7   - Use Attribute File    (0=No, 1=Apply above ATF Number)
 A-F   Not used (zero)
```

Before using this function, System Palette data should be initialized by
PAL_TRN command, and (when used) Attribute File data should be
initialized by ATTR_TRN.

## SGB Command $0B — PAL_TRN

Used to initialize SGB system color palettes in SNES RAM. System color
palette memory contains 512 pre-defined palettes, these palettes do not
directly affect the display, however, the PAL_SET command may be later
used to transfer four of these "logical" palettes to actual visible
"physical" SGB palettes. Also, the OBJ_TRN function will use groups
of 4 System Color Palettes (4\*4 colors) for SNES OBJ palettes (16
colors).

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1-F   Not used (zero)
```

The palette data is sent by VRAM-Transfer (4 KBytes).

```
 000-FFF  Data for System Color Palette 0-511
```

Each Palette consists of four 16-bit color definitions (8 bytes). Note:
The data is stored at 3000-3FFF in SNES memory.

## SGB Command $19 — PAL_PRI

If the player overrides the active palette set (a pre-defined or the custom one), it stays in effect until the smiley face is selected again, or the player presses the X button on their SNES controller.

However, if `PAL_PRI` is enabled, then changing the palette set (via any of the above commands except `PAL_TRN`) will switch back to the game's newly-modified palette set, if it wasn't already active.

_Donkey Kong_ (1994) is one known game that appears to use this.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1     Palette priority when a palette packet is sent
         Bit 0 - Priority (0=User, 1=Software)
 2-F   Not used (zero)
```
