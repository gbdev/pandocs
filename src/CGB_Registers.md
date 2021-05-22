# CGB Registers

This chapter describes only Game Boy Color (GBC or CGB) registers that didn't
fit into normal categories - most CGB registers are described in the
chapter about Video Display (Color Palettes, VRAM Bank, VRAM DMA
Transfers, and changed meaning of Bit 0 of LCDC Control register). Also,
a changed bit is noted in the chapter about the Serial/Link port.

## Unlocking CGB functions

When using any CGB registers (including those in the Video/Link
chapters), you must first unlock CGB features by changing byte 0143h in
the cartridge header. Typically use a value of 80h for games which
support both CGB and monochrome Game Boy systems, and C0h for games which work
on CGBs only. Otherwise, the CGB will operate in monochrome "Non CGB"
compatibility mode.

## Detecting CGB (and GBA) functions

CGB hardware can be detected by examing the CPU accumulator (A-register)
directly after startup. A value of 11h indicates CGB (or GBA) hardware,
if so, CGB functions can be used (if unlocked, see above). When A=11h,
you may also examine Bit 0 of the CPUs B-Register to separate between
CGB (bit cleared) and GBA (bit set), by that detection it is possible to
use "repaired" color palette data matching for GBA displays.

## Documented registers

### LCD VRAM DMA Transfers

#### FF51 - HDMA1 (New DMA Source, High) (W), FF52 - HDMA2 (New DMA Source, Low) (W) - CGB Mode Only

These two registers specify the address at which the transfer will read
data from. Normally, this should be either in ROM, SRAM or WRAM, thus
either in range 0000-7FF0 or A000-DFF0. \[Note: this has yet to be
tested on Echo RAM, OAM, FEXX, IO and HRAM\]. Trying to specify a source
address in VRAM will cause garbage to be copied.

The four lower bits of this address will be ignored and treated as 0.

#### FF53 - HDMA3 (New DMA Destination, High) (W), FF54 - HDMA4 (New DMA Destination, Low) (W) - CGB Mode Only

These two registers specify the address within 8000-9FF0 to which the
data will be copied. Only bits 12-4 are respected; others are ignored.
The four lower bits of this address will be ignored and treated as 0.

#### FF55 - HDMA5 (New DMA Length/Mode/Start) (W) - CGB Mode Only

These registers are used to initiate a DMA transfer from ROM or RAM to
VRAM. The Source Start Address may be located at 0000-7FF0 or A000-DFF0,
the lower four bits of the address are ignored (treated as zero). The
Destination Start Address may be located at 8000-9FF0, the lower four
bits of the address are ignored (treated as zero), the upper 3 bits are
ignored either (destination is always in VRAM).

Writing to this register starts the transfer, the lower 7 bits of which
specify the Transfer Length (divided by 10h, minus 1), that is, lengths of
10h-800h bytes can be defined by the values 00h-7Fh. The upper bit
indicates the Transfer Mode:

##### Bit 7 = 0 - General Purpose DMA

When using this transfer method,
all data is transferred at once. The execution of the program is halted
until the transfer has completed. Note that the General Purpose DMA
blindly attempts to copy the data, even if the LCD controller is
currently accessing VRAM. So General Purpose DMA should be used only if
the Display is disabled, or during VBlank, or (for rather short blocks)
during HBlank. The execution of the program continues when the transfer
has been completed, and FF55 then contains a value of FFh.

##### Bit 7 = 1 - HBlank DMA

The HBlank DMA transfers 10h bytes of
data during each HBlank, that is, at LY=0-143, no data is transferred during
VBlank (LY=144-153), but the transfer will then continue at LY=00. The
execution of the program is halted during the separate transfers, but
the program execution continues during the "spaces" between each data
block. Note that the program should not change the Destination VRAM bank
(FF4F), or the Source ROM/RAM bank (in case data is transferred from
bankable memory) until the transfer has completed! (The transfer should
be paused as described below while the banks are switched)

Reading from Register FF55 returns the remaining length (divided by 10h,
minus 1), a value of 0FFh indicates that the transfer has completed. It
is also possible to terminate an active HBlank transfer by writing zero
to Bit 7 of FF55. In that case reading from FF55 will return how many
\$10 "blocks" remained (minus 1) in the lower 7 bits, but Bit 7 will
be read as "1". Stopping the transfer doesn't set HDMA1-4 to \$FF.

::: warning

HBlank DMA should not be started (write to FF55) during a HBlank
period (STAT mode 0).

If the transfer's destination address overflows, the transfer stops
prematurely. \[Note: what's the state of the registers if this happens
?\]

:::

#### Confirming if the DMA Transfer is Active

Reading Bit 7 of FF55 can be used to confirm if the DMA transfer is
active (1=Not Active, 0=Active). This works under any circumstances -
after completion of General Purpose, or HBlank Transfer, and after
manually terminating a HBlank Transfer.

#### Transfer Timings

In both Normal Speed and Double Speed Mode it takes about 8 μs to
transfer a block of $10 bytes.
That is, 8 M-cycles in Normal Speed Mode [\[1\]](imgs/hdma_single_speed.png),
and 16 "fast" M-cycles in Double Speed Mode [\[2\]](imgs/hdma_double_speed.png).
Older MBC controllers (like MBC1-3) and slower ROMs are not guaranteed to support General
Purpose or HBlank DMA, that's because there are always 2 bytes
transferred per microsecond (even if the itself program runs it Normal
Speed Mode).

### VRAM Banks

The CGB has twice the VRAM of the DMG, but it is banked and either bank
has a different purpose.

#### FF4F - VBK - CGB Mode Only - VRAM Bank (R/W)

This register can be written to to change VRAM banks. Only bit 0
matters, all other bits are ignored.

#### VRAM bank 1

VRAM bank 1 is split like VRAM bank 0 ; 8000-97FF also stores tiles
(just like in bank 0), which can be accessed the same way as (and at the
same time as) bank 0 tiles. 9800-9FFF contains the attributes for the
corresponding Tile Maps.

Reading from this register will return the number of the currently
loaded VRAM bank in bit 0, and all other bits will be set to 1.

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
different results depending on the [STAT mode](<#FF41 - STAT (LCD Status) (R/W)>) it's started in:

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

## Undocumented registers

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

