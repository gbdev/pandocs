The Game Boy has a 16-bit address bus, which is used to address ROM, RAM, and I/O.

# General Memory Map


| **Start**   | **End**   | **Description**                                                                                  | **Notes**|
|-------------|-----------|--------------------------------------------------------------------------------------------------|-----------|
| 0000        | 3FFF      | 16KB ROM bank 00                                                                                 | From cartridge, usually a fixed bank|
| 4000        | 7FFF      | 16KB ROM Bank 01\~NN                                                                             | From cartridge, switchable bank via [MB](#memory-bank-controllers) (if any)|
| 8000        | 9FFF      | 8KB Video RAM (VRAM)                                                                             | Only bank 0 in Non-CGB mode Switchable bank 0/1 in CGB mode |
| A000        | BFFF      | 8KB External RAM                                                                                 | In cartridge, switchable bank if any
| C000        | CFFF      | 4KB Work RAM (WRAM) bank 0                                                                       | |
| D000        | DFFF      | 4KB Work RAM (WRAM) bank 1\~N                                                                    | Only bank 1 in Non-CGB mode Switchable bank 1\~7 in CGB mode |
| E000        | FDFF      | Mirror of C000\~DDFF (ECHO RAM)                                                                  | Nintendo says use of this area is prohibited. |
| FE00        | FE9F      | Sprite attribute table ([OAM](#vram-sprite-attribute-table-oam))   | |
| FEA0        | FEFF      | Not Usable                                                                                       | Nintendo says use of this area is prohibited |
| FF00        | FF7F      | I/O Registers                                                                                    | |
| FF80        | FFFE      | High RAM (HRAM)                                                                                  | |
| FFFF        | FFFF      | [Interrupts](#interrupts) Enable Register (IE)                                         | |

# Jump Vectors in first ROM bank

The following addresses are supposed to be used as jump vectors:

-   RST commands: 0000, 0008, 0010, 0018, 0020, 0028, 0030, 0038
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

The areas from 0000-7FFF and A000-BFFF address external hardware on
the cartridge, which is essentially an expansion board.  Typically this
is a ROM and SRAM or, more often, a [Memory Bank Controller
(MBC)](#memory-bank-controllers). The RAM area can be read
from and written to normally; writes to the ROM area control the MBC.
some MBCs allow mapping of other hardware into the RAM area in this
way.

Cartridge RAM is often battery buffered to hold saved game positions,
high score tables, and other information when the Game Boy is turned
off.  For specific information read the chapter about Memory Bank
Controllers.

# Echo RAM

The range E000-FDFF is mapped to WRAM, but only the lower 13 bits of
the address lines are connected, with the upper bits on the upper bank
set internally in the memory controller by a bank swap register.  This
causes the address to effectively wrap around.  All reads and writes to
this range have the same effect as reads and writes to C000-DDFF.

Nintendo prohibits developers from using this memory range.  The
behavior is confirmed on all official hardware. Some emulators (such as
VisualBoyAdvance \<1.8) don't emulate Echo RAM.  In some flash cartridges,
echo RAM interferes with SRAM normally at A000-BFFF. Software can check if
Echo RAM is properly emulated by writing to RAM (avoid values 00 and
FF) and checking if said value is mirrored in Echo RAM and not cart SRAM.

# I/O Registers

The Game Boy uses the following I/O ranges:

| **Start** | **End** | **Revision** | **Purpose** |
|-----------|---------|--------------|-------------|
|   $FF00   |  $FF02  |     DMG      | Port/Mode
|   $FF04   |  $FF07  |     DMG      | Port/Mode
|   $FF10   |  $FF26  |     DMG      | Sound
|   $FF30   |  $FF3F  |     DMG      | Waveform RAM
|   $FF40   |  $FF4B  |     DMG      | LCD
|   $FF4F   |         |     CGB      | VRAM Bank Select
|   $FF50   |         |     DMG      | Set to non-zero to disable boot ROM
|   $FF51   |  $FF55  |     CGB      | HDMA
|   $FF68   |  $FF69  |     CGB      | BCP/OCP
|   $FF70   |         |     CGB      | WRAM Bank Select

# FEA0-FEFF range

Nintendo indicates use of this area is prohibited.  This area returns
FF when OAM is blocked, and otherwise the behavior depends on the
hardware revision.

On DMG, MGB, SGB, and SGB2, reads during OAM block trigger the sprite
bug. Reads otherwise return 00.

On CGB revisions 0-D, this area is a unique RAM area, but is masked
with a revision-specific value.

On CGB revision E, AGB, AGS, and GBP, it returns the high nibble of the
lower address byte twice, e.g. FFAx returns AA, FFBx returns BB, and so
forth.
