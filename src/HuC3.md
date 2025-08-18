# HuC-3

HuC-3 is an MBC developed by Hudson Soft. Besides ROM and RAM banking,
it also provides a real-time clock, speaker, and infrared communication.
The CR2025 coin cell is user-replaceable. It is a successor to the
[HuC1](./HuC1.md).

The HuC-3 is poorly understood. Observed behavior suggests the
real-time clock and tone generator are implemented using a 4-bit
microcontroller core with internal program ROM.

## Memory

### 0000–3FFF — ROM Bank 00 \[read-only\]

Contains the first 16 KiB of the ROM.

### 4000–7FFF — ROM Bank 00–7F \[read-only\]

This area may contain any of the further 16 KiB banks of the ROM. Like the MBC5,
bank $00 can also be mapped to this region.

### A000–BFFF — RAM Bank 00–03, or RTC/IR register \[read/write\]

Depending on the current register selection and RAM Bank Number (see
below), this memory space is used to access an 8 KiB external RAM Bank,
or a single I/O Register.

## Memory Control Registers

### 0000–1FFF — RAM/RTC/IR Select \[read/write\]

Writing to this register maps cart RAM, RTC registers or IR registers
into memory at $A000–BFFF. Only the lower 4 bits are significant.

Value | Feature
------|-----------------------------
  $0  | Cart RAM (read-only)
  $A  | Cart RAM (read/write)
  $B  | RTC command/argument (write)
  $C  | RTC command/response (read)
  $D  | RTC semaphore (read/write)
  $E  | IR (read/write)

If any other value is written, reads from $A000–BFFF return open bus
values (often $FF, but not guaranteed), and writes are ignored.

### 2000–3FFF — ROM Bank Number [write-only]

HuC-3 can accept a 7-bit bank number here.

### 4000–5FFF — RAM Bank Select [write-only]

HuC-3 can accept a bank number of at least 2 bits here.

### 6000–7FFF — Nothing? [write-only]

Games write $01 here on startup, but it doesn’t seem to have any
observable effect.

## I/O Registers

These registers are accessed by reading or writing in the $A000–BFFF
range after setting the RAM/RTC/IR Select register. Address lines
A12–A0 are not connected to the HuC-3 chip, so the offset within the
range is ignored. Data line D7 is not connected to the HuC-3 chip, so
the most significant bit of reads always returns an open bus value
(usually high), and the most significant bit of writes is always
ignored.

The RTC MCU communication protocol is described below.

### $B — RTC Command/Argument \[write\]

The value written consists of a command in bits 6-4, and an argument
value in bits 3–0. For example the value $62 is command $6 with argument
value $2. Writing to this register just sets the values in the mailbox
registers – it does not cause the command to be executed.

### $C — RTC Command/Response \[read\]

When read, bits 6–4 return the last command written to register $B, and
bits 3–0 contain the result from the last command executed.

### $D — RTC Semaphore \[read/write\]

When reading, the least significant bit is high when the RTC MCU is
ready to receive a command, or low when the RTC MCU is busy.

Writing with the least significant bit clear requests that the RTC MCU
execute the last command written to register $B.

### $E — IR \[read/write\]

Similar to the equivalent register of the HuC1. The least significant
bit is used for infrared communication.

## RTC Communication Protocol

The HuC-3 chip provides read and write access to a 256-nybble window
into the RTC MCU’s internal memory. Multi-nybble values are stored with
the least significant nybble at the lowest address (little Endian).
There are commands for setting the access address, reading and writing
values, and a few higher-level operations.

Games use the following sequence to execute a command:

* Write $0D to $0000 (select RTC Semaphore register)
* Poll $A000 until least significant bit is set (wait until ready)
* Write $0B to $0000 (select RTC Command/Argument register)
* Write command and argument to $A000
* Write $0D to $0000 (select RTC semaphore register)
* Write $FE to $A000 (clear semaphore, requesting MCU execute command)
* Poll $A000 until least significant bit is set (wait for completion)
* Write $0C to $0000 (select RTC Command/Response register)
* Read $A000 and use value from 4 least significant bits

(The last two steps are not applicable for commands that don’t produce a
response.)

Known commands:

Command | Description
--------|-----------------------------------------------
  $1    | Read value and increment access address
  $3    | Write value and increment access address
  $4    | Set access address least significant nybble
  $5    | Set access address most significant nybble
  $6    | Execute extended command specified by argument

Pocket Family GB2 uses command $2 with argument $0, its purpose is
unknown. Commands $0 and $7 have not been observed.

The following extended commands have been observed:

Command | Description
--------|-------------------------------------------------------------
  $0    | Copy current time to locations $00–06
  $1    | Copy locations $00–06 to current time, and update event time
  $2    | Seems to be some kind of status request
  $E    | Executing twice triggers tone generator

Commands $0 and $1 are used to read and write the current time
atomically. Writing the current time in this way also updates the event
time to maintain the remaining duration until the event occurs.

Command $2 is used on start and seems to be some kind of status
request. The games will not start if the result is not $1.

### RTC Location Map

The purpose of some locations has been inferred by observing behavior:

Location | Purpose
---------|------------------------------------------------------------
  $00–06 | Output or input space for reading or writing current time
  $10–12 | Minute of day counter (rolls over at 1440)
  $13–15 | Day counter
  $26    | Tone selection in two least significant bits
  $27    | Tone generator enabled if value is 1 (all bits checked)
  $58–5A | Event time minutes
  $5B–5D | Event time days

## External links

- Source: [GBDev Forums thread](https://gbdev.gg8.se/forums/viewtopic.php?id=744)
