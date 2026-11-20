# DMA Transfers to VRAM

:::tip

One of the pitfalls of the `HDMAx` naming convention is that the register's purpose is not immediately clear from its name, so some alternative `VDMA_*` names have been proposed, [such as `VDMA_LEN` for `HDMA5`](https://github.com/gbdev/hardware.inc/blob/8d4432e5796bffe2e13c438013285c5f11c37b99/hardware.inc#L919).

:::

## FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]

These two registers specify the address at which the transfer will read
data from. Normally, this should be either in ROM, SRAM or WRAM, thus
either in range 0000-7FF0 or A000-DFF0. \[Note: this has yet to be
tested on Echo RAM, OAM, FEXX, IO and HRAM\]. Trying to specify a source
address in VRAM will cause garbage to be copied.

The four lower bits of this address will be ignored and treated as 0.
After a transfer the address contained in this register pair is 
by $10 for each block of $10 bytes transfered, these 
registers being write-only, this can only be observed by doing another 
transfer without updating these registers.

## FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]

These two registers specify the address within 8000-9FF0 to which the
data will be copied. Only bits 12-4 are respected; others are ignored.
The four lower bits of this address will be ignored and treated as 0.

## State of the VRAM DMA source/destination registers after a transfer

After a transfer, the source/destination registers are incremented by $10 
for each block of $10 bytes transfered.
Despite both the VRAM DMA source/destination registers being write-only,
knowing their state after a transfer can turn useful when performing
multiple transfer in a row.
For instance, a transfer of one large block is mostly equivalent to
multiple transfers of smaller blocks, without needing to update the source
nor the destination registers between each of the smaller transfers.
Another use case would be to fill VRAM with the same $10 bytes block repeated 
all over, as only the source address register would need be updated after each
transfer, the destination register being automatically incremented by
the block size after each transfer.

## FF55 — HDMA5 (CGB Mode only): VRAM DMA length/mode/start

These registers are used to initiate a DMA transfer from ROM or RAM to
VRAM. The Source Start Address may be located at 0000-7FF0 or A000-DFF0,
the lower four bits of the address are ignored (treated as zero). The
Destination Start Address may be located at 8000-9FF0, the lower four
bits of the address are ignored (treated as zero), the upper 3 bits are
ignored either (destination is always in VRAM).

Writing to this register starts the transfer, the lower 7 bits of which
specify the Transfer Length (divided by $10, minus 1), that is, lengths of
$10-$800 bytes can be defined by the values $00-$7F. The upper bit
indicates the Transfer Mode:

### Bit 7 = 0 — General-Purpose DMA

When using this transfer method,
all data is transferred at once. The execution of the program is halted
until the transfer has completed. Note that the General Purpose DMA
blindly attempts to copy the data, even if the LCD controller is
currently accessing VRAM. So General Purpose DMA should be used only if
the Display is disabled, or during VBlank, or (for rather short blocks)
during HBlank. The execution of the program continues when the transfer
has been completed, and FF55 then contains a value of $FF.

### Bit 7 = 1 — HBlank DMA

The HBlank DMA transfers $10 bytes of
data during each HBlank, that is, at LY=0-143, no data is transferred during
VBlank (LY=144-153), but the transfer will then continue at LY=00. The
execution of the program is halted during the separate transfers, but
the program execution continues during the "spaces" between each data
block. Note that the program should not change the Destination VRAM bank
(FF4F), or the Source ROM/RAM bank (in case data is transferred from
bankable memory) until the transfer has completed! (The transfer should
be paused as described below while the banks are switched).

Upon halting the CPU (using the [halt instruction](<#Using the HALT Instruction>)),
the transfer will also be halted and will be resumed only when the CPU resumes execution ([test rom](https://github.com/alloncm/MagenTests?tab=readme-ov-file#vram-dma-hblank-mode) exhibiting this behaviour).

Reading from Register FF55 returns the remaining length (divided by $10,
minus 1), a value of $FF indicates that the transfer has completed. It
is also possible to terminate an active HBlank transfer by writing zero
to Bit 7 of FF55. In that case reading from FF55 will return how many
\$10 "blocks" remained (minus 1) in the lower 7 bits, but Bit 7 will
be read as "1". Stopping the transfer doesn't set HDMA1-4 to \$FF.

:::warning WARNING

HBlank DMA should not be started (write to FF55) during a HBlank
period (STAT mode 0).

If the transfer's destination address overflows, the transfer stops
prematurely.
The status of the registers if this happens still needs to be [investigated](https://github.com/gbdev/pandocs/issues/364).

:::

## Confirming if the DMA Transfer is Active

Reading Bit 7 of FF55 can be used to confirm if the DMA transfer is
active (1=Not Active, 0=Active). This works under any circumstances -
after completion of General Purpose, or HBlank Transfer, and after
manually terminating a HBlank Transfer.

## Transfer Timings

In both Normal Speed and Double Speed Mode it takes about 8 μs to
transfer a block of $10 bytes.
That is, 8 M-cycles in Normal Speed Mode [\[1\]](imgs/hdma_normal_speed.png),
and 16 "fast" M-cycles in Double Speed Mode [\[2\]](imgs/hdma_double_speed.png).
Older MBC controllers (like MBC1-3) and slower ROMs are not guaranteed to support General
Purpose or HBlank DMA, that's because there are always 2 bytes
transferred per microsecond (even if the itself program runs it Normal
Speed Mode).

This allows for a transfer of 2280 bytes during VBlank, which is up to 142.5 tiles.
