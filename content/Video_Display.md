LCD Control Register
--------------------

### FF40 - LCDC - LCD Control (R/W)

` Bit 7 - LCD Display Enable             (0=Off, 1=On)`\
` Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)`\
` Bit 5 - Window Display Enable          (0=Off, 1=On)`\
` Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)`\
` Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)`\
` Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)`\
` Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)`\
` Bit 0 - BG Display (for CGB see below) (0=Off, 1=On)`

### LCDC.7 - LCD Display Enable

CAUTION: Stopping LCD operation (Bit 7 from 1 to 0) may be performed
during V-Blank ONLY, disabeling the display outside of the V-Blank
period may damage the hardware. This appears to be a serious issue,
Nintendo is reported to reject any games that do not follow this rule.
V-blank can be confirmed when the value of LY is greater than or equal
to 144. When the display is disabled the screen is blank (white), and
VRAM and OAM can be accessed freely.

\-\-- LCDC.0 has different Meanings depending on Gameboy Type \-\--

### LCDC.0 - 1) Monochrome Gameboy and SGB: BG Display

When Bit 0 is cleared, the background becomes blank (white). Window and
Sprites may still be displayed (if enabled in Bit 1 and/or Bit 5).

### LCDC.0 - 2) CGB in CGB Mode: BG and Window Master Priority

When Bit 0 is cleared, the background and window lose their priority -
the sprites will be always displayed on top of background and window,
independently of the priority flags in OAM and BG Map attributes.

### LCDC.0 - 3) CGB in Non CGB Mode: BG and Window Display

When Bit 0 is cleared, both background and window become blank (white),
ie. the Window Display Bit (Bit 5) is ignored in that case. Only Sprites
may still be displayed (if enabled in Bit 1). This is a possible
compatibility problem - any monochrome games (if any) that disable the
background, but still want to display the window wouldn\'t work properly
on CGBs.

LCD Status Register
-------------------

=== FF41 - STAT - LCDC Status (R/W) Bit 6 - LYC=LY Coincidence Interrupt
(1=Enable) (Read/Write) ===

` Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)`\
` Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)`\
` Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)`\
` Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)`\
` Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)`\
`           0: During H-Blank`\
`           1: During V-Blank`\
`           2: During Searching OAM-RAM`\
`           3: During Transfering Data to LCD Driver`

The two lower STAT bits show the current status of the LCD controller.

` Mode 0: The LCD controller is in the H-Blank period and`\
`         the CPU can access both the display RAM (8000h-9FFFh)`\
`         and OAM (FE00h-FE9Fh)`\
` `\
` Mode 1: The LCD contoller is in the V-Blank period (or the`\
`         display is disabled) and the CPU can access both the`\
`         display RAM (8000h-9FFFh) and OAM (FE00h-FE9Fh)`\
` `\
` Mode 2: The LCD controller is reading from OAM memory.`\
`         The CPU `<cannot>` access OAM memory (FE00h-FE9Fh)`\
`         during this period.`\
` `\
` Mode 3: The LCD controller is reading from both OAM and VRAM,`\
`         The CPU `<cannot>` access OAM and VRAM during this period.`\
`         CGB Mode: Cannot access Palette Data (FF69,FF6B) either.`

The following are typical when the display is enabled:

` Mode 2  2_____2_____2_____2_____2_____2___________________2____`\
` Mode 3  _33____33____33____33____33____33__________________3___`\
` Mode 0  ___000___000___000___000___000___000________________000`\
` Mode 1  ____________________________________11111111111111_____`

The Mode Flag goes through the values 0, 2, and 3 at a cycle of about
109uS. 0 is present about 48.6uS, 2 about 19uS, and 3 about 41uS. This
is interrupted every 16.6ms by the VBlank (1). The mode flag stays set
at 1 for about 1.08 ms.

Mode 0 is present between 201-207 clks, 2 about 77-83 clks, and 3 about
169-175 clks. A complete cycle through these states takes 456 clks.
VBlank lasts 4560 clks. A complete screen refresh occurs every 70224
clks.)

LCD Interrupts
--------------

### INT 40 - V-Blank Interrupt

