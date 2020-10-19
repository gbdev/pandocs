
# Overview

The Game Boy outputs to a 160x144 pixel LCD, but this hides a decently complex
system used to facilitate rendering.

## Tiles

Similarly to other retro systems, pixels are usually not manipulated
individually, as this would be expensive CPU-wise. Instead, pixels are grouped
in 8x8 squares, called *tiles* (or sometimes "patterns"), which are essentially
the base unit as far as Game Boy graphics are concerned.

## Palettes

A tile does not encode color information directly. Instead, a tile assigns a
*color ID* to each of its pixels, thus ranging from 0 to 3. (For this reason,
Game Boy graphics are called *2bpp*, or "2 bits per pixel".) These color IDs
are then combined with a *palette* to essentially colorize it.

Conceptually, palettes are simply an array of colors, 4 in the Game Boy's case.
The way a palette is stored differs between monochrome and Color Game Boys.
When applied to a tile, each color ID is used as an index into that array of
colors, which then gets sent to the LCD.

Palettes are useful, for example, for quickly flashing some graphics (damage,
invunerability, thunderstorm, etc.), for fading the screen, famously for
palette swaps, and more.

## Layers

The Game Boy has three "layers", from bottom to top: the background, the window,
and sprites. Some features and behaviors break this abstraction,
but it works for the most part.

### Background

The background is composed of what's called a *tilemap*. A tilemap is simply a
large grid of tiles. However, tiles aren't directly written to tilemaps;
instead, tilemaps contain the *ID* of tiles—a number that uniquely identifies a
given tile. This makes reusing tiles very cheap, both in CPU time and in
required memory space, and is the main mechanism that helps working around the
paltry 8 KiB of video RAM.

The background can be made to scroll as a whole, simply by writing to two
hardware registers; this makes scrolling very cheap. If a game had
to update each pixel individually, it would have to redraw the whole screen.

### Window

The window is sort of a second background layer on top of the background.
It is fairly limited: it has no transparency, it's always a
rectangle and only the position of the top-left pixel can be controlled.

Possible usage include a fixed status bar in an otherwise scrolling game (e.g.
*Super Mario Bros. 3*).

### Sprites

The background layer is useful for elements scrolling as a whole; but
it's impractical for objects that need to move separately, such as the player.
*Sprites* are designed to fill this gap: they allow displaying tiles freely
anywhere on the screen. Sprites are often called *objects* when talking about
the hardware functionality.

Sprites are fairly small (8x8 or 8x16), so several hardware objects are often
combined to produce a larger graphic, which is often referred to as a *sprite*.
