# Multiplayer Command

## SGB Command $11 — MLT_REQ

Used to request multiplayer mode (that is, input from more than one joypad).
Because this function provides feedback from the SGB/SNES to the Game
Boy program, it can also be used to detect SGB hardware.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Multiplayer Control (0-3) (Bit0=Enable, Bit1=Two/Four Players)
         0 = One player
         1 = Two players
         3 = Four players
 2-F   Not used (zero)
```

In one-player mode, the second joypad (if any) can only be used for
the SGB BIOS. In two-player mode, both joypads are used for the game.
Because SNES only has two joypad sockets, four-player mode requires an
external "Multiplayer 5" adapter.

Changing the number of active players ANDs the currently selected player
minus one with the number of players in that mode minus one. For example,
if you go from four players to two players while player 4 was active,
player 2 will then be active because `3 & 1 = 1`. However, sending the
`MLT_REQ` command will increment the counter several times so results may
not be exactly as expected. The most frequent case is going from one
player to two-or-four player which will always start with player 1
active.

## Reading Multiple Controllers (Joypads)

When having enabled multiple controllers by `MLT_REQ`, data for each
joypad can be read out through [the `P1` register](<#FF00 — P1/JOYP: Joypad>) as follows: First
set P14 and P15 both HIGH (deselect both Buttons and Cursor keys), you
can now read the lower 4 bits of `P1`, which indicate the joypad ID for
the following joypad input:

Byte | Player \#
-----|-----------
 $xF | 1
 $xE | 2
 $xD | 3
 $xC | 4

Next, read joypad state normally.
The next joypad is automatically selected when P15 goes from LOW (0) to HIGH (1) ([source](https://github.com/CasualPokePlayer/test-roms/blob/sgb-mlt-test/src/intro.asm)), so you can simply repeat reading the joypad state normally until all two (or four) joypads have been read out.
