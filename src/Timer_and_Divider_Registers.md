# Timer and Divider Registers

::: tip NOTE

The Timer described below is the built-in timer in the gameboy. It has
nothing to do with the MBC3s battery buffered Real Time Clock - that\'s
a completely different thing, described in
[Memory Bank Controllers](#MBCs).

:::

## FF04 — DIV: Divider register

This register is incremented at a rate of 16384Hz (\~16779Hz on SGB).
Writing any value to this register resets it to $00.
Additionally, this register is reset when executing the `stop` instruction, and
only begins ticking again once `stop` mode ends. This also occurs during a
[speed switch](<#FF4D — KEY1 (CGB Mode only): Prepare speed switch>).
(TODO: how is it affected by the wait after a speed switch?)

Note: The divider is affected by CGB double speed mode, and will
increment at 32768Hz in double speed.

## FF05 — TIMA: Timer counter

This timer is incremented at the clock frequency specified by the TAC
register (\$FF07). When the value overflows (exceeds $FF)
it is reset to the value specified in TMA (FF06) and [an interrupt](<#INT $50 — Timer interrupt>)
is requested, as described below.

## FF06 — TMA: Timer modulo

When TIMA overflows, it is reset to the value in this register and [an interrupt](<#INT $50 — Timer interrupt>) is requested.
Example of use: if TMA is set to $FF, an interrupt is requested at the clock frequency selected in
TAC (because every increment is an overflow). However, if TMA is set to $FE, an interrupt is
only requested every two increments, which effectively divides the selected clock by two. Setting
TMA to $FD would divide the clock by three, and so on.

If a TMA write is executed on the same cycle as the content of TMA is transferred to TIMA
due to a timer overflow, the old value is transferred to TIMA.

## FF07 — TAC: Timer control

```
Bit  2   - Timer Enable
Bits 1-0 - Input Clock Select
           00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
           01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
           10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
           11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
```

::: tip NOTE

The "Timer Enable" bit only affects the timer (TIMA). The divider (DIV) is **always** counting.

:::
