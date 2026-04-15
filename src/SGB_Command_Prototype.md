# Removed Commands

These commands do nothing on all retail SGB revisions.
The following descriptions only apply to some prototype units.

## SGB Command $18 — OBJ_TRN

Used to start transferring object attributes to SNES object attribute memory (OAM). Unlike all other
functions with names ending in "\_TRN", this function does not use the usual
one-time 4 KiB VRAM transfer method. Instead, when enabled (below
execute bit set to 1), data is continuously (each frame) read out from the
lower character line of the Game Boy screen. To suppress garbage on the
display, the lower line is masked, and only the upper 20×17 characters
of the Game Boy screen are used - the masking method is unknown - frozen,
black, or recommended to be covered by the SGB border, or else ??? Also,
when the function is enabled, attract mode (built-in borders' screen saver on idle) is not performed.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1     Control Bits
         Bit 0   - SNES OBJ Mode enable (0=Cancel, 1=Enable)
         Bit 1   - Change OBJ Color     (0=No, 1=Use definitions below)
         Bit 2-7 - Not used (zero)
 2-3   System Color Palette Number for OBJ Palette 4 (0-511)
 4-5   System Color Palette Number for OBJ Palette 5 (0-511)
 6-7   System Color Palette Number for OBJ Palette 6 (0-511)
 8-9   System Color Palette Number for OBJ Palette 7 (0-511)
         These color entries are ignored if above Control Bit 1 is zero.
         Because each OBJ palette consists of 16 colors, four system
         palette entries (of 4 colors each) are transferred into each
         OBJ palette. The system palette numbers are not required to be
         aligned to a multiple of four, and will wrap to palette number
         0 when exceeding 511. For example, a value of 511 would copy
         system palettes 511, 0, 1, 2 to the SNES OBJ palette.
 A-F   Not used (zero)
```

The recommended method is to "display" Game Boy BG tiles $F9..$FF from
left to right as first 7 characters of the bottom-most character line of
the Game Boy screen. As for normal 4 KiB VRAM transfers, this area
should not be scrolled, should not be overlapped by Game Boy objects, and
the Game Boy BGP palette register should be set up properly. By following
that method, SNES OAM data can be defined in the $70 bytes of the
Game Boy BG tile memory at following addresses:

```
 8F90-8FEF  SNES OAM, 24 Entries of 4 bytes each (96 bytes)
 8FF0-8FF5  SNES OAM MSBs, 24 Entries of 2 bits each (6 bytes)
 8FF6-8FFF  Not used, don't care (10 bytes)
```

The format of SNES OAM entries is that of the SNES PPU, as described in
the [OAM section of Fullsnes](https://problemkaputt.de/fullsnes.htm#snesppuspritesobjs).
Notice that X and Y are swapped compared to GB PPU OAM entries,
and byte 3 is shifted left by 1 bit compared to GB and GBC OAM.

```
  Byte 0  OBJ X-Position (0-511, MSB is separately stored, see below)
  Byte 1  OBJ Y-Position (0-255)
  Byte 2  Tile Number
  Byte 3  Attributes
    Bit 7    Y-Flip (0=Normal, 1=Mirror Vertically)
    Bit 6    X-Flip (0=Normal, 1=Mirror Horizontally)
    Bit 5-4  Priority relative to BG (use only 3 on SGB)
    Bit 3-1  Palette Number (4-7)
    Bit 0    Tile Page (use only 0 on SGB)
```

The format of SNES OAM MSB Entries packs 2 bits for each of 4 objects
into one byte.

```
  Bit7    OBJ 3 OBJ Size     (0=Small, 1=Large)
  Bit6    OBJ 3 X-Coordinate (upper 1bit)
  Bit5    OBJ 2 OBJ Size     (0=Small, 1=Large)
  Bit4    OBJ 2 X-Coordinate (upper 1bit)
  Bit3    OBJ 1 OBJ Size     (0=Small, 1=Large)
  Bit2    OBJ 1 X-Coordinate (upper 1bit)
  Bit1    OBJ 0 OBJ Size     (0=Small, 1=Large)
  Bit0    OBJ 0 X-Coordinate (upper 1bit)
```

## SGB Command $0D — TEST_EN

Used to enable/disable test mode for "SGB-CPU variable clock speed
function". This function is disabled by default.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Test Mode Enable    (0=Disable, 1=Enable)
 2-F   Not used (zero)
```

Maybe intended to determine whether SNES operates at 50Hz or 60Hz
display refresh rate ??? Possibly result can be read-out from joypad
register ???
