
# Overview

The Game Boy outputs to a 160x144 pixel LCD, using a quite complex
mechanism to facilitate rendering.

## Tiles

Similarly to other retro systems, pixels are usually not manipulated
individually, as this would be expensive CPU-wise. Instead, pixels are grouped
in 8x8 squares, called *tiles* (or sometimes "patterns"), often considered as
the base unit in Game Boy graphics.

## Palettes

A tile does not encode color information. Instead, a tile assigns a
*color ID* to each of its pixels, ranging from 0 to 3. For this reason,
Game Boy graphics are also called *2bpp*, 2 bits per pixel. These color IDs
are then associated with a *palette*.

A palettes consists in a array of colors, 4 in the Game Boy's case.
Palettes are stored differently in monoschrome and color versions of the console.
When applied to a tile, each color ID is used as an index to select the color
in the palette array, which then gets sent to the LCD.

Modifying palettes enables effects such as quickly flashing some graphics (damage,
invunerability, thunderstorm, etc.), fading the screen, "palette swaps", and more.

## Layers

The Game Boy has three "layers", from bottom to top: the background, the window,
and the sprites. Some features and behaviors break this abstraction,
but it works for the most part.

### Background

The background is composed of a *tilemap*. A tilemap is a
large grid of tiles. However, tiles aren't directly written to tilemaps;
instead, tilemaps contain unique references to the tiles.
This makes reusing tiles very cheap, both in CPU time and in
required memory space, and it is the main mechanism that helps working around the
paltry 8 KiB of video RAM.

The background can be made to scroll as a whole, writing to two
hardware registers. This makes scrolling very cheap.

### Window

The window is sort of a second background layer on top of the background.
It is fairly limited: it has no transparency, it's always a
rectangle and only the position of the top-left pixel can be controlled.

Possible usage include a fixed status bar in an otherwise scrolling game (e.g.
*Super Mario Bros. 3*).

### Sprites

The background layer is useful for elements scrolling as a whole, but
it's impractical for objects that need to move separately, such as the player.

The *sprites* layer is designed to fill this gap: it allows displaying tiles anywhere
on the screen.

Game Boy sprites (or *hardware objects*) are made of 1 or 2 stacked tiles,
(8x8 or 8x16 pixels), and several of them can be combined (in *metasprites*) to draw a larger 
graphical element, usually called a "sprite".

To summerise:

- **Tile**, an 8x8-pixel chunk of graphics.
- **Hardware Object**, an entry in object attribute memory, composed of 1 or 2 tiles.
- **Metasprite** a set of several sprites moved as a unit to display a sprite.
- **Sprite** (or cel), a set of tiles making up on frame of animation, diplayed using metasprites.