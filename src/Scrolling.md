
# LCD Position and Scrolling

These registers can be accessed even during Mode 3, but they have no
effect until the end of the current scanline.

## FF42 - SCY (Scroll Y) (R/W), FF43 - SCX (Scroll X) (R/W)

Those specify the top-left coordinates of the visible 160x144 pixel area within the
256x256 pixels BG map. Values in the range 0-255 may be used.

## FF44 - LY (LCD Y Coordinate) (R)

LY indicates the current horizontal line, which might be about to be drawn,
being drawn, or just been drawn. LY can hold any value from 0 to 153.
The values from 144 to 153 indicate the VBlank period.

## FF45 - LYC (LY Compare) (R/W)

The Game Boy permanently compares the value of the LYC and LY registers.
When both values are identical, the "LYC=LY" flag in the STAT register
is set, and (if enabled) a STAT interrupt is requested.

## FF4A - WY (Window Y Position) (R/W), FF4B - WX (Window X Position + 7) (R/W)

Specify the top-left coordinates of the Window. (The Window is an
alternate background area which can be displayed above of the normal
background. OBJs (sprites) may be still displayed above or behind the
Window, just as for normal BG.)

The Window is visible (if enabled) when both coordinates are in the ranges
WX=0..166, WY=0..143 respectively. Values WX=7, WY=0 place the Window at the
top left of the screen, completely covering the background.

WX values 0-6 and 166 are unreliable due to hardware bugs. If WX is set
to 0, the window will "stutter" horizontally when SCX changes.
(Depending on SCX modulo 8, behavior is a little complicated so you
should try it yourself.)

::: tip MID-FRAME QUIRKS

While the Windows should work as just mentioned, writing to WX, WY etc. mid-frame shows a more articulated behavior.

For the window to be displayed on a scanline:

- __WY condition was triggered__: i.e. at some point in this frame the value of WY was equal to LY (checked at the start of Mode 2 only)
- __WX condition was triggered__: i.e. the current X coordinate being rendered + 7 was equal to WX
- Window enable bit in LCDC is set

If the WY condition has already been triggered and at the the start of a row the window enable bit was set
then resetting that bit before the WX condition gets triggered on that row yields a nice window glitch pixel where the window would have been activated.

:::
