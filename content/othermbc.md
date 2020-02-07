Multicart MBCs
--------------

**MBC1M** uses the MBC1 IC, but the board does not connect the MBC1's
A18 address output to the ROM. This allows including multiple 2 Mbit (16
bank) games, with SRAM bank select ($4000) to select which of up to
four games is switched in. In theory, a MBC1M board could be made for 1
Mbit or 512 kbit games by additionally not connecting A17 and A16
outputs, but this appears not to have been done in licensed games.

**MMM01** is a more complex that allows for games of different sizes
[Docs on Tauwasser.eu](https://wiki.tauwasser.eu/view/MMM01)

**Bung** and **EMS** MBCs are reported to exist.

### EMS

PinoBatch learned the game selection protocol for EMS flash carts from
beware, who in turn learned it from nitro2k01. Take this with a grain of
salt, as it hasn't been verified on the authentic EMS hardware.

A [header](The_Cartridge_Header "wikilink") matching any of the
following is detected as EMS mapper:

-   Header name is "EMSMENU", NUL-padded
-   Header name is "GB16M", NUL-padded
-   Cartridge type ($0147) = $1B and region ($014A) = $E1

Registers:

$2000 write: Normal behavior, plus save written value in $2000 latch
$1000 write: $A5 enables configure mode, $98 disables, and other values have no known effect
$7000 write while configure mode is on: Copy $2000 latch to OR mask

After the OR mask has been set, all reads from ROM will OR A21-A14 (the
bank number) with the OR mask. This chooses which game is visible to the
CPU. If the OR mask is not aligned to the game size, the results may be
nonsensical.

The mapper does not support an outer bank for battery SRAM.

To start a game, do the following in code run from RAM: Write $A5 to
$1000, write game starting bank number to $2000, write any value to
$7000, write $98 to $1000, write $01 to $2000 (so that 32K games
work), jump to $0100.

### Wisdom Tree

The Wisdom Tree mapper is a simple, cost-optimized one chip design
consisting of a 74LS377 octal latch, aside from the ROM chip. Because
the mapper consists of a single standard 74 series logic chip, it has
two unusual properties:

First, unlike a usual MBC, it switches the whole 32 kiB ROM area instead
of just the $4000-$7FFF area. If you want to use the interrupt vectors
with this cart, you should duplicate them across all banks.
Additionally, since the initial state of the '377 can't be guaranteed,
the ROM header and some code for switching to a known bank should also
be included in every bank. This also means that the Wisdom Tree mapper
could be used as a multicart mapper for 32 kiB ROMs, assuming there was
enough ROM space in each bank for some small initialization code, and
none of the ROMs wrote to the $0000-$7FFF area. For example, if the
last 5 bytes of all banks are unused, games can be patched as follows:

    ; At $0100 in all banks but the first
      nop
      jp $7FFB

    ; At $7FFB in all banks
      ld hl, $0100
      ld [hl], a
      jp hl

Second, because the '377 latches data on the *positive* edge, and the
value on the Game Boy data bus is no longer valid when the positive edge
of the write pulse arrives, the designer of this mapper chose to use the
A7-A0 address lines for selecting a bank instead of the data lines.
Thus, the value you write is ignored, and the lower 8 bits of the
address is used. For example, to select bank $XX, you would write any
value to address $YYXX, where $YY is in the range $00-$7F.

An emulator can detect a ROM designed for Wisdom Tree mapper in one of
two ways:

-   ROM contains "WISDOM TREE" or "WISDOM\\x00TREE" (the space can
    be $20 or $00), $0147 = $00, $0148 = $00, size \> 32k. This
    method works for the games released by Wisdom Tree, Inc.
-   $0147 = $C0, $014A = $D1. These are the values recommended by
    beware for 3rd party developers to indicate that the ROM is
    targeting Wisdom Tree mapper hardware. (See below.)

### Magic values for detection of multicarts in emulators

Sometimes it may be useful to allow a ROM to be detected as a multicart
in emulator, for example for development of a menu for physical
multicart hardware. These are values suggested by beware, and supported
in BGB, for signaling that your ROM is supposed to interface a multicart
mapper. Emulator authors who are interested in supporting multicart
mappers are encouraged to support detection of these values in addition
to the values described in each section, which are heuristics based on
ROMs in the wild, which may not always be suitable for newly produced
software. The values are deliberately chosen to be high entropy
("random") such that the risk of an accidental false positive is
unlikely.

-   $0147 = $c0, $014a = $d1 -\> Detect as Wisdom Tree
-   $0147 = $1b, $014a = $e1 -\> Detect as EMS multicart
-   $0147 = $be -\> Detect as Bung multicart

MBC Timing Issues
-----------------

Among Nintendo MBCs, only the MBC5 is guaranteed by Nintendo to support
the tighter timing of CGB Double Speed Mode. There have been rumours
that older MBCs (like MBC1-3) wouldn't be fast enough in that mode. If
so, it might be nevertheless possible to use Double Speed during periods
which use only code and data which is located in internal RAM. However,
despite of the above, a self-made MBC1-EPROM card appears to work stable
and fine even in Double Speed Mode though.

