# MBC7

MBC7 (Memory Bank Controller 7) is an MBC containing a 2-axis
accelerometer (ADXL202E) and a 256 byte EEPROM
([93LC56](https://web.archive.org/web/20230115175018/https://www.microchip.com/en-us/product/93LC56)). A000-BFFF
does not directly address the EEPROM, as most MBCs do, but rather
contains several registers that can be read or written one at a time.
This makes EEPROM access very slow due to needing multiple writes per
address.

## Memory

### 0000-3FFF - ROM Bank 00 (Read Only)

Contains the first 16 KiB of the ROM.

### 4000-7FFF - ROM Bank 00-7F (Read Only)

Same as for MBC5. (Bank 0 mapping needs confirmation)

## Registers

### A000-AFFF - RAM Registers (Read/Write)

Must be enabled via 0000 and 4000 region writes (see respective
sections), otherwise reads read $FF and writes do nothing. Registers are
addressed through bits 4-7 of the address. Bits 0-3 and 8-11 are
ignored.

Accelerometer data must be latched before reading. Data is 16-bit and
centered at the value 81D0. Earth\'s gravity affects the value by
roughly $70, with larger acceleration providing a larger range. Maximum
range is unknown.

### Ax0x/Ax1x - Latch Accelerometer (Write Only)

Write $55 to Ax0x to erase the latched data (reset back to 8000) then
$AA to Ax1x to latch the accelerometer and update the addressable
registers. Reads return $FF. Other writes do not appear to do anything
(Partially unconfirmed). Note that you cannot re-latch the accelerometer
value without first erasing it; attempts to do so yield no change.

### Ax2x/Ax3x - Accelerometer X value (Read Only)

Ax2x contains the low byte of the X value (lower values are towards the
right and higher values are towards the left), and Ax3x contains the
high byte. Reads 8000 before first latching.

### Ax4x/Ax5x - Accelerometer Y value (Read Only)

Ax4x contains the low byte of the Y value (lower values are towards the
bottom and higher values are towards the top), and Ax5x contains the
high byte. Reads 8000 before first latching.

### Ax6x/Ax7x - Unknown

Ax6x always reads $00 and Ax7x always reads $FF. Possibly reserved for Z
axis, which does not exist on this accelerometer.

### Ax8x - EEPROM (Read/Write)

Values in this register correspond to 4 pins on the EEPROM:

-   Bit 0: Data Out (DO)
-   Bit 1: Data In (DI)
-   Bit 6: Clock (CLK or SK in existing code)
-   Bit 7: Chip Select (CS)

The other pins (notably ORG, which controls 8-bit vs 16-bit addressing)
do not appear to be connected to this register.

Commands are sent to the EEPROM by shifting in a bitstream to DI while
manually clocking CLK. All commands must be preceded by a 1 bit, and
existing games precede the 1 bit with a 0 bit (though this is not
necessary):

-   Write $00 (lower CS)
-   Write $80 (raise CS)
-   Write $C0 (shift in 0 bit)
-   Write $82 (lower CS, raise DI)
-   Write $C2 (shift in 1 bit)
-   Write command

The following commands exist, each 10 bits (excluding data shifted in or
out). \"x\" means the value of this bit is ignored. \"A\" means the
relevant bit of the address. All data is shifted in or out MSB first.
Note that data is addressed 16 bits at a time, so address 1 corresponds
to bits 16-31, thus bytes 2-3.

-   READ: 10xAAAAAAAb (then shift out 16 bits)
-   EWEN (Erase/Write enable): 0011xxxxxxb
-   EWDS (Erase/Write disable): 0000xxxxxxb
-   WRITE: 01xAAAAAAAb (then shift in 16 bits)
-   ERASE (fill address with FFFF): 11xAAAAAAAb
-   ERAL (fill EEPROM with FFFF): 0010xxxxxxb
-   WRAL (fill EEPROM with value): 0001xxxxxxb (then shift in 16 bits)

All programming operations (WRITE/ERASE/WRAL/ERAL) must be preceded with
EWEN.

According to the datasheet, programming operations take time to settle.
Continue clocking and check the value of DO to verify if command is
still running. Data sheet says that the signal to DO is RDY, thus it
reads a 1 when the command finishes.

Datasheet:
[1](http://ww1.microchip.com/downloads/en/DeviceDoc/21712C.pdf)

### Ax9x-AxFx - Unused

Reads out $FF.

### B000-BFFF - Unknown

Only seems to read out $FF.

### 0000-1FFF - RAM Enable 1 (Write Only)

Mostly the same as for MBC1, a value of $0A will enable reading and
writing to RAM registers. A value of $00 will disable it. Please note
that the RAM must second be enabled in the second RAM enable section as
well (4000-5FFF)

### 2000-3FFF - ROM Bank Number (Write Only)

The ROM bank number goes here.

### 4000-5FFF - RAM Enable 2 (Write Only)

Writing $40 to this region enables access to the RAM registers. Writing
any other value appears to disable access to RAM, but this is not fully
tested. Please note that the RAM must first be enabled in the first RAM
enable section as well (0000-1FFF)

## External links

- Source: [GBDev Forums thread by endrift](http://gbdev.gg8.se/forums/viewtopic.php?id=448)

