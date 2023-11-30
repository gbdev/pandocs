# MMM01

The MMM01 is a mapper specific to multi-game compilation cartridges. It emulates an MBC1 for the contained games, and supports containing a mix of games from 32 KiB ROMs with no RAM, up to the same maximum memory _per game_ as the MBC1:

- max 512 KiB ROM with banked 8, 16, or 32 KiB RAM (default configuration)
- max 2 MiB ROM with unbanked 8 KiB RAM ("multiplex" mode, never used commercially)

Regardless of the size or number of the games in the compilation, the maximum total cartridge size supported by the MMM01 is the same: up to 8 MiB ROM and 128 KiB RAM.

The ROM and RAM banking numbers are extended compared to the MBC1 to allow for game selection, and
the lower bits (equivalent to the MBC1 bank registers) can be masked so some of those bits can also be used for game selection (for smaller games).
This allows up to 255x 32 KiB games, plus a 32 KiB menu, in an 8 MiB ROM.
RAM is more limited at only up to 16x 8 KiB RAM banks.
However, despite these generous capabilities, no MMM01 cartridge was released with more than 4 games, and only _one_ contains any RAM at all.

The ROM and RAM "game select" banking bits do not have to be set to the same value, this allows an MMM01 cartridge to not waste RAM space on games that do not have RAM, or mix and match games that have differently-sized ROM or RAM by packing them in tightly in the overall ROM/RAM of the cartridge.

:::warning

The MMM01 can't completely block access to RAM for a game, so if the cartridge contains RAM it's recommended to assign any no-RAM games to the same single RAM bank to prevent no-RAM games from accessing or corrupting other games' save RAM.

:::

MMM01 starts up in a unique mode ("unmapped") which always maps the **last** 32 KiB of the ROM to the 0000-7FFF address region regardless of the values in the bank/mode registers.
The correct ROM header (with Nintendo logo) therefore needs to be located at offset `(size - 32 KiB) + $100` in the ROM rather than the usual `$0000 + $100` (which contains the header of the first game in the collection instead).
MMM01 cartridges have a simple menu program in this last 32 KiB, which manipulates the additional MMM01 control bits, allowing game selection and setting the game size, before entering "mapped" mode and booting the selected game (see "Mapping Enable" below).

As the last 32 KiB of the ROM are reserved for the cartridge menu, it's best to pack games into the ROM from largest to smallest to avoid having a game overlap the menu.
For example, the Taito Variety Pack contains three 128 KiB games and one 64 KiB game in a 512 KiB ROM chip, leaving 32 KiB unused and 32 KiB for the menu.

## Memory

### 0000-3FFF - ROM Bank $X0 (Read Only)

On startup (in "unmapped" mode), this is mapped to the first half of the menu program in the last 32 KiB of the ROM.

When a game is mapped, this area normally contains the first 16 KiB (bank 00) of the game ROM.

