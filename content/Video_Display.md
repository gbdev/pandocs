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
bytes define Color 0-3 of Palette 0 (BGP0), and so on for BGP1-7. Bit
0-5 Index (00-3F)

` Bit 7     Auto Increment  (0=Disabled, 1=Increment after Writing)`

Data can be read/written to/from the specified index address through
Register FF69. When the Auto Increment Bit is set then the index is
automatically incremented after each <write> to FF69. Auto Increment has
no effect when <reading> from FF69, so the index must be manually
incremented in that case.

### FF69 - BCPD/BGPD - CGB Mode Only - Background Palette Data

This register allows to read/write data to the CGBs Background Palette
Memory, addressed through Register FF68. Each color is defined by two
bytes (Bit 0-7 in first byte). Bit 0-4 Red Intensity (00-1F)

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

