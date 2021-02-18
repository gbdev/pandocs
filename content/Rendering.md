
# Overview

The Game Boy outputs to a 160x144 pixel LCD, using a quite complex
mechanism to facilitate rendering.

::: warning
Sprites/graphics terminology can vary a lot among different platforms, consoles, 
users and communities. You may be familiar with slightly different definitions.
Keep also in mind that some of the definitions refers to lower (hardware) tools
and some others to higher abstractions concepts.
:::

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

A palette consists in a array of colors, 4 in the Game Boy's case.
Palettes are stored differently in monochrome and color versions of the console.
When applied to a tile, each color ID is used as an index to select the color
in the palette array, which then gets sent to the LCD.

Modifying palettes enables graphical effects such as quickly flashing some graphics (damage,
invulnerability, thunderstorm, etc.), fading the screen, "palette swaps", and more.

## Layers

The Game Boy has three "layers", from bottom to top: the background, the window,
and the sprites. Some features and behaviors break this abstraction,
but it works for the most part.

### Background

The background is composed of a *tilemap*. A tilemap is a
large grid of tiles. However, tiles aren't directly written to tilemaps,
they merely contain references to the tiles.
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

### Objects

The background layer is useful for elements scrolling as a whole, but
it's impractical for objects that need to move separately, such as the player.

The *objects* layer is designed to fill this gap: it allows displaying tiles anywhere
on the screen.

*Objects* are made of 1 or 2 stacked tiles (8x8 or 8x16 pixels).

::: tip NOTE
Several objects can be combined (in that case they are called *metasprites*) to draw
a larger graphical element, usually called "sprite". A sprite is a collection of tiles
used to display one frame of a game character's animation.
:::

To summarise:

- **Tile**, an 8x8-pixel chunk of graphics.
- **Object**, an entry in object attribute memory, composed of 1 or 2
tiles. Independent from the background.
