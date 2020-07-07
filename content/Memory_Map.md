The Game Boy has a 16bit address bus, that is used to address ROM, RAM and I/O 

# General Memory Map


| **Start**   | **End**   | **Description**                                                                                  | **Notes**|
|-------------|-----------|--------------------------------------------------------------------------------------------------|-----------|
| 0000        | 3FFF      | 16KB ROM bank 00                                                                                 | From cartridge, usually a fixed bank|
| 4000        | 7FFF      | 16KB ROM Bank 01\~NN                                                                             | From cartridge, switchable bank via [MB](#memory-bank-controllers) (if any)|
| 8000        | 9FFF      | 8KB Video RAM (VRAM)                                                                             | Only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode |
| A000        | BFFF      | 8KB External RAM                                                                                 | In cartridge, switchable bank if any
| C000        | CFFF      | 4KB Work RAM (WRAM) bank 0                                                                       | |
| D000        | DFFF      | 4KB Work RAM (WRAM) bank 1\~N                                                                    | Only bank 1 in Non-CGB mode Switchable bank 1\~7 in CGB mode |
| E000        | FDFF      | Mirror of C000\~DDFF (ECHO RAM)                                                                  | Typically not used|
| FE00        | FE9F      | Sprite attribute table ([OAM](#vram-sprite-attribute-table-oam))   | |
| FEA0        | FEFF      | Not Usable                                                                                       | |
| FF00        | FF7F      | I/O Registers                                                                                    | |
| FF80        | FFFE      | High RAM (HRAM)                                                                                  | |
| FFFF        | FFFF      | [Interrupts](#interrupts) Enable Register (IE)                                         | |

# Jump Vectors in first ROM bank

The following addresses are supposed to be used as jump vectors:

-   RST commands: 0000, 0008,0010, 0018, 0020, 0028, 0030, 0038
-   Interrupts: 0040, 0048, 0050, 0058, 0060

However, the memory may be used for any other purpose in case that your
program doesn't use any (or only some) RST commands or interrupts. RST
commands are 1-byte opcodes that work similar to CALL opcodes, except
that the destination address is fixed. Since they are only 1 byte large,
they are also slightly faster.

# Cartridge Header in first ROM bank


The memory at 0100-014F contains the [cartridge
header](#the-cartridge-header). This area contains information
about the program, its entry point, checksums, information about the
used MBC chip, the ROM and RAM sizes, etc. Most of the bytes in this
area are required to be specified correctly.

# External Memory and Hardware

The areas from 0000-7FFF and A000-BFFF may be used to connect external
hardware. The first area is typically used to address ROM (read only, of
course), cartridges with [Memory Bank Controllers
(MBCs)](Memory_Bank_Controllers "wikilink") are additionally using this
area to output data (write only) to the MBC chip. The second area is
often used to address external RAM, or to address other external
hardware (Real Time Clock, etc). External memory is
often battery buffered, and may hold saved game positions and high score
tables (etc.) even when the Game Boy is turned off, or when the
cartridge is removed. For specific information read the chapter about
Memory Bank Controllers.

# Echo RAM

The memory range E000-FDFF is a mirror (or \"echo\") of WRAM, both for
reading and writing. For example, writing to \$E123 will modify both
\$C123 and \$E123. It is recommended to avoid using this memory range
anyways. This memory range's behavior has been confirmed on all grey
GBs as well as on CGB and GBA. Some emulators (such as VisualBoyAdvance
\<1.8) don't emulate Echo RAM. It is possible to check if Echo RAM is
properly emulated by writing to WRAM (avoid values 00 and FF) and
checking if said value is mirrored in Echo RAM.

# FEA0-FEFF range

This range is very poorly documented. It doesn't even have a name !
From my experience, this stays 00 on DMG, and alternates between 00 and
seemingly random values on CGB.
