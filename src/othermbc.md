# Other MBCs

## Multicart MBCs

**MBC1M** uses the MBC1 IC, but the board does not connect the MBC1's
A18 address output to the ROM. This allows including multiple 2 Mbit (16
bank) games, with SRAM bank select ($4000) to select which of up to
four games is switched in. In theory, a MBC1M board could be made for 1
Mbit or 512 kbit games by additionally not connecting A17 and A16
outputs, but this appears not to have been done in licensed games.

**Bung** and **EMS** MBCs are reported to exist.

## EMS

:::warning To be verified

Take the following with a grain of salt, as it hasn't been verified on authentic EMS hardware. See related github issue to contribute: [#423]( https://github.com/gbdev/pandocs/issues/423).

:::

A [header](<#The Cartridge Header>) matching any of the following is detected as EMS mapper:

- Header name is "EMSMENU", NUL-padded
- Header name is "GB16M", NUL-padded
- Cartridge type ($0147) = $1B and region ($014A) = $E1

Registers:

- $2000 write: Normal behavior, plus save written value in $2000 latch
- $1000 write: $A5 enables configure mode, $98 disables it, and other values have no known effect
- $7000 write while configure mode is on: Copy $2000 latch to OR mask

After the OR mask has been set, all reads from ROM will OR A21-A14 (the
bank number) with the OR mask. This chooses which game is visible to the
CPU. If the OR mask is not aligned to the game size, the results may be
nonsensical.

The mapper does not support an outer bank for battery SRAM.

To start a game, perform the following steps with code running from RAM:

1. Write $A5 to $1000
2. Write game's first bank number to $2000
3. Write any value to $7000
4. Write $98 to $1000
5. Write $01 to $2000 (so that 32K games work)
6. Jump to $0100

## Wisdom Tree

The Wisdom Tree mapper is a simple, cost-optimized one-chip design
consisting of a 74LS377 octal latch in addition to the ROM chip. Because
the mapper consists of a single standard 74 series logic chip, it has
two unusual properties:

First, unlike a usual MBC, it switches the whole 32 KiB ROM area instead
of just the $4000-$7FFF area. Therefore, if you want to use [the interrupt vectors](<#Interrupt Handling>)
with this cart, you should duplicate them across all banks.
Additionally, since the 74LS377's contents can't be guaranteed when powering on,
the ROM header and some code for switching to a known bank should also
be included in every bank. This also means that the Wisdom Tree mapper
could be used as a multicart mapper for 32 KiB ROMs, assuming there is
enough ROM space in each bank for some small initialization code, and
none of the ROMs wrote to the $0000-$7FFF area. For example, if the
last 5 bytes of all banks are unused, games can be patched as follows:

```rgbasm
; At $0100 in all banks but the first
    nop
    jp $7FFB
```
```rgbasm
; At $7FFB in all banks
    ld hl, $0100
    ld [hl], a
    jp hl
```

Second, because the 74LS377 latches data on the [*positive* write pulse edge](https://www.allaboutcircuits.com/textbook/digital/chpt-10/edge-triggered-latches-flip-flops/),
and the value on the Game Boy data bus is no longer valid when the
positive edge arrives, the designer of this mapper chose to use the
A7-A0 address lines for selecting a bank instead of the data lines.
Thus, the value you write is ignored, and the lower 8 bits of the
address is used. For example, to select bank $XX, you would write any
value to address $YYXX, where $YY is in the range $00-$7F.


## Magic values for detection of multicarts in emulators

:::tip proposal

The following information should not be considered a universally adopted
standard, but it's instead just a proposed solution. Actual adoption may vary.

:::

Sometimes it may be useful to allow a ROM to be detected as a multicart
in emulator, for example for development of a menu for physical
multicart hardware. 

Emulator authors who are interested in supporting the other multicart
mappers are also encouraged to support detection of the following values.

- Detect as Wisdom Tree mapper
  - [ROM title](<#0134-0143 — Title>) is "WISDOM TREE" (the space may be a
  $00 NUL character instead), $0147 = $00, $0148 = $00, size \> 32k.
  This method works for the games released by Wisdom Tree, Inc.
  - $0147 = $C0, $014A = $D1.
- Detect as EMS multicart
  - $0147 = $1b, $014a = $e1
- Detect as Bung multicart
  - $0147 = $be
