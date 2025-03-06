# MBC2

(max 256 KiB ROM and 512×4 bits RAM)

## Memory

### 0000–3FFF — ROM Bank 0 \[read-only\]

Contains the first 16 KiB of the ROM.

### 4000–7FFF — ROM Bank $01-0F \[read-only\]

Same as for MBC1, but only a total of 16 ROM banks is supported.

### A000–A1FF — Built-in RAM

The MBC2 doesn't support external RAM, instead it includes 512 half-bytes of RAM (built into the MBC2 chip itself).
It still requires an external battery to save data during power-off though.
As the data consists of 4-bit values, only the lower 4 bits of the bytes in this memory area are used.
The upper 4 bits of each byte are undefined and should not be relied upon.

### A200–BFFF — 15 "echoes" of A000–A1FF

Only the bottom 9 bits of the address are used to index into the internal RAM, so RAM access repeats.
As with the A000–A1FF region, only the lower 4 bits of the "bytes" are used, and the upper 4 bits of each byte are undefined and should not be relied upon.

## Registers

### 0000–3FFF — RAM Enable, ROM Bank Number \[write-only\]

This address range is responsible for both enabling/disabling the RAM and for controlling the ROM bank number.
Bit 8 of the address (the least
significant bit of the upper address byte) determines whether to control
the RAM enable flag or the ROM bank number.

#### When bit 8 is clear

When the least significant bit of the upper address byte is zero, the value that is written controls whether the RAM is enabled.
Save RAM will be enabled if and only if the lower 4 bits of the value written here are `$A`.
If any other value is written, RAM is disabled.

Examples of addresses that can control RAM: $0000–00FF, $0200–02FF, $0400–04FF, ..., $3E00–3EFF.

RAM is disabled by default.

#### When bit 8 is set

When the least significant bit of the upper address byte is one, the value that is written controls the selected ROM bank at 4000–7FFF.

Specifically, the lower 4 bits of the value written to this address range specify the ROM bank number.
If bank 0 is written, the resulting bank will be bank 1 instead.

Examples of address that can control ROM: $0100–01FF, $0300–03FF, $0500–05FF, ..., $3F00–3FFF.

The ROM bank is set to 1 by default.
