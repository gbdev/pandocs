# DMA Transfers to VRAM

The GBC added a system that can copy data to VRAM in 16-byte increments much faster than the CPU can;
such a transfer can be done either all at once ("GDMA"), or at a rate of 16 bytes per HBlank ("HDMA").

:::tip Register naming

The traditional `HDMAx` naming convention makes each register's purpose not immediately clear from its name, so some alternative `VDMA_*` names have been proposed, [such as `VDMA_LEN` for `HDMA5`](https://github.com/gbdev/hardware.inc/blob/8d4432e5796bffe2e13c438013285c5f11c37b99/hardware.inc#L919).

:::

## Source and destination

The GBC internally holds two 16-bit addresses (source, and destination); these registers allow updating half of one address.
Notably, the internal addresses are updated during transfers (though the intermediate updates can only be witnessed during a HDMA), and preserved even after a transfer completes ([test ROM][gdma_cont]).

### FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]

This should point to ROM, SRAM, or WRAM; thus either in range $0000–7FF0 or $A000–DFF0.
\[Note: this has yet to be tested on Echo RAM, OAM, FEXX, IO and HRAM\].
Trying to specify a source address in VRAM will cause garbage to be copied.

The four lower bits of this address will be ignored and treated as 0.

:::warning Banking hazard

If the source address falls into a banked region (e.g. $D000-DFFF), then data will be read from the region's active bank.
Thus, changing the active bank of a region a HDMA is reading from is perilous!

:::

### FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]

These two registers specify the address within 8000-9FF0 to which the data will be copied.
The four lower bits of this address will be ignored and treated as 0.
The upper 4 bits are ignored during a transfer, but they keep counting up, and the transfer aborts if they overflow (the status of the registers if this happens still [needs to be investigated](https://github.com/gbdev/pandocs/issues/364)).

:::warning Banking hazard

Data is written to the VRAM bank selected by [`VBK`] at the time a byte is copied.
Thus, writing to `VBK` while a HDMA is running is perilous!

:::

## FF55 — HDMA5 (CGB Mode only): VRAM DMA length/mode/start

{{#bits 8 >
  "VDMA_LEN (read)" 7:"Active" 6-0:"Remaining length";
	"VDMA_LEN (write)" 7:"Mode" 6-0:"Length"
}}

- **Active**: Indicates whether a HDMA is in progress; `0` = yes, `1` = no.
- **Mode**: Selects between GDMA (`0`) and HDMA (`1`), see below.
- **Length**: In 16-byte blocks, minus 1 (thus between `$00` = 16 bytes to `$7F` = 128 × 16 bytes).

Writing to this register immediately starts the transfer.

### Bit 7 = 0: General-purpose DMA

This mode transfers everything at once; thus, since it is subject to the same access restrictions as the CPU (particularly VRAM), it is only suitable for use while the LCD is off or during VBlank, or during HBlank / OAM scan for short enough transfers (see [timing information](<#Transfer Timings>) below).

The CPU is paused during the transfer (thus, as soon as `VDMA_LEN` is written to).
After completion, reading `VDMA_LEN` will return $FF.

### Bit 7 = 1: HBlank DMA

This mode copies each 16-byte block at the start of HBlank; thus, 16 bytes get copied on each of scanlines 0 to 143, and none during VBlank, until all specified blocks have been written and the transfer ends automatically.
The CPU is paused during each copy.

Quirks:

- Starting a HDMA during HBlank immediately performs the first copy.
- Starting a HDMA while the LCD is off immediately performs one copy, and no more until the LCD is turned back on.
- Transfers cannot occur while the CPU is [halted], and will be resumed only when the CPU resumes execution ([test rom](https://github.com/alloncm/MagenTests#vram-dma-hblank-mode)).

After the last copy, reads from `VDMA_LEN` return \$FF.
However, while the transfer is ongoing, reading from `VDMA_LEN` returns the remaining length (divided by 16, minus 1) and bit 7 reset.
Writing a value with bit 7 clear to `VDMA_LEN` will cancel the HDMA, and `VDMA_LEN` will report how many blocks remained to be copied (in the usual format) in the bottom 7 bits but bit 7 will be set.

## Transfer Timings

Each 16-byte copy takes 8 µs regardless of the [current CPU speed], so 8 M-cycles in Normal-speed mode [\[1\]](imgs/hdma_normal_speed.png) / 16 M-cycles in Double-speed mode [\[2\]](imgs/hdma_double_speed.png).
Thus, up to 2280 bytes (142.5 tiles) can be transferred during a single VBlank.

Since VDMA accesses ROM as fast as Double-speed mode, official documentation stated that MBCs other than MBC5 should not be used with VDMA; in practice, modern chips have sufficiently fast response times.

## Tricks

- More than 128 tiles can be transferred by writing to `HDMA5` again after a transfer completes; there is no need to write to the other registers again.
- It is possible to fill all of VRAM with a repeating 16-byte pattern by performing one transfer, resetting only the source address (`HDMA2` is sufficient if it wasn't $F0), and repeating.
- It is also possible to alter the source and/or destination between two HDMA writes, or even to change `VBK` to write to both VRAM banks; this however requires careful timing, which can be messed up by an unfortunately-timed [interrupt].

## Boot-up state

TODO: what state does the boot ROM leave them at?

[gdma_cont]: https://github.com/LIJI32/SameSuite/blob/f15645fb049a47ea235f6d2c9a033e72d8087901/dma/gbc_dma_cont.asm
[`VBK`]: <#FF4F — VBK (CGB Mode only): VRAM bank>
[halted]: <#Using the HALT Instruction>
[current CPU speed]: <#FF4D — KEY1/SPD (CGB Mode only): Prepare speed switch>
[interrupt]: #Interrupts
