# Unlocking and Detecting SGB Functions

## Cartridge Header

SGB games are required to have a cartridge header with Nintendo logo and
proper checksum just as normal Game Boy games. Also, two special entries
must be set in order to unlock SGB functions:

- [SGB flag](<#0146 — SGB flag>): Must be set to $03 for SGB games
- [Old licensee code](<#014B — Old licensee code>): Must be set to $33 for SGB games

When these entries aren't set, the game will still work just like all
"monochrome" Game Boy games, but it cannot access any of the special
SGB functions.

## Detecting SGB hardware

SGB hardware can be detected by examining the [boot-up value](<#CPU registers>) of the CPU's `c` register directly after startup: a value of $14 indicates SGB or SGB2 hardware.
The `a` register can then be used to distinguish between SGB and SGB2:

Console | A Register | C Register
--------|------------|------------
DMG     | $01        | $13
SGB     | $01        | $14
MGB     | $FF        | $13
SGB2    | $FF        | $14
CGB     | $11        | $00
AGB     | $11        | $00

Note that this detects whether running on SGB *hardware* (and, for example, its faster master clock), not if SGB enhancements are *enabled*!
There are a few (rare) ways that a SGB-enhanced game can be run on SGB without enhancements being enabled; in particular, the SGB2 can be booted in “DMG Mode” by holding <kbd>L</kbd>+<kbd>Left</kbd> ([manual, in Japanese][sgb2pamphlet]) wherein SGB enhancements are forcibly disabled.

If you wish to support those cases, enhancements can be probed by attempting to use one that provides feedback to the GB side; the traditional way of doing so is to use [`MLT_REQ` commands](<#SGB Command $11 — MLT_REQ>).
The `MLT_REQ` command enables two (or four) joypads; a normal handheld Game Boy will ignore this command, but an SGB
will return incrementing joypad IDs each time when deselecting keypad lines ([see `MLT_REQ` description](<#Reading Multiple Controllers (Joypads)>)), regardless of how many controllers are physically connected to the SNES.
The joypad state/IDs can then be read out several times, and if the IDs are changing, then it is an SGB (a normal Game Boy would typically always return $0F as the ID).

Finally, when not intending to use more than one joypad, send another `MLT_REQ` command in order to disable the multi-controller mode.

The SGB2 doesn't have any extra features which would require separate SGB2 detection, but games can nonetheless choose to get fancy: for example, *Tetris DX* chooses to display an alternate SGB border on SGB2.
Only the SGB2 has a link port, but attempting to use [link cable communication] on SGB merely results in it acting as if no cable was connected, so it's fine not to specially handle it.

[sgb2pamphlet]: imgs/sgb2_dmg_mode_pamphlet.png
[link cable communication]: <#Serial Data Transfer (Link Cable)>
