As the gameboys 16 bit address bus offers only limited space for ROM and
RAM addressing, many games are using Memory Bank Controllers (MBCs) to
expand the available address space by bank switching. These MBC chips
are located in the game cartridge (ie. not in the gameboy itself).

In each cartridge, the required (or preferred) MBC type should be
specified in the byte at 0147h of the ROM, as described in [the
cartridge header](The_Cartridge_Header#0148_-_ROM_Size "wikilink").
Several different MBC types are available:

\_\_TOC\_\_

None (32KByte ROM only)
-----------------------

Small games of not more than 32KBytes ROM do not require a MBC chip for
ROM banking. The ROM is directly mapped to memory at 0000-7FFFh.
Optionally up to 8KByte of RAM could be connected at A000-BFFF, even
though that could require a tiny MBC-like circuit, but no real MBC chip.

MBC1 (max 2MByte ROM and/or 32KByte RAM)
----------------------------------------

This is the first MBC chip for the gameboy. Any newer MBC chips are
working similiar, so that is relative easy to upgrade a program from one
MBC chip to another - or even to make it compatible to several different
types of MBCs.

Note that the memory in range 0000-7FFF is used for both reading from
ROM, and for writing to the MBCs Control Registers.

### 0000-3FFF - ROM Bank 00 (Read Only)

This area always contains the first 16KBytes of the cartridge ROM.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

This area may contain any of the further 16KByte banks of the ROM,
allowing to address up to 125 ROM Banks (almost 2MByte). As described
below, bank numbers 20h, 40h, and 60h cannot be used, resulting in the
odd amount of 125 banks.

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

This area is used to address external RAM in the cartridge (if any).
External RAM is often battery buffered, allowing to store game positions
or high score tables, even if the gameboy is turned off, or if the
cartridge is removed from the gameboy. Available RAM sizes are: 2KByte
(at A000-A7FF), 8KByte (at A000-BFFF), and 32KByte (in form of four 8K
banks at A000-BFFF).

### 0000-1FFF - RAM Enable (Write Only)

Before external RAM can be read or written, it must be enabled by
writing to this address space. It is recommended to disable external RAM
after accessing it, in order to protect its contents from damage during
power down of the gameboy. Usually the following values are used:

` 00h  Disable RAM (default)`\
` 0Ah  Enable RAM`

Practically any value with 0Ah in the lower 4 bits enables RAM, and any
other value disables RAM.

### 2000-3FFF - ROM Bank Number (Write Only)

Writing to this address space selects the lower 5 bits of the ROM Bank
Number (in range 01-1Fh). When 00h is written, the MBC translates that
to bank 01h also. That doesn\'t harm so far, because ROM Bank 00h can be
always directly accessed by reading from 0000-3FFF. But (when using the
register below to specify the upper ROM Bank bits), the same happens for
Bank 20h, 40h, and 60h. Any attempt to address these ROM Banks will
select Bank 21h, 41h, and 61h instead.

### 4000-5FFF - RAM Bank Number - or - Upper Bits of ROM Bank Number (Write Only)

This 2bit register can be used to select a RAM Bank in range from
00-03h, or to specify the upper two bits (Bit 5-6) of the ROM Bank
number, depending on the current ROM/RAM Mode. (See below.)

### 6000-7FFF - ROM/RAM Mode Select (Write Only)

This 1bit Register selects whether the two bits of the above register
should be used as upper two bits of the ROM Bank, or as RAM Bank Number.

` 00h = ROM Banking Mode (up to 8KByte RAM, 2MByte ROM) (default)`\
` 01h = RAM Banking Mode (up to 32KByte RAM, 512KByte ROM)`

The program may freely switch between both modes, the only limitiation
is that only RAM Bank 00h can be used during Mode 0, and only ROM Banks
00-1Fh can be used during Mode 1.

MBC2 (max 256KByte ROM and 512x4 bits RAM)
------------------------------------------

### 0000-3FFF - ROM Bank 00 (Read Only)

Same as for MBC1.

### 4000-7FFF - ROM Bank 01-0F (Read Only)

Same as for MBC1, but only a total of 16 ROM banks is supported.

### A000-A1FF - 512x4bits RAM, built-in into the MBC2 chip (Read/Write)

The MBC2 doesn\'t support external RAM, instead it includes 512x4 bits
of built-in RAM (in the MBC2 chip itself). It still requires an external
battery to save data during power-off though. As the data consists of
4bit values, only the lower 4 bits of the \"bytes\" in this memory area
are used.

### 0000-1FFF - RAM Enable (Write Only)

The least significant bit of the upper address byte must be zero to
enable/disable cart RAM. For example the following addresses can be used
to enable/disable cart RAM: 0000-00FF, 0200-02FF, 0400-04FF, \...,
1E00-1EFF. The suggested address range to use for MBC2 ram
enable/disable is 0000-00FF.

### 2000-3FFF - ROM Bank Number (Write Only)

Writing a value (XXXXBBBB - X = Don\'t cares, B = bank select bits) into
2000-3FFF area will select an appropriate ROM bank at 4000-7FFF.

The least significant bit of the upper address byte must be one to
select a ROM bank. For example the following addresses can be used to
select a ROM bank: 2100-21FF, 2300-23FF, 2500-25FF, \..., 3F00-3FFF. The
suggested address range to use for MBC2 rom bank selection is 2100-21FF.

MBC3 (max 2MByte ROM and/or 64KByte RAM and Timer)
--------------------------------------------------

Beside for the ability to access up to 2MB ROM (128 banks), and 64KB RAM
(8 banks), the MBC3 also includes a built-in Real Time Clock (RTC). The
RTC requires an external 32.768 kHz Quartz Oscillator, and an external
battery (if it should continue to tick when the gameboy is turned off).

### 0000-3FFF - ROM Bank 00 (Read Only)

Same as for MBC1.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

Same as for MBC1, except that accessing banks 20h, 40h, and 60h is
supported now.

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

### A000-BFFF - RTC Register 08-0C (Read/Write)

Depending on the current Bank Number/RTC Register selection (see below),
this memory space is used to access an 8KByte external RAM Bank, or a
single RTC Register.

### 0000-1FFF - RAM and Timer Enable (Write Only)

Mostly the same as for MBC1, a value of 0Ah will enable reading and
writing to external RAM - and to the RTC Registers! A value of 00h will
disable either.

### 2000-3FFF - ROM Bank Number (Write Only)

Same as for MBC1, except that the whole 7 bits of the RAM Bank Number
are written directly to this address. As for the MBC1, writing a value
of 00h, will select Bank 01h instead. All other values 01-7Fh select the
corresponding ROM Banks.

### 4000-5FFF - RAM Bank Number - or - RTC Register Select (Write Only)

As for the MBC1s RAM Banking Mode, writing a value in range for 00h-07h
maps the corresponding external RAM Bank (if any) into memory at
A000-BFFF. When writing a value of 08h-0Ch, this will map the
corresponding RTC register into memory at A000-BFFF. That register could
then be read/written by accessing any address in that area, typically
that is done by using address A000.

### 6000-7FFF - Latch Clock Data (Write Only)

When writing 00h, and then 01h to this register, the current time
becomes latched into the RTC registers. The latched data will not change
until it becomes latched again, by repeating the write 00h-\>01h
procedure. This is supposed for <reading> from the RTC registers. This
can be proven by reading the latched (frozen) time from the RTC
registers, and then unlatch the registers to show the clock itself
continues to tick in background.

### The Clock Counter Registers

` 08h  RTC S   Seconds   0-59 (0-3Bh)`\
` 09h  RTC M   Minutes   0-59 (0-3Bh)`\
` 0Ah  RTC H   Hours     0-23 (0-17h)`\
` 0Bh  RTC DL  Lower 8 bits of Day Counter (0-FFh)`\
` 0Ch  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag`\
`       Bit 0  Most significant bit of Day Counter (Bit 8)`\
`       Bit 6  Halt (0=Active, 1=Stop Timer)`\
`       Bit 7  Day Counter Carry Bit (1=Counter Overflow)`

The Halt Flag is supposed to be set before <writing> to the RTC
Registers.

### The Day Counter

The total 9 bits of the Day Counter allow to count days in range from
0-511 (0-1FFh). The Day Counter Carry Bit becomes set when this value
overflows. In that case the Carry Bit remains set until the program does
reset it. Note that you can store an offset to the Day Counter in
battery RAM. For example, every time you read a non-zero Day Counter,
add this Counter to the offset in RAM, and reset the Counter to zero.
This method allows to count any number of days, making your program
Year-10000-Proof, provided that the cartridge gets used at least every
511 days.

### Delays

When accessing the RTC Registers it is recommended to execute a 4ms
delay (4 Cycles in Normal Speed Mode) between the separate accesses.

MBC5 (max 8MByte ROM and/or 128KByte RAM)
-----------------------------------------

### 0000-3FFF - ROM Bank 00 (Read Only)

Same as for MBC1.

### 4000-7FFF - ROM Bank 00-1FF (Read Only)

Same as for MBC1, except that accessing up to bank 1E0h is supported
now. Also, bank 0 is actually bank 0.

### A000-BFFF - RAM Bank 00-0F, if any (Read/Write)

Same as for MBC1, except RAM sizes are 8KiB, 32KiB and 128KiB.

### 0000-1FFF - RAM Enable (Write Only)

Mostly the same as for MBC1, a value of 0Ah will enable reading and
writing to external RAM. A value of 00h will disable it.

### 2000-2FFF - Low 8 bits of ROM Bank Number (Write Only)

The lower 8 bits of the ROM bank number goes here. Writing 0 will indeed
give bank 0 on MBC5, unlike other MBCs.

### 3000-3FFF - High bit of ROM Bank Number (Write Only)

The 9th bit of the ROM bank number goes here.

### 4000-5FFF - RAM Bank Number (Write Only)

As for the MBC1s RAM Banking Mode, writing a value in range for 00h-0Fh
maps the corresponding external RAM Bank (if any) into memory at
A000-BFFF.

HuC1 (MBC with Infrared Controller)
-----------------------------------

This controller (made by Hudson Soft) appears to be very similar to an
MBC1 with the main difference being that it supports infrared LED input
/ output. (Similiar to the infrared port that has been later invented in
CGBs.)

The Japanese cart \"Fighting Phoenix\" (internal cart name: SUPER B
DAMAN) is known to contain this chip.

Multicart MBCs
--------------

**MBC1M** uses the MBC1 IC, but the board does not connect the MBC1\'s
A18 address output to the ROM. This allows including multiple 2 Mbit (16
bank) games, with SRAM bank select (\$4000) to select which of up to
four games is switched in. In theory, a MBC1M board could be made for 1
Mbit or 512 kbit games by additionally not connecting A17 and A16
outputs, but this appears not to have been done in licensed games.

**MMM01** is a more complex that allows for games of different sizes
[Docs on Tauwasser.eu](https://wiki.tauwasser.eu/view/MMM01)

**Bung** and **EMS** MBCs are reported to exist.

MBC Timing Issues
-----------------

Using MBCs with CGB Double Speed Mode The MBC5 has been designed to
support CGB Double Speed Mode. There have been rumours that older MBCs
(like MBC1-3) wouldn\'t be fast enough in that mode. If so, it might be
nethertheless possible to use Double Speed during periods which use only
code and data which is located in internal RAM. However, despite of the
above, my own good old selfmade MBC1-EPROM card appears to work stable
and fine even in Double Speed Mode though.

