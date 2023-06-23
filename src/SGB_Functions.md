# SGB Description

## General Description

The Super Game Boy (SGB) is an adapter cartridge that allows
to play Game Boy games on a SNES (Super Nintendo Entertainment System)
gaming console. In detail, you plug the Game Boy cartridge into the SGB
cartridge, then plug the SGB cartridge into the SNES, and then connect
the SNES to your TV Set. In result, games can be played and viewed on
the TV Set, and are controlled by using the SNES joypad(s).

## More Technical Description

The SGB cartridge contains a Game Boy system on chip, with its normal
CPU and video and sound controller. It also has a bridge circuit, the ICD2, to
translate joypad input, video signal, and control packets between the
SNES and GB as well as a system software ROM that runs on the SNES.
The system software forwards button presses from the controllers and
sends the video signal to the TV set, giving both the player and the
game a small amount of control over the appearance.

## Normal Monochrome Games

Any Game Boy games which have been designed for monochrome handheld
Game Boy systems will work with the SGB hardware as well. The SGB will
apply a four color palette to these games by replacing the normal four
shades of gray. The 160×144 pixel game screen is displayed in the middle of
the 256×224 pixel SNES screen (the unused area is filled by a screen
border bitmap). The user may access built-in menus, allowing to change
color palette data, to select between several pre-defined borders, etc.

Games that have been designed to support SGB functions may also access
the following additional features:

## Colorized Game Screen

There's limited ability to colorize the game screen by assigning custom
color palettes to each 20×18 display characters, however, this works
mainly for static display data such like title screens or status bars,
the 20×18 color attribute map cannot be scrolled, and it is not possible
to assign separate colors to moveable foreground objects (sprites), so that
animated screen regions will be typically restricted to using a single
palette of four colors only.

## SNES Foreground Sprites

Up to 24 additional 16-color objects of 8×8 or 16×16 pixels, can
be displayed. When replacing (or just overlaying) the normal Game Boy
objects by SNES objects it'd be thus possible to display objects with other
colors than normal background area. This method doesn't appear to be
very popular, even though it appears to be quite easy to implement,
however, the bottommost character line of the game screen will be masked
out because this area is used to transfer object positions to the SNES.
(Later versions of the SGB system software remove much of this object enhancement.)

## The SGB Border

The possibly most popular and most impressive feature is to replace the
default SGB screen border by a custom bitmap which is stored in the game
cartridge.  A border can be much more colorful than the game screen.

## Multiple Joypads

Up to four joypads can be conected to the SNES, and SGB software may
read-out each of these joypads separately, allowing up to four players
to play the same game simultaneously. Unlike for multiplayer handheld
games, this requires only one game cartridge and only one SGB/SNES, and
no link cables are required, the downside is that all players must share
the same display screen.

## Sound Functions

Alongside normal Game Boy sound, a number of digital sound effects is
pre-defined in the SNES BIOS, these effects may be accessed quite
easily. Programmers whom are familiar with SNES sounds may also access
the SNES sound chip, or use the SNES "[Kankichi]" sequencer engine
directly in order to produce other sound effects or music.

[Kankichi]: https://sneslab.net/wiki/N-SPC_Engine

## Taking Control of the SNES CPU

Finally, it is possible to write program code or data into SNES memory,
and to execute such program code by using the SNES CPU.

## SGB System Clock

Because the SGB is synchronized to the SNES CPU, the Game Boy system
clock is directly chained to the SNES system clock. In result, the
Game Boy CPU, video controller, timers, and sound frequencies will be all
operated approx 2.4% faster than handheld systems. Basically, this
should be no problem, and the game will just run a little bit faster.
However sensitive musicians may notice that sound frequencies are a bit
too high, particularly in programs that use GB sound alongside Kankichi.
Programs that support SGB functions may avoid this effect by
reducing frequencies of Game Boy sounds when having detected SGB
hardware. Also, "PAL version" SNES models which use a
50Hz display refresh rate (rather than 60Hz) result in
respectively slower Game Boy timings.

- NTSC SGB: 21.477 MHz master clock, 4.2955 MHz GB clock, 2.41% fast
- PAL SGB: 21.281 MHz master clock, 4.2563 MHz GB clock, 1.48% fast
- NTSC SGB2: Separate 20.972 MHz crystal, correct speed
