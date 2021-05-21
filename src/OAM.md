
# VRAM Sprite Attribute Table (OAM)

The Game Boy PPU can display up to 40 sprites either in 8x8 or
in 8x16 pixels. Because of a limitation of hardware, only ten sprites
can be displayed per scan line. Sprite tiles have the same format as
BG tiles, but they are taken from the Sprite Tiles Table located at
$8000-8FFF and have unsigned numbering.

Sprite attributes reside in the Sprite Attribute Table (OAM - Object
Attribute Memory) at \$FE00-FE9F. Each of the 40 entries consists of
four bytes with the following meanings:

## Byte0 - Y Position

Y = Sprite's vertical position on the screen + 16. So for example,
Y=0 hides a sprite,
Y=2 hides a 8x8 sprite but displays the last two rows of a 8x16 sprite,
Y=16 displays a sprite at the top of the screen,
Y=144 displays a 8x16 sprite aligned with the bottom of the screen,
Y=152 displays a 8x8 sprite aligned with the bottom of the screen,
Y=154 displays the first six rows of a sprite at the bottom of the screen,
Y=160 hides a sprite.

## Byte1 - X Position

X = Sprite's horizontal position on the screen + 8. This works similarly
to the examples above, except that the width of a sprite is always 8. An
off-screen value (X=0 or X\>=168) hides the sprite, but the sprite still
affects the priority ordering, thus other sprites with lower priority may be
left out due to the ten sprites limit per scan-line.
A better way to hide a sprite is to set its Y-coordinate off-screen.

## Byte 2 - Tile Index

In 8x8 mode (LCDC bit 2 = 0), this byte specifies the sprite's only tile index ($00-$FF).
This unsigned value selects a tile from the memory area at $8000-$8FFF.
In CGB Mode this could be either in
VRAM bank 0 or 1, depending on bit 3 of the following byte.
In 8x16 mode (LCDC bit 2 = 1), the memory area at $8000-$8FFF is still interpreted
as a series of 8x8 tiles, where every 2 tiles form a sprite. In this mode, this byte
specifies the index of the first (top) tile of the sprite. This is enforced by the
hardware: the least significant bit of the tile index is ignored; that is, the top 8x8
tile is "NN & $FE", and the bottom 8x8 tile is "NN | $01".

## Byte3 - Attributes/Flags:

```
 Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
 Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
 Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
 Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
 Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
 Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
```

## Sprite Priorities and Conflicts

During each scanline's OAM scan, the PPU compares LY to each
sprite's Y position to find the 10 sprites on that line that appear
first in OAM (\$FE00-\$FE03 being the first). It discards the rest,
displaying only those 10 sprites on that line.
To keep unused sprites from affecting onscreen sprites, set their Y
coordinate to Y = 0 or Y \>= 160 (144 + 16) (Note: Y \<= 8 also works
if sprite size is set to 8x8). Just setting the X coordinate to X = 0 or
X \>= 168 (160 + 8) on a sprite will hide it, but it will still count
towards the 10 sprite limit per scanline, possibly causing another sprite
that appears later in OAM to be left undisplayed.

If using BGB, in the VRAM viewer - OAM tab, hover your
mouse over the small screen to highlight the sprites on a line. Sprites
hidden due to the limitation will be highlighted in red.

When these 10 sprites overlap, the highest priority one will appear
above all others, etc. (Thus, no Z-fighting.) In Non-CGB mode, the smaller the X
coordinate, the higher the priority. When X coordinates are the same, sprites located
first in OAM have a higher priority. In CGB mode, only the sprite's location in OAM
determines its priority.

::: tip NOTE

Priority among opaque pixels that overlap is determined using the rules explained
above. After the pixel with the highest priority has been determined,
the "BG and Window over OBJ" attribute of *only* that pixel is honored (or disregarded if
this is a transparent pixel, i.e. a pixel with color ID zero). Thus if a sprite with a
higher priority but with "BG and Window over OBJ" toggled on
overlaps a sprite with a lower priority and a nonzero background
pixel, the background pixel is displayed regardless of the
lower-priority sprite's "BG and Window over OBJ" attribute.

:::

## Writing Data to OAM Memory

The recommended method is to write the data to normal RAM first, and to
copy that RAM to OAM by using the DMA transfer function, initiated
through DMA register (FF46). Besides, it is also possible to
write data directly to the OAM area by using normal LD instructions, but this
works only during the HBlank and VBlank periods. The current state of
the LCD controller can be read out from the STAT register (FF41).