The V-Blank interrupt occurs ca. 59.7 times a second on a regular GB and
ca. 61.1 times a second on a Super GB (SGB). This interrupt occurs at
the beginning of the V-Blank period (LY=144). During this period video
hardware is not using video ram so it may be freely accessed. This
period lasts approximately 1.1 milliseconds.

### INT 48 - LCDC Status Interrupt

There are various reasons for this interrupt to occur as described by
the STAT register (\$FF40). One very popular reason is to indicate to
the user when the video hardware is about to redraw a given LCD line.
This can be useful for dynamically controlling the SCX/SCY registers
(\$FF43/\$FF42) to perform special video effects.

LCD Position and Scrolling
--------------------------

### FF42 - SCY - Scroll Y (R/W), FF43 - SCX - Scroll X (R/W)

Specifies the position in the 256x256 pixels BG map (32x32 tiles) which
is to be displayed at the upper/left LCD display position. Values in
range from 0-255 may be used for X/Y each, the video controller
automatically wraps back to the upper (left) position in BG map when
drawing exceeds the lower (right) border of the BG map area.

### FF44 - LY - LCDC Y-Coordinate (R)

The LY indicates the vertical line to which the present data is
transferred to the LCD Driver. The LY can take on any value between 0
through 153. The values between 144 and 153 indicate the V-Blank period.
Writing will reset the counter.

### FF45 - LYC - LY Compare (R/W)

The gameboy permanently compares the value of the LYC and LY registers.
When both values are identical, the coincident bit in the STAT register
becomes set, and (if enabled) a STAT interrupt is requested.

### FF4A - WY - Window Y Position (R/W), FF4B - WX - Window X Position minus 7 (R/W)

Specifies the upper/left positions of the Window area. (The window is an
alternate background area which can be displayed above of the normal
background. OBJs (sprites) may be still displayed above or behinf the
window, just as for normal BG.) The window becomes visible (if enabled)
when positions are set in range WX=0..166, WY=0..143. A postion of WX=7,
WY=0 locates the window at upper left, it is then completly covering
normal background.

LCD Monochrome Palettes
-----------------------

### FF47 - BGP - BG Palette Data (R/W) - Non CGB Mode Only

This register assigns gray shades to the color numbers of the BG and
Window tiles.

` Bit 7-6 - Shade for Color Number 3`\
` Bit 5-4 - Shade for Color Number 2`\
` Bit 3-2 - Shade for Color Number 1`\
` Bit 1-0 - Shade for Color Number 0`

The four possible gray shades are:

` 0  White`\
` 1  Light gray`\
` 2  Dark gray`\
` 3  Black`

In CGB Mode the Color Palettes are taken from CGB Palette Memory
instead.

### FF48 - OBP0 - Object Palette 0 Data (R/W) - Non CGB Mode Only

This register assigns gray shades for sprite palette 0. It works exactly
as BGP (FF47), except that the lower two bits aren\'t used because
sprite data 00 is transparent.

### FF49 - OBP1 - Object Palette 1 Data (R/W) - Non CGB Mode Only

This register assigns gray shades for sprite palette 1. It works exactly
as BGP (FF47), except that the lower two bits aren\'t used because
sprite data 00 is transparent.

LCD Color Palettes (CGB only)
-----------------------------

### FF68 - BCPS/BGPI - CGB Mode Only - Background Palette Index

This register is used to address a byte in the CGBs Background Palette
Memory. Each two byte in that memory define a color value. The first 8
bytes define Color 0-3 of Palette 0 (BGP0), and so on for BGP1-7.

` Bit 0-5   Index (00-3F)`\
` Bit 7     Auto Increment  (0=Disabled, 1=Increment after Writing)`

Data can be read/written to/from the specified index address through
Register FF69. When the Auto Increment Bit is set then the index is
automatically incremented after each <write> to FF69. Auto Increment has
no effect when <reading> from FF69, so the index must be manually
incremented in that case.

### FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data

This register allows to read/write data to the CGBs Background Palette
Memory, addressed through Register FF68. Each color is defined by two
bytes (Bit 0-7 in first byte).

