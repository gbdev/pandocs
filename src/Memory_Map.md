# Memory Map

The Game Boy has a 16-bit address bus, which is used to address ROM, RAM, and I/O.

Start       | End       | Description                                                      | Notes
------------|-----------|------------------------------------------------------------------|----------
0000        | 3FFF      | 16 KiB ROM bank 00                                               | From cartridge, usually a fixed bank
4000        | 7FFF      | 16 KiB ROM Bank 01\~NN                                           | From cartridge, switchable bank via [mapper](#MBCs) (if any)
8000        | 9FFF      | 8 KiB Video RAM (VRAM)                                           | In CGB mode, switchable bank 0/1
A000        | BFFF      | 8 KiB External RAM                                               | From cartridge, switchable bank if any
C000        | CFFF      | 4 KiB Work RAM (WRAM)                                            |
D000        | DFFF      | 4 KiB Work RAM (WRAM)                                            | In CGB mode, switchable bank 1\~7
E000        | FDFF      | Mirror of C000\~DDFF (ECHO RAM)                                  | Nintendo says use of this area is prohibited.
FE00        | FE9F      | [Object attribute memory (OAM)](<#Object Attribute Memory (OAM)>) |
FEA0        | FEFF      | Not Usable                                                       | Nintendo says use of this area is prohibited
FF00        | FF7F      | [I/O Registers](<#I/O Ranges>)                                   |
FF80        | FFFE      | High RAM (HRAM)                                                  |
FFFF        | FFFF      | [Interrupt](#Interrupts) Enable register (IE)                    |

## Jump Vectors in first ROM bank

The following addresses are supposed to be used as jump vectors:

-   RST instructions: 0000, 0008, 0010, 0018, 0020, 0028, 0030, 0038
-   Interrupts: 0040, 0048, 0050, 0058, 0060

However, this memory area (0000-00FF) may be used for any other purpose in case that your
program doesn't use any (or only some) [`rst`](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7/#RST_vec) instructions or interrupts. `rst`
is a 1-byte instruction that works similarly to the 3-byte `call` instruction, except
that the destination address is restricted. Since it is 1-byte sized,
it is also slightly faster.

## Cartridge Header in first ROM bank

The memory area at 0100-014F contains the [cartridge
header](<#The Cartridge Header>). This area contains information
about the program, its entry point, checksums, information about the
used MBC chip, the ROM and RAM sizes, etc. Most of the bytes in this
area are required to be specified correctly.

## External Memory and Hardware

The areas from 0000-7FFF and A000-BFFF address external hardware on
the cartridge, which is essentially an expansion board.  Typically this
is a ROM and SRAM or, more often, a [Memory Bank Controller
(MBC)](#MBCs). The RAM area can be read
from and written to normally; writes to the ROM area control the MBC.
Some MBCs allow mapping of other hardware into the RAM area in this
way.

Cartridge RAM is often battery buffered to hold saved game positions,
high score tables, and other information when the Game Boy is turned
off.  For specific information read the chapter about [Memory Bank
Controllers](<#MBCs>).

## Echo RAM

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

## I/O Ranges

The Game Boy uses the following I/O ranges:

Start   | End     | First appeared | Purpose
--------|---------|----------------|-------------
$FF00   |         |       DMG      | [Joypad input](<#FF00 — P1/JOYP: Joypad>)
$FF01   |  $FF02  |       DMG      | [Serial transfer](<#Serial Data Transfer (Link Cable)>)
$FF04   |  $FF07  |       DMG      | [Timer and divider](<#Timer and Divider Registers>)
$FF10   |  $FF26  |       DMG      | [Audio](<#Audio Registers>)
$FF30   |  $FF3F  |       DMG      | [Wave pattern](<#FF30–FF3F — Wave pattern RAM>)
$FF40   |  $FF4B  |       DMG      | LCD [Control](<#FF40 — LCDC: LCD control>), [Status](<#FF41 — STAT: LCD status>), [Position, Scrolling](<#LCD Position and Scrolling>), and [Palettes](<#Palettes>)
$FF4F   |         |       CGB      | [VRAM Bank Select](<#FF4F — VBK (CGB Mode only): VRAM bank>)
$FF50   |         |       DMG      | Set to non-zero to disable boot ROM
$FF51   |  $FF55  |       CGB      | [VRAM DMA](<#LCD VRAM DMA Transfers>)
$FF68   |  $FF6B  |       CGB      | [BG / OBJ Palettes](<#LCD Color Palettes (CGB only)>)
$FF70   |         |       CGB      | [WRAM Bank Select](<#FF70 — SVBK (CGB Mode only): WRAM bank>)

## FEA0-FEFF range

Nintendo indicates use of this area is prohibited.  This area returns
$FF when OAM is blocked, and otherwise the behavior depends on the
hardware revision.

- On DMG, MGB, SGB, and SGB2, reads during OAM block trigger [OAM corruption](<#OAM Corruption Bug>).
Reads otherwise return $00.

- On CGB revisions 0-D, this area is a unique RAM area, but is masked
with a revision-specific value.

- On CGB revision E, AGB, AGS, and GBP, it returns the high nibble of the
lower address byte twice, e.g. FFAx returns $AA, FFBx returns $BB, and so
forth.
