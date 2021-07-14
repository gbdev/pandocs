# MMM01

The MMM01 is a mapper specific to multi-game compilation cartridges. It emulates an MBC1 for the contained games, and supports containing a mix of games from 32 KiB ROM with no RAM up to the same maximum memory _per game_ as the MBC1:

* max 512 KiB ROM with banked 8, 16, or 32 KiB RAM (default configuration)
* max 2 MiB ROM with unbanked 8 KiB RAM ("multiplex" mode[^multiplex])

[^multiplex]:
However, no cartridges were released using the MMM01 "multiplex" mode.

Regardless of the size or number of the games in the compilation, the maximum total cartridge size supported by the MMM01 is the same - up to 8 MiB ROM and 128 KiB RAM.

Additional bits in the control registers extend the MBC1 ROM and RAM banking numbers to allow for game selection, and the MBC1 ROM/RAM banking registers can be masked so some of _those_ bits can also be used for game selection, allowing for up to 255x 32 KiB games, plus a 32KiB menu, in an 8 MiB ROM. RAM is more limited at only up to 16x 8kB RAM banks in total. However, despite these generous capabilities, no MMM01 cartridge was released with more than 4 games, and only _one_ contains any RAM at all.

The ROM and RAM "game select" bank numbers do not have to be set to the same number - this allows an MMM01 cartridge to not waste RAM space on games that do not have RAM[^no_ram], or mix and match games that have different sized ROM or RAM by packing them in tightly in the overall ROM/RAM of the cartridge.

[^no_ram]:
Technically, the MMM01 can't completely block access to RAM for a game, so if the cartridge contains RAM it's recommended to assign any no-RAM games to the same single RAM bank to prevent no-RAM games from accessing or corrupting other games' SRAM saves.

MMM01 starts up in a unique mode ("unmapped") which always maps the _last 32 KiB_ of the ROM to the 0000-7FFF address region regardless of the values in the bank/mode registers. The correct ROM header (with Nintendo logo) therefore needs to be located at offset `(size - 32 KiB) + $100` in the ROM rather than the usual `0000 + $100` (which contains the header of the first game in the collection instead). MMM01 cartridges have a simple menu program in this last 32 KiB, which manipulates the additional MMM01 control bits, allowing game selection and setting the game size, before entering "mapped" mode and booting the selected game (see "Mapping Enable" below).

As the last 32 KiB of the ROM are reserved for the cartridge menu, it's best to pack games into the ROM from largest to smallest to avoid having a game overlap the menu. e.g. the Taito Variety Pack contains three 128 KiB games and one 64 KiB game, leaving 32 KiB unused and 32 KiB for the menu in a 512 KiB ROM chip.

## Memory

### 0000-3FFF - ROM Bank X0 (Read Only)

On start (in "unmapped" mode), this is mapped to the first half of the menu program in the last 32 KiB of the ROM.

When a game is mapped, this area normally contains the first 16 KiB (bank 00) of the game ROM. In multiplexed mode with MBC1 mode set to 1, banks $20, $40, and $60 of the game ROM can be mapped here (the same as MBC1).

### 4000-7FFF - ROM Bank 01-7F (Read Only)

On start (in "unmapped" mode), this is mapped to the second half of the menu program in the last 32 KiB of the ROM.

When a game is mapped, this area may contain any of the further 16 KiB banks of the ROM. Cannot address any banks where the masked main ROM banking register would be 00, instead it automatically maps to the bank 1 higher.

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

This area is used to address external SRAM in the cartridge (if any). The RAM is only accessible if RAM is enabled, otherwise reads return open bus values (often $FF, but not guaranteed) and writes are ignored.

External RAM is often battery-backed, allowing for the storage of game data while the Game Boy is turned off, or if the cartridge is removed from the Game Boy.

It is currently unknown whether RAM access is possible while in unmapped mode.

## Registers

The MMM01 registers are extensions of the MBC1 registers. In "mapped" mode, the extra bits are unwriteable and the MMM01 emulates an MBC1. In "unmapped" mode, the extra bits are writeable, allowing the menu program to select which game to play and configure its size and RAM access.

All the MMM01 registers are 7-bits in size and set to $00 on power-up. For the ROM Bank Number register, this behaves as if it was set to $01.

### 0000-1FFF - RAM Enable (Write Only)

```
Bits: X 6 5 4 3 2 1 0
        | \_/ \_____/
        |  |      \----- Bits 0-3: RAM Enable
        |  \------------ Bits 4-5: RAM Bank Mask
        \--------------- Bit 6: Mapping Enable
```

#### Bits 0-3: RAM Enable

As per MBC1, writing 0Ah here enables the RAM, and any other value disables the RAM. RAM is automatically disabled when the gameboy is powered off.

#### Bits 4-5: RAM Bank Mask

Can only be written in unmapped mode.

If either or both of these bits are set high, the corresponding bit(s) in the MBC1 RAM banking register are locked in as "game select" bits instead of "banking" bits. Any attempt to write to those bits while its matching bit in this register is 1 is prevented.

#### Bit 6: Mapping Enable

Can only be written in unmapped mode.

