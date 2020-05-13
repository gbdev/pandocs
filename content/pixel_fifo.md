# Pixel FIFO
The following information is construed from [SameBoy's](https://www.github.com/liji32/sameboy)
implementation of the pixel FIFO.

Before we get started, all references to a cycle are meant as T-cycles
(4.19 MHz) and cycle counts are doubled on CGB in double speed mode. Also
when stating that a certain action "lengthens mode 3" this means that
mode 0 (hblank) is shortened.

### FIFO info
FIFO stands for First In, First Out. The first pixel to be pushed to the
FIFO is the first pixel to be popped off. In theory that sounds great,
in practice there are a lot of intricacies.

The FIFOs are manipulated only during mode 3 (pixel transfer). The FIFOs
are both cleared at the beginning of mode 3.

Each pixel in the FIFO has four properties: color, palette, sprite
priority and background priority. Color is a value between 0 and 3.
Palette is a value between 0 and 7 on CGB and 0 or 1 on DMG. Sprite
priority is 0 on DMG and OAM index on CGB. Background priority holds the
[OBJ-to-BG Priority](#vram-sprite-attribute-table-oam) bit.

Notes:
- There are two pixel FIFOs. One for background pixels and one for OAM
(sprite) pixels.
- These two FIFOs are not shared. They are two independent of each
other.
- The two FIFOs are mixed only when popping items. Sprites take priority
unless they're transparent (color 0).
- The FIFOs can each hold up to 16 pixels at a time.
- FIFOs are manipulated only during mode 3 (pixel transfer)

### FIFO Pixel Fetcher
The pixel fetcher has 8 steps. The order of the steps are as follows:

- Sleep
- Get tile
- Sleep
- Get tile data low
- Sleep
- Get tile data high
- Push
- Push

### FIFO Fetcher steps
#### Get Tile:
This step determines which background/window tile to fetch pixels from.

The WX register can affect which background/window map to use. If LCDC.3
is enabled and the current tile is not a window tile, then tile map
$9C00 is used, otherwise $9800 is used.

If LCDC.6 is enabled and the current tile is a window tile then tile
map $9C00 is used, otherwise $9800 is used.

The fetcher keeps track of which X and Y coordinate of the tile it's on:

If the current tile is a window tile, the X coordinate for the window
tile is used, otherwise the following formula is used to calculate
the X coordinate: ((SCX / 8) + fetcher's X coordinate) & $1F. Because of
this formula, fetcherX can be between 0 and 31.

If the current tile is a window tile, the Y coordinate for the window
tile is used, otherwise the following formula is used to calculate
the Y coordinate: currentScanLine + SCY. Because of this formula,
fetcherY can be between 0 and 415.

The fetcher's X and Y coordinate can then be used to get the tile from
VRAM. However, if the PPU's access to VRAM is [blocked](#vram-access)
then the value for the tile is read as $FF.

CGB can access both tile index and the attributes in the same clock
cycle. This probably means the CGB has a 16-bit data bus for the VRAM.

Once all that is done, the fetcher is advanced to the Sleep state.

#### Get Tile Data Low:
Check LCDC.4 for which tilemap to use. At this step CGB also needs to
check which VRAM bank to use and if the tile is flipped vertically.
Once the tilemap, VRAM and vertical flip is calculated the tile data
is retrieved from VRAM. However, if the PPU's access to VRAM is
[blocked](#vram-access) then the tile data is read as $FF.

The tile data retrieved in this step will be used in the push steps.

Once this is done the fetcher state is advanced to the final Sleep step.

#### Get Tile Data High:
Same as Get Tile Data Low except the tile address is incremented by 1.

The tile data retrieved in this step will be used in the push steps.

This also pushes a row of background pixels to the FIFO. This extra push
is not part of the 8 steps, meaning there's 3 total chances to push
pixels to the background FIFO every time the fetcher steps are
performed.

Once this is done the fetcher state is advanced to the first Push step.

#### Push:
Pushes a row of background pixels to the FIFO. A "row" of pixels is 8
pixels from the tile to be rendered based on the X and Y coordinates
calculated in the previous steps.

If the fetcher is in the first Push state, the background map index
is incremented before the pixels are pushed to the background FIFO.
Pixels are only pushed to the background FIFO if it's empty.

This is where the tile data retrieved in the two Tile Data steps will
come in handy. Depending on if the tile is flipped horizontally the
pixels will be pushed to the background FIFO differently. If the tile
is flipped horizontally the pixels will be pushed LSB first. Otherwise
they will be pushed MSB first.

The fetcher state is either advanced to the next Push step or reset
depending on if it's in on step 7 or step 8.

#### Sleep:
Do nothing and advance the fetcher to the next step.

### VRAM Access
At various times during PPU operation read access to VRAM is blocked and
the value read is $FF:
- In double speed mode (beginning of mode 3)
- If not CGB (beginning of mode 3)
- When searching OAM and index 37 is reached, if not CGB (mode 2)
- After switching from mode 2 (oam search) to mode 3 (pixel transfer)

At various times during PPU operation read access to VRAM is restored:
- LCD turning off
- The beginning of mode 0 (hblank)
- Not in double speed mode (beginning of mode 3)
- When searching OAM and index 37 is reached, if CGB (mode 2)
- When switching from mode 3 to mode 0, if not double speed mode

NOTE: These conditions are checked only when entering STOP mode and the
PPU's access to VRAM is always restored upon leaving STOP mode.

### Mode 3 Operation
As stated before the pixel FIFO only operates during mode 3 (pixel
transfer). At the beginning of mode 3 both the background and OAM FIFOs
are cleared.

#### The Window
When rendering the window the background FIFO is cleared and the fetcher
is reset to step 1. When WX is 0 and the SCX & 7 > 0 mode 3 is shortened
by 1 cycle.

**TODO: ask liji about the condition in display.c:1291. I can't think
or find any situation where the condition is true and all the games
I tested didn't trigger that block**

#### Sprites
**TODO: ask liji about condition in display.c:1298 to make sure I
understand it correctly**

**TODO: make sure this accurate**
The following is performed while there are sprites on the current
scanline, if LCDC.1 is enabled (this condition is ignored on CGB) and
current X coordinate of the scanline has a sprite on it. If those
conditions are not met, the process described in [Sprite Fetch Abortion](#sprite-fetch-abortion)
is performed.

At this point, if the [fetcher](#fifo-pixel-fetcher) is not at step 5 or
the background FIFO is empty, the fetcher is advanced one step until one
of those conditions is met. Advancing the fetcher one step here lengthens
mode 3 by 1 cycle. This process may be [aborted](#sprite-fetch-abortion)
after the fetcher has advanced a step.

At this point if SCX & 7 > 0 and there is a sprite at X coordinate 0 of
the current scanline then mode 3 is lengthened. The amount of cycles this
lengthens mode 3 by is whatever the lower 3 bits of SCX are. After this
penalty is applied object fetching may be aborted. Note that the timing
of the penalty is not confirmed. It may happen before or after waiting
for the fetcher. More research needs to be done.

After checking for sprites at X coordinate 0 the fetcher is advanced two
steps. The first advancement lengthens mode 3 by 1 cycle and the second
advancement lengthens mode 3 by 3 cycles. After each fetcher advancement
there is a chance for a sprite fetch abortion to occur.

The lower address for the row of pixels of target object tile is now
retrieved and lengthens mode 3 by 1 cycle. Once the address is retrieved
this is the last chance for sprite fetch abortion to occur. An extra
cycle is taken here for... reasons.

The upper address for the target object tile is now retrieved and does
not shorten mode 3. Maybe the extra cycle before was for this?

At this point [VRAM Access](#vram-access) is checked for the lower and
upper addresses for the target object. The target object is now pushed
onto the OAM FIFO. Before any mixing is done, if the OAM FIFO doesn't
have at least 8 pixels in it, it is filled with white pixels. Once this
is done each pixel of the target object row is checked. On CGB,
horizontal flip is checked here. If the target object pixel is not white
and the pixel in the OAM FIFO *is* white or if the pixel in the OAM FIFO
has higher priority than the target object's pixel then the pixel in the
OAM FIFO is replaced with the target object's properties.

Once the target object's pixels have been merged with the OAM FIFO it's
time to [render a pixel](#pixel-rendering).

At this point the same process described in Sprite Fetch Abortion is
performed.

Everything in this section is repeated for every sprite on the current
scanline unless it was decided that fetching should be aborted or the
X coordinate is 160.

#### Pixel Rendering
This is where the background FIFO and OAM FIFO are mixed. There are
conditions where the either a background pixel or a sprite pixel will
have display priority.

If there are pixels in the background and OAM FIFOs then a pixel is
popped off each. If the OAM pixel is not transparent and LCDC.1 is
enabled then the OAM pixel's background priority property is used if it's
higher or the same as the background pixel's background priority.

Pixels won't be pushed to the LCD if there is nothing in the background
FIFO or the X coordinate for the current scanline is greater than 159.

If LCDC.0 is disabled then the background is disabled on DMG and the
background pixel won't have priority on CGB. When the background pixel
is disabled the pixel color value will be 0, otherwise the color will be
whatever color pixel was popped off the background FIFO. When the pixel
is not 0 and the background pixel has priority the sprite pixel will not
be drawn.

At this point, on DMG, the color of the pixel is retrieved from the BGP
register. On CGB when **TODO: explain cgb_palettes_ppu_blocked**.

When a sprite pixel has priority the color value is retrieved from the
popped pixel from the OAM FIFO. On DMG the color for the pixel is
retrieved from either the OBP1 or OBP0 register depending on the pixel's
palette property. If the palette property is 1 then OBP1 is used,
otherwise OBP0 is used. On CGB when **TODO: explain cgb_palettes_ppu_blocked**.

The pixel is then pushed to the LCD.

#### Sprite Fetch Abortion
If LCDC.1 is disabled while the PPU is fetching an object from OAM the
fetch is aborted and mode 3 is lengthened by the amount of cycles the
previous instruction took plus the residual cycles left for the PPU to
process. **TODO: does that make sense???**

When OAM fetching is aborted a pixel is [rendered](#pixel-rendering), the
[fetcher](#fifo-pixel-fetcher) is advanced one step. If the X coordinate
being drawn is 160 the PPU stops processing sprites. Otherwise mode 3 is
lengthened by 1 cycle and the PPU will continue to process sprites until
there are no more for the current line.
