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

SGB hardware can be detected by examining the initial value of the C
register directly after startup: a value of $14 indicates SGB or SGB2
hardware. It is also possible to separate between SGB and SGB2 by
examining the initial value of the A register directly after startup.
Note that the DMG and MGB share initial A register values with the SGB
and SGB2 respectively.

Console | A Register | C Register
--------|------------|------------
DMG     | $01        | $13
SGB     | $01        | $14
MGB     | $FF        | $13
SGB2    | $FF        | $14
CGB     | $11        | $00
AGB     | $11        | $00

For initial register values on all systems, see the table of all [CPU
registers after power-up](<#CPU registers>).

The SGB2 doesn't have any extra features which'd require separate SGB2
detection except for curiosity purposes, for example, the game "Tetris
DX" chooses to display an alternate SGB border on SGB2s.

Only the SGB2 contains a link port.

SGB hardware has traditionally been detected by sending [`MLT_REQ` commands](<#SGB Command $11 — MLT_REQ>), but this
method is more complicated and slower than checking the value of the A
and C registers after startup. The `MLT_REQ` command enables two (or four)
joypads; a normal handheld Game Boy will ignore this command, but an SGB
will return incrementing joypad IDs each time when deselecting keypad
lines ([see `MLT_REQ` description](<#Reading Multiple Controllers (Joypads)>)). The joypad state/IDs can
then be read out several times, and if the IDs are changing, then it is
an SGB (a normal Game Boy would typically always return $0F as the ID).
Finally, when not intending to use more than one joypad, send another
`MLT_REQ` command in order to disable the multi-controller mode.
Detection works regardless of how many joypads are physically connected
to the SNES. However, unlike the C register method, this detection works only when
SGB functions [are unlocked from the cartridge header](<#Cartridge Header>).
