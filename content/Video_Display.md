LCD Control Register
--------------------

### FF40 - LCDC - LCD Control (R/W)

LCDC is special, because it can be written to during Mode 3 (while
graphics are being drawn), and **will** affect display during the
scanline !

This could be used for very complex video effects, but they will
probably fail to render properly on most emulators. (They could be
pretty sick though)

` Bit 7 - LCD Display Enable             (0=Off, 1=On)`\
` Bit 6 - Window Tile Map Display Select (0=9800-9BFF, 1=9C00-9FFF)`\
` Bit 5 - Window Display Enable          (0=Off, 1=On)`\
` Bit 4 - BG & Window Tile Data Select   (0=8800-97FF, 1=8000-8FFF)`\
` Bit 3 - BG Tile Map Display Select     (0=9800-9BFF, 1=9C00-9FFF)`\
` Bit 2 - OBJ (Sprite) Size              (0=8x8, 1=8x16)`\
` Bit 1 - OBJ (Sprite) Display Enable    (0=Off, 1=On)`\
` Bit 0 - BG/Window Display/Priority     (0=Off, 1=On)`

#### LCDC.7 - LCD Display Enable

CAUTION: Stopping LCD operation (Bit 7 from 1 to 0) may be performed
during V-Blank ONLY, disabling the display outside of the V-Blank period
may damage the hardware. This appears to be a serious issue, Nintendo is
reported to reject any games that do not follow this rule.

V-blank can be confirmed when the value of LY is greater than or equal
to 144. When the display is disabled the screen is blank (white), and
VRAM and OAM can be accessed freely.

When re-enabling the LCD, it will immediately start at LY = 0, meaning
it will immediately start drawing.

#### LCDC.6 - Window Tile Map Display Select

