# LCD Status Registers

::: tip TERMINOLOGY

A *dot* is the shortest period over which the PPU can output one pixel: is it equivalent to 1 T-state on DMG or on CGB single-speed mode or 2 T-states on CGB double-speed mode. On each dot during mode 3, either the PPU outputs a pixel or the fetcher is stalling the [FIFOs](<#Pixel FIFO>).

:::

## FF44 — LY: LCD Y coordinate \[read-only\]

LY indicates the current horizontal line, which might be about to be drawn,
being drawn, or just been drawn. LY can hold any value from 0 to 153, with
values from 144 to 153 indicating the VBlank period.

## FF45 — LYC: LY compare

The Game Boy constantly compares the value of the LYC and LY registers.
When both values are identical, the "LYC=LY" flag in the STAT register
is set, and (if enabled) a STAT interrupt is requested.

## FF41 — STAT: LCD status

```
Bit 6 - LYC=LY STAT Interrupt source         (1=Enable) (Read/Write)
Bit 5 - Mode 2 OAM STAT Interrupt source     (1=Enable) (Read/Write)
Bit 4 - Mode 1 VBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 3 - Mode 0 HBlank STAT Interrupt source  (1=Enable) (Read/Write)
Bit 2 - LYC=LY Flag                          (0=Different, 1=Equal) (Read Only)
Bit 1-0 - Mode Flag                          (Mode 0-3, see below) (Read Only)
          0: HBlank
          1: VBlank
          2: Searching OAM
          3: Transferring Data to LCD Controller
```

The two lower STAT bits show the current [status of the PPU](<#PPU modes>).

Bit 2 is set when [LY](<#FF44 — LY: LCD Y coordinate \[read-only\]>) contains the same value as [LYC](<#FF45 — LYC: LY compare>).
It is constantly updated.

Bits 3-6 select which sources are used for [the STAT interrupt](<#INT $48 — STAT interrupt>).

### Spurious STAT interrupts

A hardware quirk in the monochrome Game Boy makes the LCD interrupt
sometimes trigger when writing to STAT (including writing \$00) during
OAM scan, HBlank, VBlank, or LY=LYC. It behaves as if \$FF were
written for one cycle, and then the written value were written the next
cycle. Because the GBC in DMG mode does not have this quirk, two games
that depend on this quirk (Ocean's *Road Rash* and Vic Tokai's *Xerd
no Densetsu*) will not run on a GBC.
