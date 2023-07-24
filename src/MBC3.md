# MBC3

(max 2MByte ROM and/or 32KByte RAM and Timer)

Beside for the ability to access up to 2MB ROM (128 banks), and 32KB RAM
(4 banks), the MBC3 also includes a built-in Real Time Clock (RTC). The
RTC requires an external 32.768 kHz Quartz Oscillator, and an external
battery (if it should continue to tick when the Game Boy is turned off).

## Memory

### 0000-3FFF - ROM Bank 00 (Read Only)

Contains the first 16 KiB of the ROM.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

Same as for MBC1, except that accessing banks $20, $40, and $60 is
supported now.

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

Depending on the current Bank Number/RTC Register selection (see below),
this memory space is used to access an 8 KiB external RAM Bank, or a
single RTC Register.

## Registers

### A000-BFFF - RTC Register 08-0C (Read/Write)

Depending on the current Bank Number/RTC Register selection (see below),
this memory space is used to access an 8KByte external RAM Bank, or a
single RTC Register.

### 0000-1FFF - RAM and Timer Enable (Write Only)

Mostly the same as for MBC1, a value of $0A will enable reading and
writing to external RAM - and to the RTC Registers! A value of $00 will
disable either.

### 2000-3FFF - ROM Bank Number (Write Only)

Same as for MBC1, except that the whole 7 bits of the ROM Bank Number
are written directly to this address. As for the MBC1, writing a value
of $00 will select Bank $01 instead. All other values $01-$7F select the
corresponding ROM Banks.

### 4000-5FFF - RAM Bank Number - or - RTC Register Select (Write Only)

As for the MBC1s RAM Banking Mode, writing a value in range for $00-$03
maps the corresponding external RAM Bank (if any) into memory at
A000-BFFF. When writing a value of $08-$0C, this will map the
corresponding RTC register into memory at A000-BFFF. That register could
then be read/written by accessing any address in that area, typically
that is done by using address A000.

### 6000-7FFF - Latch Clock Data (Write Only)

When writing $00, and then $01 to this register, the current time
becomes latched into the RTC registers. The latched data will not change
until it becomes latched again, by repeating the write $00-\>$01
procedure. This provides a way to read the RTC registers while the
clock keeps ticking.

### The Clock Counter Registers

```
$08  RTC S   Seconds   0-59 ($00-$3B)
$09  RTC M   Minutes   0-59 ($00-$3B)
$0A  RTC H   Hours     0-23 ($00-$17)
$0B  RTC DL  Lower 8 bits of Day Counter ($00-$FF)
$0C  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag
      Bit 0  Most significant bit of Day Counter (Bit 8)
      Bit 6  Halt (0=Active, 1=Stop Timer)
      Bit 7  Day Counter Carry Bit (1=Counter Overflow)
```

The Halt Flag is supposed to be set before **writing** to the RTC
Registers.

### The Day Counter

The total 9 bits of the Day Counter allow counting days in range from
0-511 ($000-$1FF). The Day Counter Carry Bit becomes set when this value
overflows. In that case the Carry Bit remains set until the program does
reset it. Note that you can store an offset to the Day Counter in
battery RAM. For example, every time you read a non-zero Day Counter,
add this Counter to the offset in RAM, and reset the Counter to zero.
This method allows counting any number of days, making your program
Year-10000-Proof, provided that the cartridge gets used at least every
511 days.

### Delays

When accessing the RTC Registers, it is recommended to wait 4 µs
(4 cycles in Normal Speed Mode) between any separate accesses.
