The gameboy is having a 16bit address bus, that is used to address ROM,
RAM, and I/O registers.

General Memory Map
------------------

` 0000-3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)`\
` 4000-7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)`\
` 8000-9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)`\
` A000-BFFF   8KB External RAM     (in cartridge, switchable bank, if any)`\
` C000-CFFF   4KB Work RAM Bank 0 (WRAM)`\
` D000-DFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)`\
` E000-FDFF   Same as C000-DDFF (ECHO)    (typically not used)`\
` FE00-FE9F   Sprite Attribute Table (OAM)`\
` FEA0-FEFF   Not Usable`\
` FF00-FF7F   I/O Ports`\
` FF80-FFFE   High RAM (HRAM)`\
` FFFF        Interrupt Enable Register`

Jump Vectors in First ROM Bank
------------------------------

The following addresses are supposed to be used as jump vectors:

` 0000,0008,0010,0018,0020,0028,0030,0038   for RST commands`\
` 0040,0048,0050,0058,0060                  for Interrupts`

However, the memory may be used for any other purpose in case that your
program doesn\'t use any (or only some) RST commands or Interrupts. RST
commands are 1-byte opcodes that work similiar to CALL opcodes, except
that the destination address is fixed.

Cartridge Header in First ROM Bank
----------------------------------

The memory at 0100-014F contains the [cartridge
header](The_Cartridge_Header "wikilink"). This area contains information
about the program, its entry point, checksums, information about the
used MBC chip, the ROM and RAM sizes, etc. Most of the bytes in this
area are required to be specified correctly. For more information read
the chapter about The Cartridge Header.

External Memory and Hardware
----------------------------

The areas from 0000-7FFF and A000-BFFF may be used to connect external
hardware. The first area is typically used to address ROM (read only, of
course), cartridges with [Memory Bank Controllers
(MBCs)](Memory_Bank_Controllers "wikilink") are additionally using this
area to output data (write only) to the MBC chip. The second area is
often used to address external RAM, or to address other external
hardware (Real Time Clock, etc). External memory is often battery
buffered, and may hold saved game positions and high score tables (etc.)
even when the gameboy is turned of, or when the cartridge is removed.
For specific information read the chapter about Memory Bank Controllers.

