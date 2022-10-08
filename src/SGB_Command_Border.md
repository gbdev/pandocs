# Border and OBJ Commands

## SGB Command $13 — CHR_TRN

Used to transfer tile data (characters) to SNES Tile memory in VRAM.
This normally used to define BG tiles for the SGB Border (see PCT_TRN),
but might be also used to define moveable SNES foreground sprites (see
OBJ_TRN).

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Tile Transfer Destination
         Bit 0   - Tile Numbers   (0=Tiles 00h-7Fh, 1=Tiles 80h-FFh)
         Bit 1   - Tile Type      (0=BG Tiles, 1=OBJ Tiles)
         Bit 2-7 - Not used (zero)
 2-F   Not used (zero)
```

The tile data is sent by VRAM-Transfer (4 KBytes).

```
 000-FFF  Bitmap data for 128 Tiles
```

Each tile occupies 32 bytes (8x8 pixels, 16 colors each). When intending
to transfer more than 128 tiles, call this function twice (once for
tiles 00h-7Fh, and once for tiles 80h-FFh). Note: The BG/OBJ Bit seems
to have no effect and writes to the same VRAM addresses for both BG and
OBJ ???

Each tile is stored in 4-bit-per-pixel format consisting of bit planes 0 and 1 interleaved by row, followed by bit planes 2 and 3 interleaved by row.
In effect, each tile consists of two Game Boy tiles, the first to determine bits 0 and 1 (choosing among color 0, 1, 2, or 3 within a 4-color subpalette), and the second to determine bits 2 and 3 (choosing among colors 0-3, 4-7, 8-11, or 12-15).

## SGB Command $14 — PCT_TRN

Used to transfer tile map data and palette data to SNES BG Map memory in
VRAM to be used for the SGB border. The actual tiles must be separately
transferred by using the CHR_TRN function.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1-F   Not used (zero)
```

The map data is sent by VRAM-Transfer (4 KBytes).

```
 000-6FF  BG Map 32x28 Entries of 16 bits each (1792 bytes)
 700-7FF  Not used, don't care
 800-85F  BG Palette Data (Palettes 4-6, each 16 colors of 16 bits each)
 860-FFF  Not used, don't care
```

Each BG Map Entry consists of a 16-bit value as such:
`VH01 PP00 NNNN NNNN```

```
 Bit 0-9   - Character Number (use only 00h-FFh, upper 2 bits zero)
 Bit 10-12 - Palette Number   (use only 4-6)
 Bit 13    - BG Priority      (use only 0)
 Bit 14    - X-Flip           (0=Normal, 1=Mirror horizontally)
 Bit 15    - Y-Flip           (0=Normal, 1=Mirror vertically)
```

The 32x28 map entries correspond to 256x224 pixels of the Super NES
screen. The 20x18 entries in the center of the 32x28 area should be set
to a blank (solid color 0) tile as transparent space for the Game Boy
window to be displayed inside. Non-transparent border data will cover
the Game Boy window (for example, *Mario's Picross* does this, as does
*WildSnake* to a lesser extent).

All borders repeat tiles. Assuming that the blank space for the GB
screen is a single tile, as is the letterbox in a widescreen border, a
border defining all unique tiles would have to define this many tiles:

-   (256\*224-160\*144)/64+1 = 537 tiles in fullscreen border
-   (256\*176-160\*144)/64+2 = 346 tiles in widescreen border

Because the CHR RAM allocated by SGB for border holds only 256 tiles, a full-screen border must repeat at least 281 tiles and a widescreen border at least 90.

The Super NES supports 8 background palettes.
The SGB system software (when run in a LLE such as Mesen-S) has been observed to use background palette 0 for the GB screen, palettes 1, 2, 3, and 7 for the menus, and palettes 4, 5, and 6 for the border.
Thus a border can use three 15-color palettes.

## SGB Command $18 — OBJ_TRN

Used to transfer OBJ attributes to SNES OAM memory. Unlike all other
functions with the ending \_TRN, this function does not use the usual
one-shot 4KBytes VRAM transfer method. Instead, when enabled (below
execute bit set), data is permanently (each frame) read out from the
lower character line of the Game Boy screen. To suppress garbage on the
display, the lower line is masked, and only the upper 20x17 characters
of the Game Boy window are used - the masking method is unknwon - frozen,
black, or recommended to be covered by the SGB border, or else ??? Also,
when the function is enabled, "system attract mode is not performed" -
whatever that means ???

This command does nothing on some SGB revisions. (SGBv2, SGB2?)

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

The recommended method is to "display" Game Boy BG tiles F9h..FFh from
left to right as first 7 characters of the bottom-most character line of
the Game Boy screen. As for normal 4KByte VRAM transfers, this area
should not be scrolled, should not be overlapped by Game Boy OBJs, and
the Game Boy BGP palette register should be set up properly. By following
that method, SNES OAM data can be defined in the 70h bytes of the
Game Boy BG tile memory at following addresses:

```
 8F90-8FEF  SNES OAM, 24 Entries of 4 bytes each (96 bytes)
 8FF0-8FF5  SNES OAM MSBs, 24 Entries of 2 bits each (6 bytes)
 8FF6-8FFF  Not used, don't care (10 bytes)
```

The format of SNES OAM Entries is:

```
  Byte 0  OBJ X-Position (0-511, MSB is separately stored, see below)
  Byte 1  OBJ Y-Position (0-255)
  Byte 2-3  Attributes (16bit)
    Bit 0-8    Tile Number     (use only 00h-FFh, upper bit zero)
    Bit 9-11   Palette Number  (use only 4-7)
    Bit 12-13  OBJ Priority    (use only 3)
    Bit 14     X-Flip          (0=Normal, 1=Mirror horizontally)
    Bit 15     Y-Flip          (0=Normal, 1=Mirror vertically)
```

The format of SNES OAM MSB Entries is:

Actually, the format is unknown ??? However, 2 bits are used per entry:
One bit is the most significant bit of the OBJ X-Position.
The other bit specifies the OBJ size (8x8 or 16x16 pixels).
