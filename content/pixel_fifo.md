# Game Boy Pixel FIFO
The following information is construed from [SameBoy's](https://www.github.com/liji32/sameboy)
implementation of the pixel FIFO.

Before we get started, all references to a cycle are meant as T-cycles
(4.19 MHz) and cycle counts are doubled on CGB in double speed mode.

### FIFO info
FIFO stands for First In, First Out. The first pixel to be pushed to the
FIFO is the first pixel to be popped off. In theory that sounds great,
in practice there are a lot of intricacies.

The FIFOs are manipulated only during mode 3 (pixel transfer). The FIFOs
are both cleared at the beginning of mode 3.

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
The pixel fetcher has 8 steps, each taking 1 cycle(?). The order
of the steps are as follows:

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
9C00h is used, otherwise 9800h is used.

If LCDC.6 is enabled and the current tile is a window tile then tile
map 9C00h is used, otherwise 9800h is used.

The fetcher keeps track of which X and Y coordinate of the tile it's on:

If the current tile is a window tile, the X coordinate for the window
tile is used, otherwise the following formula is used to calculate
the X coordinate: ((SCX / 8) + fetcher's X coordinate) & 1Fh. Because of
this formula, fetcherX can be between 0 and 31.

If the current tile is a window tile, the Y coordinate for the window
tile is used, otherwise the following formula is used to calculate
the Y coordinate: currentScanLine + SCY. Because of this formula,
fetcherY can be between 0 and 415.

The fetcher's X and Y coordinate can then be used to get the tile from
VRAM. However, if the PPU's access to VRAM is [blocked](#vram-access)
then the value for the tile is read as FFh.

CGB can access both tile index and the attributes in the same clock
cycle. This probably means the CGB has a 16-bit data bus for the VRAM.

Once all that is done, the fetcher is advanced to the Sleep state.

#### Get Tile Data Low:
Check LCDC.4 for which tilemap to use. At this step CGB also needs to
check which VRAM bank to use and if the tile is flipped vertically.
Once the tilemap, VRAM and vertical flip is calculated the tile data
is retrieved from VRAM. However, if the PPU's access to VRAM is
[blocked](#vram-access) then the tile data is read as FFh.

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
At various times during PPU operation access to VRAM is blocked:
- In double speed mode (beginning of mode 3)
- If not CGB (beginning of mode 3)
- When searching OAM and index 37 is reached, if not CGB (mode 2)
- After switching from mode 2 (oam search) to mode 3 (pixel transfer)

At various times during PPU operation access to VRAM is restored:
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
is reset to step 1. When WX is 0 and the lower 3 bits of SCX are set
mode 3 is shortened by 1 cycle.

#### Sprites
When the sprite enabled bit is off, this process is skipped entirely on
the DMG, but not on the CGB. On the CGB, this bit is checked only when
the pixel is actually popped from the FIFO.

**TODO: figure out the todo in display.c line 1277**

If sprite pixels are to be rendered on the current scanline and LCDC.1
is enabled and **TODO: display.c:1303**
This is repeated for every sprite on the current scanline.

### TODO
- Timing
  - I know each SCX shift takes 1 clock, but I don't know when
that delay is applied.
  - How long does a fetcher step take?

> SCX changes the length of Mode 3  
> When the scanline begins, the pixel FIFO is shifted SCX % 8 times  
> Each shift takes one clock

~ ISSOtm