If 1, the MMM01 enters "mapped" mode. This immediately begins MBC1 emulation, mapping in the selected banks of the game ROM (typically $00 and $01 within the game's subsection of the cartridge ROM) and prevents write access to any of the MMM01 extended control bits.

It is unknown if setting bit 6 to enter mapping mode will prevent simultaneous writes to bits 4-5 (RAM Bank Mask). The only released MMM01 cartridge containing RAM performs two separate writes to set the bank mask before enabling mapping.

### 2000-3FFF - ROM Bank Number (Write Only)

```
Bits: X 6 5 4 3 2 1 0
        \_/ \_______/
         |       \------ Bits 0-4: ROM Bank Low
         \-------------- Bits 5-6: ROM Bank Mid
```

#### Bits 0-4: ROM Bank Low

This is equivalent to the MBC1 ROM Bank register, and (when in mapped mode, in addition to the ROM Bank Mid/High game select registers) selects the ROM bank visible in the 4000-7FFF region. It can be masked to reduce its size, reserving some of the bits for game select (see "ROM Bank Mask"). Bits masked for game select also affect the bank visible in the 0000-3FFF region.

If the _unmasked_ bits are all 0, it behaves as if the lowest bit is set to 1 (as per MBC1 behaviour, mapping bank 00 maps bank 01). However, the actual value of the register doesn't change.

Note: It's believed the lowest bit of the bank number (post 00->01 remapping) still affects the bank mapped to the 4000-7FFF region in "unmapped" mode - causing it to swap between the same second-last bank as 0000-3FFF (if the unmasked bits of ROM Bank Low are set to a non-zero value with the lowest bit 0) or the last bank of the ROM (if the unmasked bits of ROM Bank Low are all zero (which is treated as 01) or the lowest bit is in fact 1).

#### Bits 5-6: ROM Bank Mid

_Can only be written in unmapped mode._

This register represents an additional two bits of ROM bank number, for game selection. Affects both the 0000-3FFF and 4000-7FFF region. Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

In multiplexed mode, functionality is swapped with RAM Bank Low (See "Multiplex Enable") allowing for larger game ROM.

### 4000-5FFF - RAM Bank Number (Write Only)

Bits: X 6 5 4 3 2 1 0
        | \_/ \_/ \_/
        |  |   |   \---- Bits 0-1: RAM Bank Low
        |  |   \-------- Bits 2-3: RAM Bank High
        |  \------------ Bits 4-5: ROM Bank High
        \--------------- Bit 6: MBC1 Mode Write Disable

#### Bits 0-1: RAM Bank Low

This is equivalent to the MBC1 RAM Bank register. It can be masked to reduce its size, reserving some of the bits for game select (see "RAM Bank Mask").

In multiplexed mode, functionality is swapped with ROM Bank Mid (See "Multiplex Enable") allowing for larger game ROM.

#### Bits 1-2: RAM Bank High

_Can only be written in unmapped mode._

This register represents an additional two bits of RAM bank number, for game selection. Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

#### Bits 4-5: ROM Bank High

_Can only be written in unmapped mode._

This register represents _another_ two bits of ROM bank number, for game selection. Affects both the 0000-3FFF and 4000-7FFF region. Can only be used for game selection, as it's not writeable once entering a game (mapped mode).

#### Bit 6: MBC1 Mode Write Disable

_Can only be written in unmapped mode._

When set to 1, prevents changes to the "MBC1 Mode Select". This might be for compatibility with games designed for non-MBC1 mappers that don't expect the MBC1 mode select register to exist.

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

```
00 = Simple ROM Banking Mode (default)
01 = RAM Banking Mode / Advanced ROM Banking Mode
```

In mode 0, the unmasked (see "RAM Bank Mask") bits of the RAM Bank Low register are treated as zero for accesses to the A000-BFFF region, or the 0000-3FFF region in multiplex mode. In multiplex mode accesses to the 4000-7FFF region use the full bank number as normal.

In mode 1, the RAM Bank Low register behaves normally. In multiplex mode, this has the side effect of it affecting accesses to the 0000-3FFF region, as per MBC1.

#### Bits 1-5: ROM Bank Mask

_Can only be written in unmapped mode._

If any or all bits are set high, the corresponding bit(s) in the MBC1 ROM banking register are locked in as "game select" bits instead of "banking" bits. Any attempt to write to those bits while its matching bit in this register is 1 is prevented. The lowest bit of the ROM Bank Mask is ignored and treated as always 0, so the lowest bit of ROM Bank Low is always writeable.

Note: changing the mask can alter what bank would be mapped - only the _unmasked_ bits are used for the "bank 0 is treated as bank 1" logic. So if ROM Bank Low is set to $10, 

#### Multiplex Enable

_Can only be written in unmapped mode._

When set to 1, swaps the functionality of RAM Bank Low with ROM Bank Mid. As RAM Bank Low is writeable in mapped mode, this allows for the contained game to control (up to - if unmasked in "RAM Bank Mask") two extra bits of the full ROM bank number, allowing for larger game ROMs.

This is equivalent to the "large ROM" wiring of an MBC1 cart.

## Operation Notes

Because the MMM01 has registers which disable write access to other registers, it is optimal to set the value of certain registers before others. The optimal order is:

```
1. 2000 ROM Bank Register - contains ROM Bank
2. 6000 Mode Register     - contains ROM Bank Mask, and Mode
3. 4000 RAM Register      - contains Mode Write Disable, and RAM Bank
4. 0000 RAM Enable        - contains RAM Bank Mask, and Mapping Enable
```

The majority of released MMM01 cartridges stick to this order.