` Bit 0-4   Red Intensity   (00-1F)`\
` Bit 5-9   Green Intensity (00-1F)`\
` Bit 10-14 Blue Intensity  (00-1F)`

Much like VRAM, Data in Palette Memory cannot be read/written during the
time when the LCD Controller is reading from it. (That is when the STAT
register indicates Mode 3). Note: Initially all background colors are
initialized as white.

### FF6A - OCPS/OBPI - CGB Mode Only - Sprite Palette Index, FF6B - OCPD/OBPD - CGB Mode Only - Sprite Palette Data

These registers are used to initialize the Sprite Palettes OBP0-7,
identically as described above for Background Palettes. Note that four
colors may be defined for each OBP Palettes - but only Color 1-3 of each
Sprite Palette can be displayed, Color 0 is always transparent, and can
be initialized to a don\'t care value. Note: Initially all sprite colors
are uninitialized.

### RGB Translation by CGBs

When developing graphics on PCs, note that the RGB values will have
different appearance on CGB displays as on VGA monitors: The highest
intensity will produce Light Gray color rather than White. The
intensities are not linear; the values 10h-1Fh will all appear very
bright, while medium and darker colors are ranged at 00h-0Fh. The CGB
display will mix colors quite oddly, increasing intensity of only one
R,G,B color will also influence the other two R,G,B colors. For example,
a color setting of 03EFh (Blue=0, Green=1Fh, Red=0Fh) will appear as
Neon Green on VGA displays, but on the CGB it\'ll produce a decently
washed out Yellow.

### RGB Translation by GBAs

Even though GBA is described to be compatible to CGB games, most CGB
games are completely unplayable on GBAs because most colors are
invisible (black). Of course, colors such like Black and White will
appear the same on both CGB and GBA, but medium intensities are arranged
completely different. Intensities in range 00h..0Fh are invisible/black
(unless eventually under best sunlight circumstances, and when gazing at
the screen under obscure viewing angles), unfortunately, these
intensities are regulary used by most existing CGB games for medium and
darker colors. Newer CGB games may avoid this effect by changing palette
data when detecting GBA hardware. A relative simple method would be
using the formula GBA=CGB/2+10h for each R,G,B intensity, probably the
result won\'t be perfect, and (once colors became visible) it may turn
out that the color mixing is different also, anyways, it\'d be still
ways better than no conversion. Asides, this translation method should
have been VERY easy to implement in GBA hardware directly, even though
Nintendo obviously failed to do so. How did they say, This seal is your
assurance for excellence in workmanship and so on?

LCD OAM DMA Transfers
---------------------

### FF46 - DMA - DMA Transfer and Start Address (W)

Writing to this register launches a DMA transfer from ROM or RAM to OAM
memory (sprite attribute table). The written value specifies the
transfer source address divided by 100h, ie. source & destination are:

` Source:      XX00-XX9F   ;XX in range from 00-F1h`\
` Destination: FE00-FE9F`

It takes 160 microseconds until the transfer has completed (80
microseconds in CGB Double Speed Mode), during this time the CPU can
access only HRAM (memory at FF80-FFFE). For this reason, the programmer
must copy a short procedure into HRAM, and use this procedure to start
the transfer from inside HRAM, and wait until the transfer has finished:

`  ld  (0FF46h),a ;start DMA transfer, a=start address/100h`\
`  ld  a,28h      ;delay...`\
` wait:           ;total 5x40 cycles, approx 200ms`\
`  dec a          ;1 cycle`\
`  jr  nz,wait    ;4 cycles`

Most programs are executing this procedure from inside of their VBlank
procedure, but it is possible to execute it during display redraw also,
allowing to display more than 40 sprites on the screen (ie. for example
40 sprites in upper half, and other 40 sprites in lower half of the
screen).

LCD VRAM DMA Transfers (CGB only)
---------------------------------

### FF51 - HDMA1 - CGB Mode Only - New DMA Source, High; FF52 - HDMA2 - CGB Mode Only - New DMA Source, Low; FF53 - HDMA3 - CGB Mode Only - New DMA Destination, High; FF54 - HDMA4 - CGB Mode Only - New DMA Destination, Low; FF55 - HDMA5 - CGB Mode Only - New DMA Length/Mode/Start

