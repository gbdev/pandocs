This chapter describes only Game Boy Color (GBC or CGB) registers that didn't
fit into normal categories - most CGB registers are described in the
chapter about Video Display (Color Palettes, VRAM Bank, VRAM DMA
Transfers, and changed meaning of Bit 0 of LCDC Control register). Also,
a changed bit is noted in the chapter about the Serial/Link port.

# Unlocking CGB functions

When using any CGB registers (including those in the Video/Link
chapters), you must first unlock CGB features by changing byte 0143h in
the cartridge header. Typically use a value of 80h for games which
support both CGB and monochrome Game Boy systems, and C0h for games which work
on CGBs only. Otherwise, the CGB will operate in monochrome "Non CGB"
compatibility mode.

# Detecting CGB (and GBA) functions

CGB hardware can be detected by examing the CPU accumulator (A-register)
directly after startup. A value of 11h indicates CGB (or GBA) hardware,
if so, CGB functions can be used (if unlocked, see above). When A=11h,
you may also examine Bit 0 of the CPUs B-Register to separate between
CGB (bit cleared) and GBA (bit set), by that detection it is possible to
use "repaired" color palette data matching for GBA displays.

# Documented registers

### FF4D - KEY1 - CGB Mode Only - Prepare Speed Switch

```
 Bit 7: Current Speed     (0=Normal, 1=Double) (Read Only)
 Bit 0: Prepare Speed Switch (0=No, 1=Prepare) (Read/Write)
```

This register is used to prepare the Game Boy to switch between CGB
Double Speed Mode and Normal Speed Mode. The actual speed switch is
performed by executing a `stop` instruction after Bit 0 has been set. After
that, Bit 0 will be cleared automatically, and the Game Boy will operate
at the "other" speed. The recommended speed switching procedure in
pseudo code would be:

```
IF KEY1_BIT7 != DESIRED_SPEED THEN
   IE = $00       ; (FFFF) = $00
   JOYP = $30     ; (FF00) = $30
   KEY1 = $01     ; (FF4D) = $01
   STOP
ENDIF
```

The CGB is operating in Normal Speed Mode when it is first turned on. Note
that using the Double Speed Mode increases the power consumption; therefore, it
would be recommended to use Single Speed whenever possible.

In Double Speed Mode the following will operate twice as fast as normal:

- The CPU (2.10 MHz, so 1 cycle = approx. 0.5 µs)
- Timer and Divider Registers
- Serial Port (Link Cable)
- DMA Transfer to OAM

And the following will keep operating as usual:

- LCD Video Controller
- HDMA Transfer to VRAM
- All Sound Timings and Frequencies

The CPU stops for 2050 cycles (= 8200 clocks) after the `stop` instruction is
executed. During this time, the CPU is in a strange state. `DIV` does not tick, so
*some* audio events are not processed. Additionally, VRAM/OAM/... locking is "frozen", yielding
different results depending on the [STAT mode](#lcd-status-register) it's started in:

- HBlank / VBlank (Mode 0 / Mode 1): The PPU cannot access any videomemory, and produces black pixels
- OAM scan (Mode 2): The PPU can access VRAM just fine, but not OAM, leading to rendering background, but not sprites
- Rendering (Mode 3): The PPU can access everything correctly, and so rendering is not affected

TODO: confirm whether interrupts can occur (just the joypad one?) during the pause, and consequences if so

### FF56 - RP - CGB Mode Only - Infrared Communications Port

This register allows to input and output data through the CGBs built-in
Infrared Port. When reading data, bit 6 and 7 must be set (and obviously
Bit 0 must be cleared - if you don't want to receive your own Game Boy's
IR signal). After sending or receiving data you should reset the
register to 00h to reduce battery power consumption again.

```
 Bit 0:   Write Data   (0=LED Off, 1=LED On)             (Read/Write)
 Bit 1:   Read Data    (0=Receiving IR Signal, 1=Normal) (Read Only)
 Bit 6-7: Data Read Enable (0=Disable, 3=Enable)         (Read/Write)
```

Note that the receiver will adapt itself to the normal level of IR
pollution in the air, so if you would send a LED ON signal for a longer
period, then the receiver would treat that as normal (=OFF) after a
while. For example, a Philips TV Remote Control sends a series of 32 LED
ON/OFF pulses (length 10us ON, 17.5us OFF each) instead of a permanent
880us LED ON signal. Even though being generally CGB compatible, the GBA
does not include an infra-red port.

### FF6C - OPRI - CGB Mode Only - Object Priority Mode

This register serves as a flag for which object priority mode to use. While
the DMG prioritizes objects by x-coordinate, the CGB prioritizes them by
location in OAM. This flag is set by the CGB bios after checking the game's CGB compatibility.

OPRI has an effect if a PGB value (`0xX8`, `0xXC`) is written to KEY0 but STOP hasn't been executed yet, and the write takes effect instantly.

::: warning TO BE VERIFIED
It does not have an effect, at least not an instant effect, if written to during CGB or DMG mode after the boot ROM has been unmapped.
It is not known if triggering a PSM NMI, which remaps the boot ROM, has an effect on this register's behavior.
:::

```
Bit 0: OBJ Priority Mode (0=OAM Priority, 1=Coordinate Priority) (Read/Write)
```

### FF70 - SVBK - CGB Mode Only - WRAM Bank

In CGB Mode 32 KBytes internal RAM are available. This memory is divided
into 8 banks of 4 KBytes each. Bank 0 is always available in memory at
C000-CFFF, Bank 1-7 can be selected into the address space at D000-DFFF.

```
 Bit 0-2  Select WRAM Bank (Read/Write)
```

Writing a value of 01h-07h will select Bank 1-7, writing a value of 00h
will select Bank 1 too.

# Undocumented registers

These are undocumented CGB Registers. The purpose of these registers is
unknown (if any). It isn't recommended to use them in your software,
but you could, for example, use them to check if you are running on an
emulator or on DMG hardware.

### FF72 - Bits 0-7 (Read/Write), FF73 - Bits 0-7 (Read/Write)

Both of these registers are fully read/write. Their initial value is
$00.

### FF74 - Bits 0-7 (Read/Write) - CGB Mode Only

In CGB mode, this register is fully readable and writable. Its initial
value is $00.

Otherwise, this register is read-only, and locked at value $FF.

### FF75 - Bits 4-6 (Read/Write)

Only bits 4, 5 and 6 of this register are read/write enabled. Their
initial value is 0.

### FF76 - PCM12 - PCM amplitudes 1 & 2 (Read Only)

This register is read-only. The low nibble is a copy of sound channel
\#1's PCM amplitude, the high nibble a copy of sound channel \#2's.

### FF77 - PCM34 - PCM amplitudes 3 & 4 (Read Only)

Same, but with channels 3 and 4.

