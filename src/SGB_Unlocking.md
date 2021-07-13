# Unlocking and Detecting SGB Functions

## Cartridge Header

SGB games are required to have a cartridge header with Nintendo and
proper checksum just as normal Game Boy games. Also, two special entries
must be set in order to unlock SGB functions:

- 146h - SGB Flag - Must be set to 03h for SGB games
- 14Bh - Old Licensee Code - Must be set 33h for SGB games

When these entries aren't set, the game will still work just like all
"monochrome" Game Boy games, but it cannot access any of the special
SGB functions.

## Detecting SGB hardware

The recommended detection method is to send a MLT_REQ command which
enables two (or four) joypads. A normal handheld Game Boy will ignore
this command, a SGB will now return incrementing joypad IDs each time
when deselecting keyboard lines (see MLT_REQ description for details).
Now read-out joypad state/IDs several times, and if the ID-numbers are
changing, then it is a SGB (a normal Game Boy would typically always
return 0Fh as ID). Finally, when not intending to use more than one
joypad, send another MLT_REQ command in order to re-disable the
multi-controller mode. Detection works regardless of whether and how
many joypads are physically connected to the SNES. However, detection
works only when having unlocked SGB functions in the cartridge header,
as described above.

## Separating between SGB and SGB2

It is also possible to separate between SGB and SGB2 models by examining
the inital value of the accumulator (register A) directly after startup.

Value | Console
------|---------
 $01  | SGB or original Game Boy (DMG)
 $FF  | SGB2 or Game Boy Pocket
 $11  | CGB or GBA

Because values 01h and FFh are shared for both handhelds and SGBs, it is
still required to use the above MLT_REQ detection procedure. As far as
I know the SGB2 doesn't have any extra features which'd require
separate SGB2 detection except for curiosity purposes, for example, the
game "Tetris DX" chooses to display an alternate SGB border on SGB2s.

Only the SGB2 contains a link port.