These registers are used to initiate a DMA transfer from ROM or RAM to
VRAM. The Source Start Address may be located at 0000-7FF0 or A000-DFF0,
the lower four bits of the address are ignored (treated as zero). The
Destination Start Address may be located at 8000-9FF0, the lower four
bits of the address are ignored (treated as zero), the upper 3 bits are
ignored either (destination is always in VRAM).

Writing to FF55 starts the transfer, the lower 7 bits of FF55 specify
the Transfer Length (divided by 10h, minus 1). Ie. lengths of 10h-800h
bytes can be defined by the values 00h-7Fh. And the upper bit of FF55
indicates the Transfer Mode:

=== Bit7=0 - General Purpose DMA === When using this transfer method,
all data is transferred at once. The execution of the program is halted
until the transfer has completed. Note that the General Purpose DMA
blindly attempts to copy the data, even if the LCD controller is
currently accessing VRAM. So General Purpose DMA should be used only if
the Display is disabled, or during V-Blank, or (for rather short blocks)
during H-Blank. The execution of the program continues when the transfer
has been completed, and FF55 then contains a value if FFh.

=== Bit7=1 - H-Blank DMA === The H-Blank DMA transfers 10h bytes of data
during each H-Blank, ie. at LY=0-143, no data is transferred during
V-Blank (LY=144-153), but the transfer will then continue at LY=00. The
execution of the program is halted during the separate transfers, but
the program execution continues during the \'spaces\' between each data
block. Note that the program may not change the Destination VRAM bank
(FF4F), or the Source ROM/RAM bank (in case data is transferred from
bankable memory) until the transfer has completed! Reading from Register
FF55 returns the remaining length (divided by 10h, minus 1), a value of
0FFh indicates that the transfer has completed. It is also possible to
terminate an active H-Blank transfer by writing zero to Bit 7 of FF55.
In that case reading from FF55 may return any value for the lower 7
bits, but Bit 7 will be read as \"1\".

### Confirming if the DMA Transfer is Active

Reading Bit 7 of FF55 can be used to confirm if the DMA transfer is
active (1=Not Active, 0=Active). This works under any circumstances -
after completion of General Purpose, or H-Blank Transfer, and after
manually terminating a H-Blank Transfer.

### Transfer Timings

In both Normal Speed and Double Speed Mode it takes about 8us to
transfer a block of 10h bytes. That are 8 cycles in Normal Speed Mode,
and 16 \'fast\' cycles in Double Speed Mode. Older MBC controllers (like
MBC1-4) and slower ROMs are not guaranteed to support General Purpose or
H-Blank DMA, that\'s because there are always 2 bytes transferred per
microsecond (even if the itself program runs it Normal Speed Mode).

VRAM Tile Data
--------------

Tile Data is stored in VRAM at addresses 8000h-97FFh, this area defines
the Bitmaps for 192 Tiles. In CGB Mode 384 Tiles can be defined, because
memory at 0:8000h-97FFh and at 1:8000h-97FFh is used.

Each tile is sized 8x8 pixels and has a color depth of 4 colors/gray
shades. Tiles can be displayed as part of the Background/Window map,
and/or as OAM tiles (foreground sprites). Note that foreground sprites
may have only 3 colors, because color 0 is transparent.

As it was said before, there are two Tile Pattern Tables at \$8000-8FFF
and at \$8800-97FF. The first one can be used for sprites and the
background. Its tiles are numbered from 0 to 255. The second table can
be used for the background and the window display and its tiles are
numbered from -128 to 127.

Each Tile occupies 16 bytes, where each 2 bytes represent a line:

` Byte 0-1  First Line (Upper 8 pixels)`\
` Byte 2-3  Next Line`\
` etc.`

For each line, the first byte defines the least significant bits of the
color numbers for each pixel, and the second byte defines the upper bits
of the color numbers. In either case, Bit 7 is the leftmost pixel, and
Bit 0 the rightmost.

So, each pixel is having a color number in range from 0-3. The color
numbers are translated into real colors (or gray shades) depending on
the current palettes. The palettes are defined through registers
FF47-FF49 (Non CGB Mode), and FF68-FF6B (CGB Mode).

