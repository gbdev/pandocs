# MBC6

MBC6 (Memory Bank Controller 6) is an unusual MBC that contains two
separately switchable ROM banks ($4000 and $6000) and RAM banks
($A000 and $B000), SRAM and an 8 Mbit Macronix MX29F008TC-14 flash
memory chip. It is only used in one game, Net de Get: Minigame @ 100,
which uses the Mobile Adapter to connect to
the web to download mini-games onto the local flash. Both ROM banks and
both RAM banks are views into the same ROM and RAM, but with separately
adjustable offsets. Since the banked regions are smaller the effective
number of banks is twice what it usually would be; 8 KiB ROM banks
instead of 16 KiB and 4 KiB RAM banks instead of 8 KiB.

## Memory

### 0000-3FFF — ROM Bank 00 (Read Only)

Contains the first 16 KiB of the ROM.

### 4000-5FFF — ROM/Flash Bank A 00-7F (Read/Write for flash, Read Only for ROM)

Read-only access to ROM and flash banks 00-7F, switchable independently
of ROM/Flash Bank B.

### 6000-7FFF — ROM/Flash Bank B 00-7F (Read/Write for flash, Read Only for ROM)

Read-only access to ROM and flash banks 00-7F, switchable independently
of ROM/Flash Bank A.

### A000-AFFF — RAM Bank A 00-07 (Read/Write)

Read/write access to RAM banks 00-07, switchable independently of RAM
Bank B.

### B000-BFFF — RAM Bank B 00-07 (Read/Write)

Read/write access to RAM banks 00-07, switchable independently of RAM
Bank A.

## Registers

### 0000-03FF — RAM Enable (Write Only)

Mostly the same as for MBC1, a value of $0A will enable reading and
writing to external RAM. A value of $00 will disable it.

### 0400-07FF — RAM Bank A Number (Write Only)

Select the active RAM Bank A (A000-AFFF)

### 0800-0BFF — RAM Bank B Number (Write Only)

Select the active RAM Bank B (B000-BFFF)

### 0C00-0FFF — Flash Enable (Write Only)

Enable or disable access to the flash chip. Only the lowest bit (0 for
disable, 1 for enable) is used. If disabled, then the /CE (chip enable)
signal from the MBC6 chip to the flash will stay disabled.

### 1000 — Flash Write Enable (Write Only)

Enable or disable write mode for flash sector 0 and the hidden region.
Only the lowest bit (0 for disable, 1 for enable) is used. Note that
this maps to the /WP pin (write protect) on the flash chip, not whether
writing to the bus is enabled; most flash commands still work with this
off so long as Flash Enable is on (see table below). If this register
is set to 0 (disabled), which is the default value after power up, then
neither flash sector 0 nor the hidden flash region can be erased or
programmed. Flash sectors 1 to 7 are not affected by this and can
always be erased and programmed as long as Flash Enable is on.

### 2000-27FF — ROM/Flash Bank A Number (Write Only)

The number for the active bank in ROM/Flash Bank A.

### 2800-2FFF — ROM/Flash Bank A Select (Write Only)

Selects whether the ROM or the Flash is mapped into ROM/Flash Bank A. A
value of 00 selects the ROM and 08 selects the flash.

### 3000-37FF — ROM/Flash Bank B Number (Write Only)

The number for the active bank in ROM/Flash Bank B.

### 3800-3FFF — ROM/Flash Bank B Select (Write Only)

Selects whether the ROM or the Flash is mapped into ROM/Flash Bank B. A
value of 00 selects the ROM and 08 selects the flash.

### Flash Commands

The flash chip is mapped directly into the A or B address space, which
means standard flash access commands are used. To issue a command, you
must write the value $AA to $5555 then $55 to $2AAA and, which are
mapped as 2:5555/1:4AAA for bank A or 2:7555/1:6AAA for bank B followed
by the command at either 2:5555/2:7555, or a relevant address, depending
on the command.

The commands and access sequences are as follows, were X refers to
either 4 or 6 and Y to 5 or 7, depending on the bank region:

```
------------- ------------- ------------- ------------- ------------- ------------- ---------------------------------------------------
2:Y555=$AA    1:XAAA=$55    2:Y555=$80    2:Y555=$AA    1:XAAA=$55    ?:????=$30    Erase sector (set 128 KiB region to $FFs)
2:Y555=$AA    1:XAAA=$55    2:Y555=$80    2:Y555=$AA    1:XAAA=$55    2:Y555=$10    Erase chip (set entire 1 MiB to $FFs)
2:Y555=$AA    1:XAAA=$55    2:Y555=$60    2:Y555=$AA    1:XAAA=$55    2:Y555=$04    Erase hidden region* (set hidden 256 bytes to $FFs)
2:Y555=$AA    1:XAAA=$55    2:Y555=$90                                              ID mode (reads out JEDEC ID (C2,81) at $XXX0,$XXX1)
2:Y555=$AA    1:XAAA=$55    2:Y555=$77    2:Y555=$AA    1:XAAA=$55    2:Y555=$77    Read hidden region
2:Y555=$AA    1:XAAA=$55    2:Y555=$A0                                              Program mode
2:Y555=$AA    1:XAAA=$55    2:Y555=$60    2:Y555=$AA    1:XAAA=$55    2:Y555=$E0    Program mode for hidden region*
2:Y555=$AA    1:XAAA=$55    2:Y555=$60    2:Y555=$AA    1:XAAA=$55    2:Y555=$40    Unprotect sector 0*
2:Y555=$AA    1:XAAA=$55    2:Y555=$60    2:Y555=$AA    1:XAAA=$55    2:Y555=$20    Protect sector 0*
?:????=$F0                                                                          Exit any of the commands above
------------- ------------- ------------- ------------- ------------- ------------- ---------------------------------------------------
```

Commands marked with \* require the Flash Write Enable bit to be 1. The
erase, program and (un-)protect flash commands will make the flash read
out status bytes instead of values. If status bit 7 is set (mask $80) the
operation has finished and you should exit the mode using the $F0
command. Status bit 4 (mask $10) indicates a timeout. Status bit 1 (mask
$02) is set when the sector 0 protection was enabled by the Protect Sector
0 command.

Programming must be done by first erasing a sector, activating program
mode, writing out 128 bytes (aligned), then writing any value (except
$F0) to the final address again to commit the write, waiting for the
status to indicate completion, and writing $F0 to any address to exit
program mode. If a sector is not erased first, programming will not work
properly. In some cases it will only allow the stored bytes to be anded
instead of replaced; in others it just won't work at all. The only way
to set the bits back to 1 is to erase the sector entirely. It is
recommended to check the flash to make sure all bytes were written
properly and re-write (without erasing) the 128 byte block if some bits
didn't get set to 0 properly. After writing all blocks in a sector,
Flash Write Enable should be set to 0.

In addition to the 1 MiB, the flash has a hidden 256 byte region that can
be made accessible by the read hidden region command.

Erasing and programming the hidden 256 byte region works the same as for
the "main" flash, just use the dedicated commands instead. It also needs
to be programmed in 128 byte chunks, so the program hidden region command
needs to be used twice to program the whole 256 bytes.

The last byte of the erase sector command needs to be written to an
address that lies within the sector that you want to erase. There are
eight sectors, 128 KiB each. E.g., to erase sector 2, the last
byte ($30) has to be written to address $40000. The bank number for
that address can be calculated like this: 2 \* 16, where 2 is the sector
number. Therefore, for erasing sector 2, before writing the last byte
($30), write 32 to $2000 to select the bank, and then write $30 to
$4000 or any other address between $4000-$5FFF, the lower address bits
are not relevant.

The erase chip command erases the whole 1 MiB flash. The 256 byte hidden
region is **not** erased by the erase chip command. If sector 0 is
protected, either by the Flash Write Enable bit in MBC6 register 0x1000
or by the Protect Sector 0 flash command (or both), only sectors 1 to 7
are erased.

Sector 0 (the first 128 KiB of the flash) can be protected from erasure
and programming, using the Protect/Unprotect Sector 0 flash commands.
The state of the protection is stored non-volatile. This acts as a second
layer of protection in addition to the Flash Write Enable bit. The Flash
Write Enable bit protects both, sector 0 and the hidden region. The
Protect Sector 0 command only protects sector 0. If you want to make sure
that you can erase and program sector 0, you first have to set the Flash
Write Enable bit to 1, then issue the Unprotect Sector 0 flash command.

## External links

- Source: [GBDev Forums thread by endrift](https://web.archive.org/web/20241204030511/https://gbdev.gg8.se/forums/viewtopic.php?id=544)
- Reference: [Nintendo Power Game Boy Memory cartridge documentation](https://iceboy.a-singer.de/doc/np_gb_memory.html)
  * The NP GB Memory cartridges use a nearly identical flash chip.
    It seemingly only slightly differs in its part number. Therefore,
    the part about the flash chip also applies to Net de Get.