This bit controls which [background
map](#VRAM_Background_Maps "wikilink") the Window uses for rendering.
Note that the window will always use the \"top-left\" data, ie. the data
you\'d see at SCY = 0, SCX = 0, as its top-left point.

#### LCDC.5 - Window Display Enable

This bit controls whether the window shall be displayed or not. (TODO :
what happens when toggling this mid-scanline ?)

Note that on CGB models, setting this bit to 0 then back to 1 mid-frame
may cause the second write to be ignored. (TODO : test this.)

#### LCDC.4 - BG & Window Tile Data Select

This bit controls which addressing mode the BG and Window use to pick
tiles. (See [below](#VRAM_Tile_Data "wikilink") for details on
addressing modes).

#### LCDC.3 - BG Tile Map Display Select

This bit works similarly to bit 6 (explained above).

#### LCDC.2 - OBJ Size

This bit controls the sprite size (1 tile or 2 stacked vertically).

Be cautious when changing this mid-frame from 8x8 to 8x16 : \"remnants\"
of the sprites intended for 8x8 could \"leak\" into the 8x16 zone and
cause artifacts.

#### LCDC.1 - OBJ Display Enable

This bit toggles whether sprites are displayed or not. This doesn\'t
affect Mode 3 timings, just whether they are rendered or not.

This can be toggled mid-frame, for example to avoid sprites being
displayed above a status bar or text box.

#### LCDC.0 - BG/Window Display/Priority

LCDC.0 has different meanings depending on Gameboy type and Mode:

##### Monochrome Gameboy, SGB and CGB in Non-CGB Mode: BG Display

When Bit 0 is cleared, both background and window become blank (white),
ie. the Window Display Bit (Bit 5) is ignored in that case. Only Sprites
may still be displayed (if enabled in Bit 1).

##### CGB in CGB Mode: BG and Window Master Priority

When Bit 0 is cleared, the background and window lose their priority -
the sprites will be always displayed on top of background and window,
independently of the priority flags in OAM and BG Map attributes.

LCD Status Register
-------------------

### FF41 - STAT - LCDC Status (R/W)

` Bit 6 - LYC=LY Coincidence Interrupt (1=Enable) (Read/Write)`\
` Bit 5 - Mode 2 OAM Interrupt         (1=Enable) (Read/Write)`\
` Bit 4 - Mode 1 V-Blank Interrupt     (1=Enable) (Read/Write)`\
` Bit 3 - Mode 0 H-Blank Interrupt     (1=Enable) (Read/Write)`\
` Bit 2 - Coincidence Flag  (0:LYC<>LY, 1:LYC=LY) (Read Only)`\
` Bit 1-0 - Mode Flag       (Mode 0-3, see below) (Read Only)`\
`           0: During H-Blank`\
`           1: During V-Blank`\
`           2: During Searching OAM`\
`           3: During Transferring Data to LCD Driver`

The two lower STAT bits show the current status of the LCD controller.

` Mode 0: The LCD controller is in the H-Blank period and`\
`         the CPU can access both the display RAM (8000h-9FFFh)`\
`         and OAM (FE00h-FE9Fh)`\
` `\
` Mode 1: The LCD controller is in the V-Blank period (or the`\
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

A hardware bug in the gray Game Boys makes the LCD interrupt sometimes
trigger when writing to STAT (including writing \$00) during a H-Blank
or V-Blank period.

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

Example application : set LYC to WY, enable LY=LYC interrupt, and have
the handler disable sprites. This can be used if you use the window for
a text box (at the bottom of the screen), and you want sprites to be
hidden by the text box.

The interrupt is triggered when transitioning from \"No conditions met\"
to \"Any condition met\", which can cause the interrupt to not fire.
Example : the Mode 0 and LY=LYC interrupts are enabled ; since the
latter triggers during Mode 2 (right after Mode 0), the interrupt will
trigger for Mode 0 but fail to for LY=LYC.

LCD Position and Scrolling
--------------------------

These registers can be accessed even during Mode 3, but they have no
effect until the end of the current scanline.

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

The Gameboy permanently compares the value of the LYC and LY registers.
When both values are identical, the coincident bit in the STAT register
becomes set, and (if enabled) a STAT interrupt is requested.

### FF4A - WY - Window Y Position (R/W), FF4B - WX - Window X Position minus 7 (R/W)

Specifies the upper/left positions of the Window area. (The window is an
alternate background area which can be displayed above of the normal
background. OBJs (sprites) may be still displayed above or behind the
window, just as for normal BG.)

The window becomes visible (if enabled) when positions are set in range
WX=0..166, WY=0..143. A position of WX=7, WY=0 locates the window at
upper left, it is then completely covering normal background.

Due to a hardware bug, if WX is set to 0, the window will \"stutter\"
horizontally when SCX changes. (Depending on SCX modulo 8, behavior is a
little complicated so you should try it yourself)

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
Register FF69. When the Auto Increment bit is set then the index is
automatically incremented after each <write> to FF69. Auto Increment has
no effect when <reading> from FF69, so the index must be manually
incremented in that case.

Unlike the following, this register can be accessed outside V-Blank and
H-Blank.

### FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data

This register allows to read/write data to the CGBs Background Palette
Memory, addressed through Register FF68. Each color is defined by two
bytes (Bit 0-7 in first byte).

` Bit 0-4   Red Intensity   (00-1F)`\
` Bit 5-9   Green Intensity (00-1F)`\
` Bit 10-14 Blue Intensity  (00-1F)`

Much like VRAM, data in Palette Memory cannot be read/written during the
time when the LCD Controller is reading from it. (That is when the STAT
register indicates Mode 3). Note: All background colors are initialized
as white by the boot ROM, but it\'s a good idea to initialize at least
one color yourself (for example if you include a soft-reset mechanic).

### FF6A - OCPS/OBPI - CGB Mode Only - Sprite Palette Index, FF6B - OCPD/OBPD - CGB Mode Only - Sprite Palette Data

These registers are used to initialize the Sprite Palettes OBP0-7,
identically as described above for Background Palettes. Note that four
colors may be defined for each OBP Palettes - but only Color 1-3 of each
Sprite Palette can be displayed, Color 0 is always transparent, and can
be initialized to a don\'t care value or plain never initialized.

Note: All sprite colors are left uninitialized by the boot ROM, and are
somewhat random.

### RGB Translation by CGBs

![VGA versus CGB color
mixing](VGA_versus_CGB.png "fig:VGA versus CGB color mixing"){width="150"}
When developing graphics on PCs, note that the RGB values will have
different appearance on CGB displays as on VGA/HDMI monitors: The
highest intensity will produce Light Gray color rather than White. The
intensities are not linear; the values 10h-1Fh will all appear very
bright, while medium and darker colors are ranged at 00h-0Fh.

The CGB display will mix colors quite oddly, increasing intensity of
only one R,G,B color will also influence the other two R,G,B colors. For
example, a color setting of 03EFh (Blue=0, Green=1Fh, Red=0Fh) will
appear as Neon Green on VGA displays, but on the CGB it\'ll produce a
decently washed out Yellow. See image on the right.

### RGB Translation by GBAs

Even though GBA is described to be compatible to CGB games, most CGB
games are completely unplayable on GBAs because most colors are
invisible (black). Of course, colors such like Black and White will
appear the same on both CGB and GBA, but medium intensities are arranged
completely different. Intensities in range 00h..0Fh are invisible/black
(unless eventually under best sunlight circumstances, and when gazing at
the screen under obscure viewing angles), unfortunately, these
intensities are regularly used by most existing CGB games for medium and
darker colors.

Newer CGB games may avoid this effect by changing palette data when
detecting GBA hardware ([see
how](CGB_Registers#Detecting_CGB_.28and_GBA.29_functions "wikilink")). A
relative simple method would be using the formula GBA=CGB/2+10h for each
R,G,B intensity, probably the result won\'t be perfect, and (once colors
become visible) it may turn out that the color mixing is different also;
anyways, it\'d be still ways better than no conversion. Asides, this
translation method should have been VERY easy to implement in GBA
hardware directly, even though Nintendo obviously failed to do so. How
did they say, \"This seal is your assurance for excellence in
workmanship\" and so on?

LCD OAM DMA Transfers
---------------------

### FF46 - DMA - DMA Transfer and Start Address (R/W)

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

`  ld a, start address / 100h`\
`  ldh  (FF46h),a ;start DMA transfer (starts right after instruction)`\
`  ld  a,28h      ;delay...`\
` wait:           ;total 4x40 cycles, approx 160ms`\
`  dec a          ;1 cycle`\
`  jr  nz,wait    ;3 cycles`\
`  ret`

Most programs are executing this procedure from inside of their VBlank
procedure, but it is possible to execute it during display redraw also,
allowing to display more than 40 sprites on the screen (ie. for example
40 sprites in upper half, and other 40 sprites in lower half of the
screen).

Sprites are not displayed while OAM DMA is being performed.

A more compact procedure is

`  ldh ($FF00+c), a`\
` wait:`\
`  dec b`\
`  jr nz,wait`\
`  ret`

which should be called with a = start address / 100h, c = 46h, b = 29h.
This saves 5 bytes of HRAM, but is slightly slower overall (except in
some specific cases).

LCD VRAM DMA Transfers (CGB only)
---------------------------------

### FF51 - HDMA1 - CGB Mode Only - New DMA Source, High

### FF52 - HDMA2 - CGB Mode Only - New DMA Source, Low

These two registers specify the address at which the transfer will read
data from. Normally, this should be either in ROM, SRAM or WRAM, thus
either in range 0000-7FF0 or A000-DFF0. \[Note : this has yet to be
tested on Echo RAM, OAM, FEXX, IO and HRAM\]. Trying to specify a source
address in VRAM will cause garbage to be copied.

The four lower bits of this address will be ignored and treated as 0.

### FF53 - HDMA3 - CGB Mode Only - New DMA Destination, High

### FF54 - HDMA4 - CGB Mode Only - New DMA Destination, Low

These two registers specify the address to which the data will be
copied. \[Note : bits are supposedly ignored so this is in VRAM, but it
doesn\'t seem to actually be the case\...\]

The four lower bits of this address will be ignored and treated as 0.

### FF55 - HDMA5 - CGB Mode Only - New DMA Length/Mode/Start

These registers are used to initiate a DMA transfer from ROM or RAM to
VRAM. The Source Start Address may be located at 0000-7FF0 or A000-DFF0,
the lower four bits of the address are ignored (treated as zero). The
Destination Start Address may be located at 8000-9FF0, the lower four
bits of the address are ignored (treated as zero), the upper 3 bits are
ignored either (destination is always in VRAM).

Writing to this register starts the transfer, the lower 7 bits of which
specify the Transfer Length (divided by 10h, minus 1), ie. lengths of
10h-800h bytes can be defined by the values 00h-7Fh. The upper bit
indicates the Transfer Mode:

==== Bit7=0 - General Purpose DMA ==== When using this transfer method,
all data is transferred at once. The execution of the program is halted
until the transfer has completed. Note that the General Purpose DMA
blindly attempts to copy the data, even if the LCD controller is
currently accessing VRAM. So General Purpose DMA should be used only if
the Display is disabled, or during V-Blank, or (for rather short blocks)
during H-Blank. The execution of the program continues when the transfer
has been completed, and FF55 then contains a value of FFh.

==== Bit7=1 - H-Blank DMA ==== The H-Blank DMA transfers 10h bytes of
data during each H-Blank, ie. at LY=0-143, no data is transferred during
V-Blank (LY=144-153), but the transfer will then continue at LY=00. The
execution of the program is halted during the separate transfers, but
the program execution continues during the \'spaces\' between each data
block. Note that the program should not change the Destination VRAM bank
(FF4F), or the Source ROM/RAM bank (in case data is transferred from
bankable memory) until the transfer has completed! (The transfer should
be paused as described below while the banks are switched)

Reading from Register FF55 returns the remaining length (divided by 10h,
minus 1), a value of 0FFh indicates that the transfer has completed. It
is also possible to terminate an active H-Blank transfer by writing zero
to Bit 7 of FF55. In that case reading from FF55 will return how many
\$10 \"blocks\" remained (minus 1) in the lower 7 bits, but Bit 7 will
be read as \"1\". Stopping the transfer doesn\'t set HDMA1-4 to \$FF.

### Precautions

H-Blank DMA should not be started (write to FF55) during a H-Blank
period (STAT mode 0).

If the transfer\'s destination address overflows, the transfer stops
prematurely. \[Note : what\'s the state of the registers if this happens
?\]

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

Tile Data is stored in VRAM at addresses \$8000h-97FF; with one tile
being 16 bytes large, this area defines data for 384 Tiles. In CGB Mode,
this is doubled (768 tiles) because of the two [LCDC bit
4](#VRAM_banks._Each_tile_is_sized_8x8_pixels_and_has_a_color_depth_of_4_colors/gray_shades._Tiles_can_be_displayed_as_part_of_the_Background/Window_map,_and/or_as_OAM_tiles_(foreground_sprites)._Note_that_foreground_sprites_don't_use_color_0_-_it's_transparent_instead._There_are_three_"blocks"_of_128_tiles_each_:_block_0_is_$8000-87FF,_block_1_is_$8800-8FFF,_block_-1_is_$9000-$97FF._Tiles_are_always_indexed_using_a_8-bit_integer,_but_the_addressing_method_may_differ._The_"8000_method"_uses_$8000_as_its_base_pointer_and_uses_an_unsigned_addressing,_meaning_that_tiles_0-127_are_in_block_0,_and_tiles_128-255_are_in_block_1._The_"8800_method"_uses_$9000_as_its_base_pointer_and_uses_a_signed_addressing._To_put_it_differently,_"8000_addressing"_takes_tiles_0-127_from_block_0_and_tiles_128-255_from_block_1,_whereas_"8800_addressing"_takes_tiles_0-127_from_block_-1_and_tiles_128-255_from_block_1._(You_can_notice_that_block_1_is_shared_by_both_addressing_methods)_Sprites_always_use_8000_addressing,_but_the_BG_and_Window_can_use_either_mode,_controlled_by_[[#LCDC.4_-_BG_.26_Window_Tile_Data_Select "wikilink").

Each Tile occupies 16 bytes, where each 2 bytes represent a line:

` Byte 0-1  First Line (Upper 8 pixels)`\
` Byte 2-3  Next Line`\
` etc.`

For each line, the first byte defines the least significant bits of the
color numbers for each pixel, and the second byte defines the upper bits
of the color numbers. In either case, Bit 7 is the leftmost pixel, and
Bit 0 the rightmost. For example : let\'s say you have \$57 \$36 (in
this order in memory). To obtain the color index for the leftmost pixel,
you take bit 7 of both bytes : 0, and 0. Thus the index is 00b = 0. For
the second pixel, repeat with bit 6 : 1, and 0. Thus the index is 01b =
1 (remember to flip the order of the bits !). If you repeat the
operation you\'ll find that the indexes for the 8 pixels are 0 1 2 3 0 3
3 1.

So, each pixel is having a color number in range from 0-3. The color
numbers are translated into real colors (or gray shades) depending on
the current palettes. The palettes are defined through registers
[BGP](#FF47_-_BGP_-_BG_Palette_Data_.28R.2FW.29_-_Non_CGB_Mode_Only "wikilink"),
[OBP0](#FF48_-_OBP0_-_Object_Palette_0_Data_.28R.2FW.29_-_Non_CGB_Mode_Only "wikilink")
and
[OBP1](#FF49_-_OBP1_-_Object_Palette_1_Data_.28R.2FW.29_-_Non_CGB_Mode_Only "wikilink")
(Non CGB Mode), and
[BCPS/BGPI](#FF68_-_BCPS.2FBGPI_-_CGB_Mode_Only_-_Background_Palette_Index "wikilink"),
[BCPD/BGPD](#FF69_-_BCPD.2FBGPD_-_CGB_Mode_Only_-_Background_Palette_Data "wikilink"),
[OCPS/OBPI and
OCPD/OBPD](#FF6A_-_OCPS.2FOBPI_-_CGB_Mode_Only_-_Sprite_Palette_Index.2C_FF6B_-_OCPD.2FOBPD_-_CGB_Mode_Only_-_Sprite_Palette_Data "wikilink")
(CGB Mode).

VRAM Background Maps
--------------------

The Gameboy contains two 32x32 tile background maps in VRAM at addresses
9800h-9BFFh and 9C00h-9FFFh. Each can be used either to display
\"normal\" background, or \"window\" background.

### BG Map Tile Numbers

An area of VRAM known as Background Tile Map contains the numbers of
tiles to be displayed. It is organized as 32 rows of 32 bytes each. Each
byte contains a number of a tile to be displayed.

Tile patterns are taken from the Tile Data Table using either of the two
addressing modes (described [above](#VRAM_Tile_Data "wikilink")), which
can be selected via LCDC register.

As one background tile has a size of 8x8 pixels, the BG maps may hold a
picture of 256x256 pixels, and an area of 160x144 pixels of this picture
can be displayed on the LCD screen.

### BG Map Attributes (CGB Mode only)

In CGB Mode, an additional map of 32x32 bytes is stored in VRAM Bank 1
(each byte defines attributes for the corresponding tile-number map
entry in VRAM Bank 0, ie. 1:9800 defines the attributes for the tile at
0:9800):

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

Note that, if the tile at 0:9800 is tile \$2A, the attribute at 1:9800
doesn\'t define properties for ALL tiles \$2A on-screen, but only the
one at 0:9800 !

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

VRAM Banks (CGB only)
---------------------

The CGB has twice the VRAM of the DMG, but it is banked and either bank
has a different purpose.

### FF4F - VBK - CGB Mode Only - VRAM Bank (R/W)

This register can be written to to change VRAM banks. Only bit 0
matters, all other bits are ignored.

### VRAM bank 1

VRAM bank 1 is split like VRAM bank 0 ; 8000-97FF also stores tiles
(just like in bank 0), which can be accessed the same way as (and at the
same time as) bank 0 tiles. 9800-9FFF contains the attributes for the
corresponding Tile Maps.

Reading from this register will return the number of the currently
loaded VRAM bank in bit 0, and all other bits will be set to 1.

VRAM Sprite Attribute Table (OAM)
---------------------------------

Gameboy video controller can display up to 40 sprites either in 8x8 or
in 8x16 pixels. Because of a limitation of hardware, only ten sprites
can be displayed per scan line. Sprite patterns have the same format as
BG tiles, but they are taken from the Sprite Pattern Table located at
\$8000-8FFF and have unsigned numbering.

Sprite attributes reside in the Sprite Attribute Table (OAM - Object
Attribute Memory) at \$FE00-FE9F. Each of the 40 entries consists of
four bytes with the following meanings:

### Byte0 - Y Position

Specifies the sprites vertical position on the screen (minus 16). An
off-screen value (for example, Y=0 or Y\>=160) hides the sprite.

### Byte1 - X Position

Specifies the sprites horizontal position on the screen (minus 8). An
off-screen value (X=0 or X\>=168) hides the sprite, but the sprite still
affects the priority ordering - a better way to hide a sprite is to set
its Y-coordinate off-screen.

### Byte2 - Tile/Pattern Number

Specifies the sprites Tile Number (00-FF). This (unsigned) value selects
a tile from memory at 8000h-8FFFh. In CGB Mode this could be either in
VRAM Bank 0 or 1, depending on Bit 3 of the following byte. In 8x16
mode, the lower bit of the tile number is ignored. IE: the upper 8x8
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

When sprites overlap, the highest priority one will appear above all
others, etc. (Thus, no Z-fighting). In CGB mode, the first sprite in OAM
(\$FE00-\$FE03) has the highest priority, and so on. In Non-CGB mode,
the smaller the X coordinate, the higher the priority. The tie breaker
(same X coordinates) is the same priority as in CGB mode.

Only 10 sprites can be displayed on any one line. When this limit is
exceeded, the lower sprites (in their order in OAM, \$FE00-\$FE03 being
the highest) won\'t be displayed. To keep unused sprites from affecting
onscreen sprites, set their Y coordinate to Y = 0 or Y =\> 160 (144 +
16) (Note : Y \<= 8 also works if sprite size is set to 8x8). Just
setting the X coordinate to X = 0 or X =\> 168 (160 + 8) on a sprite
will hide it, but it will still affect other sprites sharing the same
lines.

If using [BGB](BGB "wikilink"), in the VRAM viewer - OAM tab, hover your
mouse over the small screen to highlight the sprites on a line. Sprites
hidden due to the limitation will be highlighted in red.

### Writing Data to OAM Memory

The recommended method is to write the data to normal RAM first, and to
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

For this reason the program should verify if VRAM/OAM is accessible
before actually reading or writing to it. This is usually done by
reading the Mode Bits from the STAT Register (FF41). When doing this (as
described in the examples below) you should take care that no interrupts
occur between the wait loops and the following memory access - the
memory is guaranteed to be accessible only for a few cycles directly
after the wait loops have completed.

### VRAM (memory at 8000h-9FFFh) is accessible during Mode 0-2

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

### OAM (memory at FE00h-FE9Fh) is accessible during Mode 0-1

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

When the display is disabled, both VRAM and OAM are accessible at any
time. The downside is that the screen is blank (white) during this
period, so that disabling the display would be recommended only during
initialization.