If [multiplex is enabled](<#Multiplex Enable>), entering mode 1 allows mapping game ROM banks $20, $40, and $60 to this region.

#### Addressing diagrams

When "unmapped":

```
                                    /------------- In a smaller cartridge, only the needed
                                    |              bits are used (e.g 512 KiB uses 19 bits)
                  /---------------------------\
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \_____________________/  | \____________/
                 |             |      \----------- From Game Boy address
                 |             \------------------ Always 0
                 \-------------------------------- Always 1
```

Mapped, multiplex disabled:

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \___/ \___/ \____________/ \____________/
        |     |          |            \----------- From Game Boy address
        |     |          \------------------------ bitwise and of ROM Bank Low & ROM Bank Mask
        |     \----------------------------------- ROM Bank Mid
        \----------------------------------------- ROM Bank High
```

### 4000-7FFF - ROM Bank $01-7F (Read Only)

On startup (in "unmapped" mode), this is mapped to the second half of the menu program in the last 32 KiB of the ROM.

When a game is mapped, this area may contain any of the further 16 KiB banks of the game ROM, except for game banks $00, $20, $40, or $60.
If one of those banks is selected, the low bit is forced to 1 and that bank is mapped instead ($01, $21, $41, or $61).

i.e. in mapped mode, if `(ROM Bank Low) & ~(ROM Bank Mask)` is equal to $00 (indicating bank $00 within the game ROM), `(ROM Bank Low) | 1` is used instead.
As an example, if ROM Bank Low is set to $10, and the ROM Bank Mask is set to $30, then the bank within the game ROM would be `($10) & ~($30) = $00`.
As game bank $00 is disallowed, the low bit is forced on, mapping bank $11 instead of $10.

If [multiplex is enabled](<#Multiplex Enable>), the MMM01 has the same limitation as MBC1 regarding accessing game ROM banks $20, $40, and $60 - they can only be mapped to 0000-3FFF (in mode 1), and not to 4000-7FFF.

#### Addressing diagrams

When "unmapped":

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \_____________________/  | \____________/
                 |             |      \----------- From Game Boy address
                 |             \------------------ Always 1? See note
                 \-------------------------------- Always 1
```

:::warning TO BE VERIFIED

It's suspected the lowest bit of ROM Bank Low (post $00 -> $01 remapping) still affects the bank mapped to the 4000-7FFF region in "unmapped" mode the same as it does in mapped mode.
Most of the time this would still be a 1, but during game selection it could momentarily go to 0 in between setting the game select bits in [ROM Bank Low](<#Bits 0-4: ROM Bank Low>) and masking them as such in [ROM Bank Mask](<#Bits 1-5: ROM Bank Mask>).

:::

Mapped, multiplex disabled:

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \___/ \___/ \____________/ \____________/
        |     |          |            \----------- From Game Boy address
        |     |          \------------------------ ROM Bank Low (complete value, not masked)
        |     |                                     anti-masked $00 is remapped to $01 (see explanation above)
        |     \----------------------------------- ROM Bank Mid
        \----------------------------------------- ROM Bank High
```

### A000-BFFF - RAM Bank $00-03, if any (Read/Write)

This area is used to address external SRAM in the cartridge (if any).
The RAM is only accessible [if RAM is enabled](<#0000-1FFF - RAM Enable (Write Only)>), otherwise reads return open bus values (often $FF, but not guaranteed) and writes are ignored.

External RAM is often battery-backed, allowing for the storage of game data while the Game Boy is turned off, or if the cartridge is removed from the Game Boy.

It is currently unknown whether RAM access is possible while in unmapped mode.

#### Addressing diagrams

In mode 0:

```
Bits: 16 15 14 13 12 .. 01 00
      \___/ \___/ \_________/
        |     |        \-------- From Game Boy address
        |     \----------------- bitwise and of RAM Bank Low & RAM Bank Mask
        \----------------------- RAM Bank High
```

In mode 1:

```
Bits: 16 15 14 13 12 .. 01 00
      \___/ \___/ \_________/
        |     |        \-------- From Game Boy address
        |     \----------------- RAM Bank Low (complete value, not masked)
        \----------------------- RAM Bank High
```

## Registers

The MMM01 registers are extensions of the MBC1 registers.
In "mapped" mode, the extra bits cannot be written to, and the MMM01 emulates an MBC1.
In "unmapped" mode, the extra bits are writeable, allowing the menu program to select which game to play and configure its size and RAM access.

All the MMM01 registers are 7-bits in size and set to $00 on power-up.
For the ROM Bank Number register, this behaves as if it was set to $01.

### 0000-1FFF - RAM Enable (Write Only)

```
Bits: X 6 5 4 3 2 1 0
        | \_/ \_____/
        |  |      \----- Bits 0-3: RAM Enable
        |  \------------ Bits 4-5: RAM Bank Mask
        \--------------- Bit 6: Mapping Enable
```

#### Bits 0-3: RAM Enable

As per MBC1, writing $A to the lower 4 bits enables the external RAM, and any other value disables it.
The external RAM is automatically disabled when the Game Boy is powered off or the cartridge is removed.

#### Bits 4-5: RAM Bank Mask

_Can only be changed in "unmapped" mode._

These two bits act as "write locks" to the matching bits in the RAM Bank Low register.
Any attempt to write to those bits while its matching bit in this register is 1 is prevented.
Writes to masked bits are prevented even in "unmapped" mode.

The purpose of the mask is to lock some RAM banking bits as "game selection", instead of letting games freely toggle them.
Setting these bits effectively reduces the size of the ram available to the game that's about to be booted:

| Mask | Game RAM |
| ---- | -------- |
| 00   | 32 KiB   |
| 10   | 16 KiB   |
| 11   | 8 KiB    |

If [multiplex is enabled](<#Multiplex Enable>), this mask still applies to the RAM Bank Low register, even though that register is used as part of the **ROM** bank number in multiplex mode.
This has the effect of reducing the ROM size instead of the RAM size, as follows:

| Mask | Game ROM  |
| ---- | --------- |
| 00   | 2 MiB     |
| 10   | 1 MiB     |
| 11   | 512 KiB\* |

Setting the RAM Bank Mask to 11 when multiplex is enabled is pointless - the whole point of multiplex mode is to support 1 MiB or larger MBC1 games.

#### Bit 6: Mapping Enable

_Can only be changed in "unmapped" mode._

When writing 1 to this bit, the MMM01 enters "mapped" mode. This immediately begins MBC1 emulation, mapping in the selected banks of the game ROM (typically $00 and $01 within the game's subsection of the cartridge ROM) and prevents write access to any of the MMM01 extended control bits.

It is unknown if setting bit 6 to enter "mapped" mode will honor or ignore the value simultaneously being written to bits 4-5 (RAM Bank Mask).
The only released MMM01 cartridge containing RAM performs two separate writes to set the bank mask before enabling mapping, with the same mask set in both writes.

### 2000-3FFF - ROM Bank Number (Write Only)

```
Bits: X 6 5 4 3 2 1 0
        \_/ \_______/
         |       \------ Bits 0-4: ROM Bank Low
         \-------------- Bits 5-6: ROM Bank Mid
```

#### Bits 0-4: ROM Bank Low

This is equivalent to the MBC1 ROM Bank register, and primarily selects which bank of the game ROM is visible in the 4000-7FFF region.
It can be masked to reduce its size, reserving some bits for game select (see [ROM Bank Mask](<#Bits 1-5: ROM Bank Mask>)).

If the _unmasked_ bits are all 0, it behaves as if the lowest bit is set to 1 (as per MBC1 behaviour, attempting to map bank $00 of the game ROM maps bank $01 instead).
However, the actual value of the register doesn't change, as changes to [ROM Bank Mask](<#Bits 1-5: ROM Bank Mask>) can undo this remapping.

#### Bits 5-6: ROM Bank Mid

_Can only be changed in "unmapped" mode._

This register represents an additional two bits of ROM bank number, for game selection.
Affects both the 0000-3FFF and 4000-7FFF region.
Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

If [multiplex is enabled](<#Multiplex Enable>), functionality is swapped with [RAM Bank Low](<#Bits 0-1: RAM Bank Low>) allowing for larger game ROM.

### 4000-5FFF - RAM Bank Number (Write Only)

```
Bits: X 6 5 4 3 2 1 0
        | \_/ \_/ \_/
        |  |   |   \---- Bits 0-1: RAM Bank Low
        |  |   \-------- Bits 2-3: RAM Bank High
        |  \------------ Bits 4-5: ROM Bank High
        \--------------- Bit 6: MBC1 Mode Write Disable
```

#### Bits 0-1: RAM Bank Low

This is equivalent to the MBC1 RAM Bank register.
It can be masked to reduce its size, reserving some bits for game select (see [RAM Bank Mask](<#Bits 4-5: RAM Bank Mask>)).

If [multiplex is enabled](<#Multiplex Enable>), functionality is swapped with [ROM Bank Mid](<#Bits 5-6: ROM Bank Mid>) allowing for larger game ROM.

#### Bits 1-2: RAM Bank High

_Can only be changed in "unmapped" mode._

This register represents an additional two bits of RAM bank number, for game selection.
Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

#### Bits 4-5: ROM Bank High

_Can only be changed in "unmapped" mode._

This register represents _another_ two bits of ROM bank number, for game selection.
Affects both the 0000-3FFF and 4000-7FFF region.
Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

#### Bit 6: MBC1 Mode Write Lock

_Can only be changed in "unmapped" mode._

When set to 1, prevents changes to the [MBC1 Mode Select](<#Bit 0: MBC1 Mode Select>).
This might be for compatibility with games designed for non-MBC1 mappers that don't expect the MBC1 mode select register to exist.

### 6000-7FFF - Banking Mode Select (Write Only)

```
Bits: X 6 5 4 3 2 X 0
        | \_______/ |
        |    |      \--- MBC1 Mode Select
        |    \---------- ROM Bank Mask
        \--------------- Multiplex Enable
```

#### Bit 0: MBC1 Mode Select

This is equivalent to the MBC1 Mode Select register.

This 1-bit register selects between the two MBC1 banking modes.
The behaviour varies depending on whether multiplex is enabled or disabled.

**Multiplex disabled**

- 0 = RAM Banking Disabled (default)
- 1 = RAM Banking Enabled

In mode 0, the A000-BFFF region is locked to bank 0 of the game RAM.
The unmasked bits of [RAM Bank Low](<#Bits 0-1: RAM Bank Low>) are treated as 0.

In mode 1, the A000-BFFF region can be bank-switched by the game as the full RAM Bank Low register is used.

**Multiplex enabled**

- 0 = Simple ROM Banking Mode (default)
- 1 = Advanced ROM Banking Mode

In mode 0, the 0000-3FFF region is locked to bank 0 of the game ROM.
The unmasked bits of [RAM Bank Low](<#Bits 0-1: RAM Bank Low>) are treated as 0 for accesses to the 0000-3FFF region, matching the behaviour of [ROM Bank Low](<#Bits 0-4: ROM Bank Low>).

In mode 1, the 0000-3FFF region can be bank-switched using the RAM Bank Low register — allowing access to game ROM banks $20, $40, and $60.

#### Bits 1-5: ROM Bank Mask

_Can only be changed in "unmapped" mode._

These five bits act as "write locks" to the matching bits in the [ROM Bank Low](<#Bits 0-4: ROM Bank Low>) register.
Any attempt to write to the bits in ROM Bank Low while its matching bit in this register is 1 is prevented.
Writes to masked bits are prevented even in "unmapped" mode.

The value written to the lowest bit of the mask is ignored, and treated as always zero.
As a result, the lowest bit of ROM Bank Low is always writeable.

The purpose of the mask is to lock some ROM banking bits as "game selection", instead of letting games freely toggle them.
Setting these bits effectively reduces the size of the ROM accessible to the game that's about to be booted:

| Mask  | Game ROM |
| ----- | -------- |
| 00000 | 512 KiB  |
| 10000 | 256 KiB  |
| 11000 | 128 KiB  |
| 11100 | 64 KiB   |
| 11110 | 32 KiB   |

Note: changing the mask can alter which bank would be mapped.
Only the _unmasked_ bits of [ROM Bank Low](<#Bits 0-4: ROM Bank Low>) are used for the "[attempting to map bank 0 maps bank 1](<#4000-7FFF - ROM Bank $01-7F (Read Only)>)" logic, and it updates live if the ROM Bank Mask changes.
ROM Bank Low itself doesn't change when this happens — only the value used for calculating the bank number.

If [multiplex is enabled](<#Multiplex Enable>), the [RAM Bank Mask](<#Bits 4-5: RAM Bank Mask>) affects ROM banking as well.
In this case the ROM Bank Mask should be set to 00000 to avoid masking bits in the _middle_ of the full game ROM bank number.

#### Multiplex Enable

_Can only be changed in "unmapped" mode._

When set to 1, swaps the functionality of [RAM Bank Low](<#Bits 0-1: RAM Bank Low>) and [ROM Bank Mid](<#Bits 5-6: ROM Bank Mid>).
As RAM Bank Low is writeable in mapped mode, this allows for the contained game to control (up to, if unmasked in "RAM Bank Mask") two extra bits of the full ROM bank number, allowing for larger game ROMs at the cost of only 8 KiB of external RAM.

This is equivalent to the ["large ROM" wiring of an MBC1 cartridge](#MBC1).

## Multiplex addressing diagrams

If [multiplex is enabled](<#Multiplex Enable>), the addressing diagrams change as follows (changes marked with `*`):

### 0000-3FFF - ROM Bank X0

No change to "unmapped" mode.

Mapped, Multiplexed, Mode 0:

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \___/ \___/ \____________/ \____________/
        |     |          |            \----------- From Game Boy address
        |     |          \------------------------ bitwise and of ROM Bank Low & ROM Bank Mask
        |     \----------------------------------- * bitwise and of RAM Bank Low & RAM Bank Mask
        \----------------------------------------- ROM Bank High
```

Mapped, Multiplexed, Mode 1:

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \___/ \___/ \____________/ \____________/
        |     |          |            \----------- From Game Boy address
        |     |          \------------------------ bitwise and of ROM Bank Low & ROM Bank Mask
        |     \----------------------------------- * RAM Bank Low (complete value, not masked)
        \----------------------------------------- ROM Bank High
```

##### 4000-7FFF - ROM Bank 01-7F

No change to "unmapped" mode.

Mapped, Multiplexed, Mode 0 or 1:

```
Bits: 22 21 20 19 18 17 16 15 14 13 12 .. 01 00
      \___/ \___/ \____________/ \____________/
        |     |          |            \----------- From Game Boy address
        |     |          \------------------------ ROM Bank Low (complete value, not masked)
        |     \----------------------------------- * RAM Bank Low (complete value, not masked)
        \----------------------------------------- ROM Bank High
```

##### A000-BFFF - RAM Bank 00-03

Multiplexed, Mode 0 or 1:

```
Bits: 16 15 14 13 12 .. 01 00
      \___/ \___/ \_________/
        |     |        \-------- From Game Boy address
        |     \----------------- * ROM Bank Mid
        \----------------------- RAM Bank High
```

## Operation Notes

Because the MMM01 has registers which disable write access to other registers, it is optimal to set the value of certain registers before others. The optimal order is:

1. ROM Bank Register ($2000) — contains ROM Bank
2. Mode Register ($6000) — contains ROM Bank Mask, and Mode
3. RAM Register ($4000) — contains Mode Write Disable, and RAM Bank
4. RAM Enable ($0000) — contains RAM Bank Mask, and Mapping Enable

The majority of released MMM01 cartridges stick to this order.

### References

- Source: [MMM01](https://wiki.tauwasser.eu/view/MMM01) on tauwasser wiki
