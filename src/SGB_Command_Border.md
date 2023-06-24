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
         Bit 0   - Tile Numbers   (0=Tiles $00-$7F, 1=Tiles $80-$FF)
         Bit 1   - Tile Type      (0=BG Tiles, 1=OBJ Tiles)
         Bit 2-7 - Not used (zero)
 2-F   Not used (zero)
```

The tile data is sent by VRAM transfer (4 KiB).

```
 000-FFF  Bitmap data for 128 Tiles
```

Each tile occupies 32 bytes (8×8 pixels, 16 colors each). When intending
to transfer more than 128 tiles, call this function twice (once for
tiles $00-$7F, and once for tiles $80-$FF). Note: The BG/OBJ Bit seems
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

The map data is sent by VRAM transfer (4 KiB).

```
 000-6FF  BG Map 32×28 Entries of 16 bits each (1792 bytes)
 700-73F  BG Map 1×28 extra row, 32 entries of 16 bits each (64 bytes)
 740-7FF  Not used, don't care
 800-85F  BG Palette Data (Palettes 4-6, 16 little-endian RGB555 colors each)
 860-FFF  Not used, don't care
```

Each BG Map Entry consists of a 16-bit value as such:
`VH01 PP00 NNNN NNNN```

```
 Bit 0-9   - Character Number (use only $00-$FF, upper 2 bits zero)
 Bit 10-12 - Palette Number   (use only 4-6)
 Bit 13    - BG Priority      (use only 0)
 Bit 14    - X-Flip           (0=Normal, 1=Mirror horizontally)
 Bit 15    - Y-Flip           (0=Normal, 1=Mirror vertically)
```

The 32×28 map entries correspond to 256×224 pixels of the Super NES
screen. The 20×18 entries in the center of the 32×28 area should be set
to a blank (solid color 0) tile as transparent space for the Game Boy
window to be displayed inside. Non-transparent border data will cover
the Game Boy window (for example, *Mario's Picross* does this, as does
*WildSnake* to a lesser extent).

A border designed for a modern (post-2006) widescreen television may use the center 256×176 pixels and leave the top and bottom 24 lines blank.
Using letterbox allows more tile variety in the portion of the border that a widescreen TV's zoom mode does not cut off.

All borders repeat tiles. Assuming that the blank space for the GB
screen is a blank tile, and the letterbox (if any) is a solid tile, a
border defining all unique tiles would have to define this many tiles:

-   (256\*224-160\*144)/64+1 = 537 tiles in full-screen border
-   (256\*176-160\*144)/64+2 = 346 tiles in letterboxed border

Because the CHR RAM allocated by SGB for border holds only 256 tiles, a full-screen border must repeat at least 281 tiles and a letterboxed border at least 90.

It turns out that 29 rows of the border tilemap sent through PCT_TRN are at least partly visible in some situations.
The SGB system software sets the border layer's vertical scroll position (BG1VOFS) to 0.
Because the S-PPU normally displays lines BGxVOFS+1 through BGxVOFS+224 of each layer, this hides the first scanline of the top row of tiles and adds one scanline of the nominally invisible 29th row at the bottom.
Most of the time, SGB hides this extra line with forced blanking (writing $80 to INIDISP at address $012100).
While SGB is busy processing some packets, such as fading out the border's palette or loading a new scene's palette and attributes, it neglects to force blanking, making the line flicker on some TVs.
This can be seen even with some built-in borders.

To fully eliminate flicker, write a row of all-black tilemap entries after the bottom row of the border ($8700-$873F in VRAM in a PCT_TRN), or at least a row of tiles whose top row of pixels is blank.
If that is not convenient, such as if a border data format doesn't guarantee an all-black tile ID, you can make the flicker less noticeable by repeating the last scanline.
Take the bottommost row (at $86C0-$86FF in VRAM) and copy it to the extra row, flipped vertically (XOR with $8000).

The Super NES supports 8 background palettes.
The SGB system software (when run in a LLE such as Mesen 2) has been observed to use background palette 0 for the GB screen, palettes 1, 2, 3, and 7 for the menus, and palettes 4, 5, and 6 for the border.
Thus a border can use three 15-color palettes.

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
