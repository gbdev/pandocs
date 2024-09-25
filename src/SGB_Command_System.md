# System Control Commands

## SGB Command $17 — MASK_EN

Used to mask the Game Boy window, among others this can be used to freeze
the Game Boy screen before transferring data through VRAM (the SNES then
keeps displaying the Game Boy screen, even though VRAM doesn't contain
meaningful display information during the transfer).

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1     Game Boy Screen Mask (0-3)
         0  Cancel Mask   (Display activated)
         1  Freeze Screen (Keep displaying current picture)
         2  Blank Screen  (Black)
         3  Blank Screen  (Color 0)
 2-F   Not used (zero)
```

Freezing works only if the SNES has stored a picture, that is, if necessary
wait one or two frames before freezing (rather than freezing directly
after having displayed the picture). The Cancel Mask function may be
also invoked (optionally) by completion of PAL_SET and ATTR_SET
commands.

## SGB Command $0C — ATRC_EN

Used to enable/disable Attraction mode, which is enabled by default.

Built-in borders other than the Game Boy frame and the plain black
border have a "screen saver" activated by pressing R, L, L, L, L, R or
by leaving the controller alone for roughly 7 minutes (tested with 144p
Test Suite). It is speculated that the animation may have interfered
with rarely-used SGB features, such as OBJ_TRN or JUMP, and that
Attraction Disable disables this animation.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Attraction Disable  (0=Enable, 1=Disable)
 2-F   Not used (zero)
```

## SGB Command $0D — TEST_EN

Used to enable/disable test mode for "SGB-CPU variable clock speed
function". This function is disabled by default.

This command does nothing on some SGB revisions. (SGBv2 confirmed,
unknown on others)

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Test Mode Enable    (0=Disable, 1=Enable)
 2-F   Not used (zero)
```

Maybe intended to determine whether SNES operates at 50Hz or 60Hz
display refresh rate ??? Possibly result can be read-out from joypad
register ???

## SGB Command $0E — ICON_EN

Used to enable/disable ICON function. Possibly meant to enable/disable
SGB/SNES popup menues which might otherwise activated during Game Boy
game play. By default all functions are enabled (0).

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     Disable Bits
         Bit 0 - Use of SGB-Built-in Color Palettes    (1=Disable)
         Bit 1 - Controller Set-up Screen    (0=Enable, 1=Disable)
         Bit 2 - SGB Register File Transfer (0=Receive, 1=Disable)
         Bit 3-6 - Not used (zero)
 2-F   Not used (zero)
```

Above Bit 2 will suppress all further packets/commands when set, this
might be useful when starting a monochrome game from inside of the
SGB-menu of a multi-gamepak which contains a collection of different
games.

## SGB Command $0F — DATA_SND

Used to write one or more bytes directly into SNES Work RAM.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     SNES Destination Address, low
 2     SNES Destination Address, high
 3     SNES Destination Address, bank number
 4     Number of bytes to write ($01-$0B)
 5     Data Byte #1
 6     Data Byte #2 (if any)
 7     Data Byte #3 (if any)
 etc.
```

Unused bytes at the end of the packet should be set to zero, this
function is restricted to a single packet, so that not more than 11
bytes can be defined at once. Free Addresses in SNES memory are Bank 0
1800-1FFF, Bank $7F 0000-FFFF.

## SGB Command $10 — DATA_TRN

Used to transfer binary code or data directly into SNES RAM.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     SNES Destination Address, low
 2     SNES Destination Address, high
 3     SNES Destination Address, bank number
 4-F   Not used (zero)
```

The data is sent by VRAM-Transfer (4 KBytes).

```
 000-FFF  Data
```

Free Addresses in SNES memory are Bank 0 1800-1FFF, Bank $7F
0000-FFFF. The transfer length is fixed at 4KBytes ???, so that
directly writing to the free 2KBytes at 0:1800 would be a not so good
idea ???

## SGB Command $12 — JUMP

Used to set the SNES program counter and NMI (vblank interrupt) handler
to specific addresses.

```
 Byte  Content
 0     Command*8+Length    (fixed length=1)
 1     SNES Program Counter, low
 2     SNES Program Counter, high
 3     SNES Program Counter, bank number
 4     SNES NMI Handler, low
 5     SNES NMI Handler, high
 6     SNES NMI Handler, bank number
 7-F   Not used, zero
```

The game *Space Invaders* uses this function when selecting "Arcade
mode" to execute SNES program code which has been previously
transferred from the SGB to the SNES. The SNES CPU is a Ricoh 5A22,
which combines a 65C816 core licensed from WDC with a custom memory
controller. For more information, see ["fullsnes" by
nocash](https://problemkaputt.de/fullsnes.htm).

Some notes for intrepid Super NES programmers seeking to use a flash
cartridge in a Super Game Boy as a storage server:

-   JUMP overwrites the NMI handler even if it is $000000.
-   The SGB system software does not appear to use NMIs.
-   JUMP can return to SGB system software via a 16-bit RTS. To do this,
    JML to a location in bank $00 containing byte value $60, such as
    any of the [stubbed commands](<#Stubbed commands>).
-   IRQs and COP and BRK instructions are not useful because their
    handlers still point into SGB ROM. Use SEI WAI.
-   If a program called through JUMP does not intend to return to SGB
    system software, it can overwrite all Super NES RAM except $0000BB
    through $0000BD, the NMI vector.
-   To enter APU boot ROM, write $FE to $2140. Echo will still be on
    though.
