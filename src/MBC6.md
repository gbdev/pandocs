# MBC6

MBC6 (Memory Bank Controller 6) is an unusual MBC that contains two
separately switchable ROM banks ($4000 and $6000) and RAM banks
($A000 and $B000), SRAM and an 8 Mbit Macronix MX29F008TC-14 flash
memory chip. It is only used in one game, Net de Get: Minigame @ 100,
which uses the Mobile Adapter to connect to
the web to download mini-games onto the local flash. Both ROM banks and
both RAM banks are views into the same ROM and RAM, but with separately
adjustable offsets. Since the banked regions are smaller the effective
number of banks is twice what it usually would be; 8 kB ROM banks
instead of 16 kB and 4 kB RAM banks instead of 8 kB.

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
disable, 1 for enable) is used. Flash Write Enable must be active to
change this.

### 1000 — Flash Write Enable (Write Only)

Enable or disable write mode for the flash chip. Only the lowest bit (0
for disable, 1 for enable) is used. Note that this maps to the /WE pin
on the flash chip, not whether writing to the bus is enabled;
some flash commands (e.g. JEDEC ID query) still work with this off so
long as Flash Enable is on.

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
------------- ------------- ------------- ------------- ------------- ------------- ------------------------------------------------
2:Y555=$AA    1:XAAA=$55    2:Y555=$80    2:Y555=$AA    1:XAAA=$55    ?:X000=$30    Erase sector\* (set 8 kB region to $FFs)
2:Y555=$AA    1:XAAA=$55    2:Y555=$80    2:Y555=$AA    1:XAAA=$55    ?:Y555=$10    Erase chip\* (set entire flash to $FFs)
2:Y555=$AA    1:XAAA=$55    2:Y555=$90                                                 ID mode (reads out JEDEC ID (C2,81) at $X000)
2:Y555=$AA    1:XAAA=$55    2:Y555=$A0                                                 Program mode\*
2:Y555=$AA    1:XAAA=$55    2:Y555=$F0                                                 Exit ID/erase chip mode
2:Y555=$AA    1:XAAA=$55    ?:X000=$F0                                                 Exit erase sector mode
?:????=$F0                                                                               Exit program mode
------------- ------------- ------------- ------------- ------------- ------------- ------------------------------------------------
```

Commands marked with \* require the Write Enable bit to be 1. These will
make the flash read out status bytes instead of values. A status of $80
means the operation has finished and you should exit the mode using the
appropriate command. A status of $10 indicates a timeout.

Programming must be done by first erasing a sector, activating write
mode, writing out 128 bytes (aligned), then writing a 0 to the final
address to commit the write, waiting for the status to indicate
completion, and writing $F0 to the final address again to exit program
mode. If a sector is not erased first programming will not work
properly. In some cases it will only allow the stored bytes to be anded
instead of replaced; in others it just won't work at all. The only way
to set the bits back to 1 is to erase the sector entirely. It is
recommended to check the flash to make sure all bytes were written
properly and re-write (without erasing) the 128 byte block if some bits
didn't get set to 0 properly. After writing all blocks in a sector
Flash Write Enable should be set to 0.

## External links

- Source: [GBDev Forums thread by endrift](http://gbdev.gg8.se/forums/viewtopic.php?id=544)
