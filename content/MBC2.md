(max 256 KiB ROM and 512x4 bits RAM)

## Memory

### 0000-3FFF - ROM Bank 00 (Read Only)

Contains the first 16KByte banks of the ROM.

### 4000-7FFF - ROM Bank 01-0F (Read Only)

Same as for MBC1, but only a total of 16 ROM banks is supported.

### A000-A1FF - 512x4bits RAM, built-in into the MBC2 chip (Read/Write)

The MBC2 doesn't support external RAM, instead it includes 512x4 bits
of built-in RAM (in the MBC2 chip itself). It still requires an external
battery to save data during power-off though. As the data consists of
4 bit values, only the lower 4 bits of the bytes in this memory area
are used.

## Registers

### 0000-3FFF - RAM Enable and ROM Bank Number (Write Only)

This address range is responsible for both enabling/disabling the RAM
and for controlling the ROM bank number. Bit 8 of the address determines
whether to control the RAM-enable flag or the ROM bank number.

#### When Bit 8 is Clear

When the least significant bit of the upper address byte is zero, the
value that is written controls whether the RAM is enabled. When the
value written to this address range is equal to `0Ah`, RAM is enabled.
If any other value is written, RAM is disabled.

Examples of address that can control RAM: 0000-00FF, 0200-02FF,
0400-04FF, ..., 3E00-3EFF.

RAM is disabled by default.

#### When Bit 8 is Set

When the least significant bit of the upper address byte is one, the
value that is written controls the selected ROM bank at 4000-7FFF.

Specifically, the lower 4 bits of the value written to this address
range specify the ROM bank number. If bank 0 is written, the resulting
bank will be bank 1 instead.

Examples of address that can control ROM: 0100-01FF, 0300-03FF,
0500-05FF, ..., 3F00-3FFF.

The ROM bank is set to 1 by default.