VRAM Background Maps
--------------------

The gameboy contains two 32x32 tile background maps in VRAM at addresses
9800h-9BFFh and 9C00h-9FFFh. Each can be used either to display
\"normal\" background, or \"window\" background.

### BG Map Tile Numbers

An area of VRAM known as Background Tile Map contains the numbers of
tiles to be displayed. It is organized as 32 rows of 32 bytes each. Each
byte contains a number of a tile to be displayed. Tile patterns are
taken from the Tile Data Table located either at \$8000-8FFF or
\$8800-97FF. In the first case, patterns are numbered with unsigned
numbers from 0 to 255 (i.e. pattern \#0 lies at address \$8000). In the
second case, patterns have signed numbers from -128 to 127 (i.e. pattern
\#0 lies at address \$9000). The Tile Data Table address for the
background can be selected via LCDC register.

### BG Map Attributes (CGB Mode only)

In CGB Mode, an additional map of 32x32 bytes is stored in VRAM Bank 1
(each byte defines attributes for the corresponding tile-number map
entry in VRAM Bank 0):

` Bit 0-2  Background Palette number  (BGP0-7)`\
` Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)`\
` Bit 4    Not used`\
` Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)`\
` Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)`\
` Bit 7    BG-to-OAM Priority         (0=Use OAM priority bit, 1=BG Priority)`

When Bit 7 is set, the corresponding BG tile will have priority above
all OBJs (regardless of the priority bits in OAM memory). There\'s also
an Master Priority flag in LCDC register Bit 0 which overrides all other
priority bits when cleared.

As one background tile has a size of 8x8 pixels, the BG maps may hold a
picture of 256x256 pixels, an area of 160x144 pixels of this picture can
be displayed on the LCD screen.

### Normal Background (BG)

The SCY and SCX registers can be used to scroll the background, allowing
to select the origin of the visible 160x144 pixel area within the total
256x256 pixel background map. Background wraps around the screen (i.e.
when part of it goes off the screen, it appears on the opposite side.)

### The Window

Besides background, there is also a \"window\" overlaying the
background. The window is not scrollable i.e. it is always displayed
starting from its left upper corner. The location of a window on the
screen can be adjusted via WX and WY registers. Screen coordinates of
the top left corner of a window are WX-7,WY. The tiles for the window
are stored in the Tile Data Table. Both the Background and the window
share the same Tile Data Table.

Both background and window can be disabled or enabled separately via
bits in the LCDC register.

VRAM Sprite Attribute Table (OAM)
---------------------------------

GameBoy video controller can display up to 40 sprites either in 8x8 or
in 8x16 pixels. Because of a limitation of hardware, only ten sprites
can be displayed per scan line. Sprite patterns have the same format as
BG tiles, but they are taken from the Sprite Pattern Table located at
\$8000-8FFF and have unsigned numbering.

Sprite attributes reside in the Sprite Attribute Table (OAM - Object
Attribute Memory) at \$FE00-FE9F. Each of the 40 entries consists of
four bytes with the following meanings:

### Byte0 - Y Position

Specifies the sprites vertical position on the screen (minus 16). An
offscreen value (for example, Y=0 or Y\>=160) hides the sprite.

### Byte1 - X Position

Specifies the sprites horizontal position on the screen (minus 8). An
offscreen value (X=0 or X\>=168) hides the sprite, but the sprite still
affects the priority ordering - a better way to hide a sprite is to set
its Y-coordinate offscreen.

### Byte2 - Tile/Pattern Number

Specifies the sprites Tile Number (00-FF). This (unsigned) value selects
a tile from memory at 8000h-8FFFh. In CGB Mode this could be either in
VRAM Bank 0 or 1, depending on Bit 3 of the following byte. In 8x16
mode, the lower bit of the tile number is ignored. Ie. the upper 8x8
tile is \"NN AND FEh\", and the lower 8x8 tile is \"NN OR 01h\".

### Byte3 - Attributes/Flags:

` Bit7   OBJ-to-BG Priority (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)`\
`        (Used for both BG and Window. BG color 0 is always behind OBJ)`\
` Bit6   Y flip          (0=Normal, 1=Vertically mirrored)`\
` Bit5   X flip          (0=Normal, 1=Horizontally mirrored)`\
` Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)`\
` Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)`\
` Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)`

### Sprite Priorities and Conflicts

When sprites with different x coordinate values overlap, the one with
the smaller x coordinate (closer to the left) will have priority and
appear above any others. This applies in Non CGB Mode only. When sprites
with the same x coordinate values overlap, they have priority according
to table ordering. (i.e. \$FE00 - highest, \$FE04 - next highest, etc.)
In CGB Mode priorities are always assigned like this.

Only 10 sprites can be displayed on any one line. When this limit is
exceeded, the lower priority sprites (priorities listed above) won\'t be
displayed. To keep unused sprites from affecting onscreen sprites set
their Y coordinate to Y=0 or Y=\>144+16. Just setting the X coordinate
to X=0 or X=\>160+8 on a sprite will hide it but it will still affect
other sprites sharing the same lines.

### Writing Data to OAM Memory

The recommened method is to write the data to normal RAM first, and to
copy that RAM to OAM by using the DMA transfer function, initiated
through DMA register (FF46). Beside for that, it is also possible to
write data directly to the OAM area by using normal LD commands, this
works only during the H-Blank and V-Blank periods. The current state of
the LCD controller can be read out from the STAT register (FF41).

Accessing VRAM and OAM
----------------------

### CAUTION

When the LCD Controller is drawing the screen it is directly reading
from Video Memory (VRAM) and from the Sprite Attribute Table (OAM).
During these periods the Gameboy CPU may not access the VRAM and OAM.
That means, any attempts to write to VRAM/OAM are ignored (the data
remains unchanged). And any attempts to read from VRAM/OAM will return
undefined data (typically a value of FFh).

For this reason the program should verify if VRAM/OAM is accessable
before actually reading or writing to it. This is usually done by
reading the Mode Bits from the STAT Register (FF41). When doing this (as
described in the examples below) you should take care that no interrupts
occur between the wait loops and the following memory access - the
memory is guaranted to be accessable only for a few cycles directly
after the wait loops have completed.

### VRAM (memory at 8000h-9FFFh) is accessable during Mode 0-2

` Mode 0 - H-Blank Period,`\
` Mode 1 - V-Blank Period, and`\
` Mode 2 - Searching OAM Period`

A typical procedure that waits for accessibility of VRAM would be:

` ld   hl,0FF41h    ;-STAT Register`\
`@@wait:            ;\`\
` bit  1,(hl)       ; Wait until Mode is 0 or 1`\
` jr   nz,@@wait    ;/`

Even if the procedure gets executed at the <end> of Mode 0 or 1, it is
still proof to assume that VRAM can be accessed for a few more cycles
because in either case the following period is Mode 2 which allows
access to VRAM either. In CGB Mode an alternate method to write data to
VRAM is to use the HDMA Function (FF51-FF55).

### OAM (memory at FE00h-FE9Fh) is accessable during Mode 0-1

` Mode 0 - H-Blank Period`\
` Mode 1 - V-Blank Period`

Aside from that, OAM can be accessed at any time by using the DMA
Function (FF46). When directly reading or writing to OAM, a typical
procedure that waits for accessibility of OAM Memory would be:

` ld   hl,0FF41h    ;-STAT Register`\
`@@wait1:           ;\`\
` bit  1,(hl)       ; Wait until Mode is -NOT- 0 or 1`\
` jr   z,@@wait1    ;/`\
`@@wait2:           ;\`\
` bit  1,(hl)       ; Wait until Mode 0 or 1 -BEGINS-`\
` jr   nz,@@wait2   ;/`

The two wait loops ensure that Mode 0 or 1 will last for a few clock
cycles after completion of the procedure. In V-Blank period it might be
recommended to skip the whole procedure - and in most cases using the
above mentioned DMA function would be more recommended anyways.

### Note

When the display is disabled, both VRAM and OAM are accessable at any
time. The downside is that the screen is blank (white) during this
period, so that disabling the display would be recommended only during
initialization.

