# Graphics Overview

The Game Boy outputs graphics to a 160×144 pixel LCD, using a quite complex
mechanism to facilitate rendering.

:::warning Terminology

Sprites/graphics terminology can vary a lot among different platforms, consoles,
users and communities. You may be familiar with slightly different definitions.
Keep also in mind that some definitions refer to lower (hardware) tools
and some others to higher abstractions concepts.

:::

## Tiles

Similarly to other retro systems, pixels are not manipulated
individually, as this would be expensive CPU-wise. Instead, pixels are grouped
in 8×8 squares, called _tiles_ (or sometimes "patterns" or "characters"), often considered as
the base unit in Game Boy graphics.

A tile does not encode color information. Instead, a tile assigns a
_color ID_ to each of its pixels, ranging from 0 to 3. For this reason,
Game Boy graphics are also called _2bpp_ (2 bits per pixel). When a tile is used
in the Background or Window, these color IDs are associated with a _palette_. When
a tile is used in an object, the IDs 1 to 3 are associated with a palette, but
ID 0 means transparent.

## Palettes

A palette consists of an array of colors, 4 in the Game Boy's case.
Palettes are stored differently in monochrome and color versions of the console.

Modifying palettes enables graphical effects such as quickly flashing some graphics (damage,
invulnerability, thunderstorm, etc.), fading the screen, "palette swaps", and more.

## Layers

The Game Boy has three "layers", from back to front: the Background, the Window,
and the Objects. Some features and behaviors break this abstraction,
but it works for the most part.

### Background

The background is composed of a _tilemap_. A tilemap is a
large grid of tiles. However, tiles aren't directly written to tilemaps,
they merely contain references to the tiles.
This makes reusing tiles very cheap, both in CPU time and in
required memory space, and it is the main mechanism that helps work around the
paltry 8 KiB of video RAM.

The background can be made to scroll as a whole, writing to two
hardware registers. This makes scrolling very cheap.

### Window

The window is sort of a second background layer on top of the background.
It is fairly limited: it has no transparency, it's always a
rectangle and only the position of the top-left pixel can be controlled.

Possible usage include a fixed status bar in an otherwise scrolling game (e.g.
_Super Mario Land 2_).

### Objects

The background layer is useful for elements scrolling as a whole, but
it's impractical for objects that need to move separately, such as the player.

The _objects_ layer is designed to fill this gap: _objects_ are made of 1 or 2 stacked tiles (8×8 or 8×16 pixels)
and can be displayed anywhere on the screen.

:::tip NOTE

Several objects can be combined (they can be called _metasprites_) to draw
a larger graphical element, usually called "sprite". Originally, the term "sprites"
referred to fixed-sized objects composited together, by hardware, with a background.
Use of the term has since become more general.

:::

To summarise:

- **Tile**, an 8×8-pixel chunk of graphics.
- **Object**, an entry in object attribute memory, composed of 1 or 2
  tiles. Can be moved independently of the background.
