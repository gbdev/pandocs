::: tip NOTE
The Timer described below is the built-in timer in the gameboy. It has
nothing to do with the MBC3s battery buffered Real Time Clock - that\'s
a completely different thing, described in
[Memory Bank Controllers](#memory-bank-controllers).
:::

### FF04 - DIV - Divider Register (R/W)

This register is incremented at rate of 16384Hz (\~16779Hz on SGB).
Writing any value to this register resets it to $00.
Additionally, this register is reset when executing the `stop` instruction, and
only begins ticking again once `stop` mode is exited. This also occurs during a
[speed switch](#ff4d-key1-cgb-mode-only-prepare-speed-switch).
(TODO: how is it affected by the wait after a speed switch?)

Note: The divider is affected by CGB double speed mode, and will
increment at 32768Hz in double speed.

### FF05 - TIMA - Timer counter (R/W)

This timer is incremented at a clock frequency specified by the TAC
register (\$FF07). When the value overflows (gets bigger than FFh) then
it will be reset to the value specified in TMA (FF06), and an interrupt
will be requested, as described below.

### FF06 - TMA - Timer Modulo (R/W)

When the TIMA overflows, this data will be loaded.

### FF07 - TAC - Timer Control (R/W)

```
Bit  2   - Timer Enable
Bits 1-0 - Input Clock Select
           00: CPU Clock / 1024 (DMG, SGB2, CGB Single Speed Mode:   4096 Hz, SGB1:   ~4194 Hz, CGB Double Speed Mode:   8192 Hz)
           01: CPU Clock / 16   (DMG, SGB2, CGB Single Speed Mode: 262144 Hz, SGB1: ~268400 Hz, CGB Double Speed Mode: 524288 Hz)
           10: CPU Clock / 64   (DMG, SGB2, CGB Single Speed Mode:  65536 Hz, SGB1:  ~67110 Hz, CGB Double Speed Mode: 131072 Hz)
           11: CPU Clock / 256  (DMG, SGB2, CGB Single Speed Mode:  16384 Hz, SGB1:  ~16780 Hz, CGB Double Speed Mode:  32768 Hz)
```

::: tip NOTE
The "Timer Enable" bit only affects the timer, the divider is **always** counting
:::

### INT 50 - Timer Interrupt

Each time when the timer overflows (that is, when TIMA gets bigger than FFh),
then an interrupt is requested by setting Bit 2 in the IF Register
(FF0F). When that interrupt is enabled, then the CPU will execute it by
calling the timer interrupt vector at 0050h.
