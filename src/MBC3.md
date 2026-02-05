# MBC3

(max 4 MiB ROM, 64 KiB RAM, and timer)

Beside for the ability to access up to 4 MiB ROM (256 banks) and 64 KiB RAM (8 banks), this MBC also includes a built-in Real Time Clock (RTC), sometimes referred to as the timer. The RTC requires an external 32.768 kHz quartz crystal oscillator, and an external battery (if it should continue to tick when the Game Boy is turned off). All official releases with this mapper utilize cartridge RAM and a battery, but a few DMG games don't use the timer and therefore lack the crystal on their PCB.

There are (at least) four different versions of this MBC that can be distinguished by the print on the chip itself: MBC3, MBC3A, MBC3B and MBC30. Only the latter supports the full 4 MiB of ROM and 64 KiB of RAM whereas the other three can only access half of that. The MBC30 is only found in the Japanese _Pocket Monsters: Crystal Version_, which is also the only release with the full 64 KiB of RAM (8 banks). No game uses the full 4 MiB.

The different versions of the chip are not distinguished in the cartridge header, not even the MBC30.

## Memory

### 0000-3FFF - ROM Bank 00 (Read Only)

Contains the first 16 KiB of the ROM.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

Same as for MBC1, except that accessing banks $20, $40, and $60 is supported now.

### A000-BFFF - RAM Bank 00-07 (Read/Write)

This memory space is used to access an 8 KiB external RAM bank. It can also be used to access one of the clock counter registers (see 4000-5FFF register).

## Registers

### A000-BFFF - Clock Counter Registers 08-0C (Read/Write)

If this area is set up to represent one of the clock counter registers (see 4000-5FFF register), that register can be read/written by accessing any address in that area, typically using address A000.

### 0000-1FFF - RAM and Timer Enable (Write Only)

Mostly the same as for MBC1, a value of $0A will enable reading and writing to external RAM - and to the clock counter registers. A value of $00 will disable either.

### 2000-3FFF - ROM Bank Number (Write Only)

Same as for MBC1, except that the whole up to 8 bits of the ROM bank number are written directly to this address. As for the MBC1, writing a value of $00 will select Bank $01 instead. All other values $01-$FF select the corresponding ROM banks.

### 4000-5FFF - RAM Bank Number - or - RTC Register Select (Write Only)

Controls what is mapped into memory at A000-BFFF:

| Value   | Selection                                     |
|---------|-----------------------------------------------|
| $00-$07 | The corresponding RAM bank.                   |
| $08-$0C | The corresponding clock counter register (see below).    |

If the low nibble is $D, $E or $F, or if the bank number is not available on that cartridge (or if RAM is turned off), the A000-BFFF area returns $FF. The high nibble is ignored.


### 6000-7FFF - Latch Clock Data (Write Only)

Latching makes a static copy of the current timestamp available in the clock counter registers while the clock keeps running in the background. This makes sure that your reads from the counter registers will be consistent, since any counter overflowing while you read the different parts can have you read an incorrect value (e.g. reading the minute at 11:59 and the hour at 12:00 will give 12:59.)

The exact behavior of this register varies depending on hardware:

MBC3B provides a running clock on power-on and after writing any even value to this register. It is still recommended to latch the clock by writing any odd value. MBC3B can only latch while it provides a running clock, so you must write an even value before you can write an odd value again.

MBC3A's clock counter registers are indeterminate when powered on. Writing any value to this register latches the clock. MBC3A cannot provide a running clock. Naturally, it can latch repeatedly.

As a result, most games write $00 and then $01 to this register. This procedure will latch the clock on both chips, even if it is already latched.

:::tip

**Help wanted**

The exact latching behavior of MBC3 and MBC30 has not been tested and the sample size could be improved even for the MBC3A and MBC3B.

If you would like to help, have a flashcart and any official RTC cartridge, please reach out to us on gbdev Discord so you can be given the test ROMs.

:::

### Clock Counter Registers
| Register | Name | Description | Range |
|----------|------|-------------|-------|
| $08 | RTC S | Seconds | 0-59 ($00-$3B) |
| $09 | RTC M | Minutes | 0-59 ($00-$3B) |
| $0A | RTC H | Hours | 0-23 ($00-$17) |
| $0B | RTC DL | Lower 8 bits of day counter | ($00-$FF) |
| $0C | RTC DH | Upper 1 bit of day counter, carry bit, halt flag. <br>Bit 0: Most significant bit (Bit 8) of day counter<br>Bit 6: Halt (0=active, 1=stop the timer)<br>Bit 7: Day counter carry bit (1=counter overflowed) | |

The halt flag is supposed to be set before **writing** to the clock counter registers. This makes sure that no register overflows while you write the different parts. However, it is not required that you halt (or latch) the clock before you write to the registers. Note that latching also prevents you from seeing your writes reflected immediately.

Bits that are not required to store the above information will be ignored and always read 0.

You can write values larger than the ones mentioned above (up to 63 for seconds and minutes, and up to 31 for hours). Invalid values will then continue incrementing like a valid value and will only overflow once the available bits no longer suffice. This overflow however will not cause a carry, neither does writing 60 or 24 directly. For example, if you write 30:59:63 (and clear the Halt Flag), it will be 30:59:00 one second later, and 31:00:00 one minute after that.

Writing to the seconds register also resets the inaccessible sub-second counter.

### The Day Counter

The total 9 bits of the Day Counter allow counting days in range from 0-511 ($000-$1FF). The day counter carry bit becomes set when this value overflows and remains set until the program clears it. Note that you can store an offset to the day counter in battery RAM. For example, every time you read a non-zero day counter, add the counter to the offset in RAM, and reset the counter to zero. This method allows counting any number of days, provided that the cartridge gets used at least every 511 days.

### Delays

When accessing the clock counter registers, it is recommended to wait 4 µs (4 M-cycles in normal speed mode) between any separate accesses.
