Forward
-------

This chapter describes only CGB (Color Gameboy) registers that didn\'t
fit into normal categories - most CGB registers are described in the
chapter about Video Display (Color Palettes, VRAM Bank, VRAM DMA
Transfers, and changed meaning of Bit 0 of LCDC Control register). Also,
a changed bit is noted in the chapter about the Serial/Link port.

Unlocking CGB functions
-----------------------

When using any CGB registers (including those in the Video/Link
chapters), you must first unlock CGB features by changing byte 0143h in
the cartridge header. Typically use a value of 80h for games which
support both CGB and monochrome gameboys, and C0h for games which work
on CGBs only. Otherwise, the CGB will operate in monochrome \"Non CGB\"
compatibility mode.

Detecting CGB (and GBA) functions
---------------------------------

CGB hardware can be detected by examing the CPU accumulator (A-register)
directly after startup. A value of 11h indicates CGB (or GBA) hardware,
if so, CGB functions can be used (if unlocked, see above). When A=11h,
you may also examine Bit 0 of the CPUs B-Register to separate between
CGB (bit cleared) and GBA (bit set), by that detection it is possible to
use \'repaired\' color palette data matching for GBA displays.

Documented registers
--------------------

### FF4D - KEY1 - CGB Mode Only - Prepare Speed Switch

` Bit 7: Current Speed     (0=Normal, 1=Double) (Read Only)`\
` Bit 0: Prepare Speed Switch (0=No, 1=Prepare) (Read/Write)`

This register is used to prepare the gameboy to switch between CGB
Double Speed Mode and Normal Speed Mode. The actual speed switch is
performed by executing a STOP command after Bit 0 has been set. After
that Bit 0 will be cleared automatically, and the gameboy will operate
at the \'other\' speed. The recommended speed switching procedure in
pseudo code would be:

` IF KEY1_BIT7 <> DESIRED_SPEED THEN`\
`   IE=00H       ;(FFFF)=00h`\
`   JOYP=30H     ;(FF00)=30h`\
`   KEY1=01H     ;(FF4D)=01h`\
`   STOP         ;STOP`\
` ENDIF`

The CGB is operating in Normal Speed Mode when it is turned on. Note
that using the Double Speed Mode increases the power consumption, it
would be recommended to use Single Speed whenever possible. However, the
display will flicker (white) for a moment during speed switches, so this
cannot be done permanentely. In Double Speed Mode the following will
operate twice as fast as normal:

` The CPU (2.10 MHz, 1 Cycle = approx. 0.5us)`\
` Timer and Divider Registers`\
` Serial Port (Link Cable)`\
` DMA Transfer to OAM`

And the following will keep operating as usual:

` LCD Video Controller`\
` HDMA Transfer to VRAM`\
` All Sound Timings and Frequencies`

### FF56 - RP - CGB Mode Only - Infrared Communications Port

This register allows to input and output data through the CGBs built-in
Infrared Port. When reading data, bit 6 and 7 must be set (and obviously
Bit 0 must be cleared - if you don\'t want to receive your own gameboys
IR signal). After sending or receiving data you should reset the
register to 00h to reduce battery power consumption again.

` Bit 0:   Write Data   (0=LED Off, 1=LED On)             (Read/Write)`\
` Bit 1:   Read Data    (0=Receiving IR Signal, 1=Normal) (Read Only)`\
` Bit 6-7: Data Read Enable (0=Disable, 3=Enable)         (Read/Write)`

Note that the receiver will adapt itself to the normal level of IR
pollution in the air, so if you would send a LED ON signal for a longer
period, then the receiver would treat that as normal (=OFF) after a
while. For example, a Philips TV Remote Control sends a series of 32 LED
ON/OFF pulses (length 10us ON, 17.5us OFF each) instead of a permanent
880us LED ON signal. Even though being generally CGB compatible, the GBA
does not include an infra-red port.

### FF70 - SVBK - CGB Mode Only - WRAM Bank

In CGB Mode 32 KBytes internal RAM are available. This memory is divided
into 8 banks of 4 KBytes each. Bank 0 is always available in memory at
C000-CFFF, Bank 1-7 can be selected into the address space at D000-DFFF.

` Bit 0-2  Select WRAM Bank (Read/Write)`

Writing a value of 01h-07h will select Bank 1-7, writing a value of 00h
will select Bank 1 either.

Undocumented registers
----------------------

These are undocumented CGB Registers. Purpose of these registers is
unknown (if any). It isn\'t recommended to use them in your software,
but you could, for example, use them to check if you are running on an
emulator or on DMG hardware.

### FF6C - Bit 0 (Read/Write) - CGB Mode Only

Only the least significant bit of this register can be written to. It
defaults to 0, so this register\'s initial value is \$FE.

In non-CGB mode, it isn\'t writable, and its value is locked at \$FF.

### FF72 - Bits 0-7 (Read/Write)

### FF73 - Bits 0-7 (Read/Write)

Both of these registers are fully read/write. Their initial value is
\$00.

### FF74 - Bits 0-7 (Read/Write) - CGB Mode Only

In CGB mode, this register is fully readable and writable. Its initial
value is \$00.

Otherwise, this register is read-only, and locked at value \$FF.

### FF75 - Bits 4-6 (Read/Write)

Only bits 4, 5 and 6 of this register are read/write enabled. Their
initial value is 0.

### FF76 - PCM12 - PCM amplitudes 1 & 2 (Read Only)

This register is read-only. The low nibble is a copy of sound channel
\#1\'s PCM amplitude, the high nibble a copy of sound channel \#2\'s.

### FF77 - PCM34 - PCM amplitudes 3 & 4 (Read Only)

Same, but with channels 3 and 4.

