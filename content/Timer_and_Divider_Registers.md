::: tip NOTE
The Timer described below is the built-in timer in the gameboy. It has
nothing to do with the MBC3s battery buffered Real Time Clock - that\'s
a completely different thing, described in
[Memory Bank Controllers](#memory-bank-controllers).
:::

### FF04 - DIV - Divider Register (R/W)

This register is incremented at a rate of 16384Hz (\~16779Hz on SGB).
Writing any value to this register resets it to $00.
Additionally, this register is reset when executing the `stop` instruction, and
only begins ticking again once `stop` mode ends. This also occurs during a
[speed switch](#ff4d-key1-cgb-mode-only-prepare-speed-switch).
(TODO: how is it affected by the wait after a speed switch?)

Note: The divider is affected by CGB double speed mode, and will
increment at 32768Hz in double speed.

### FF05 - TIMA - Timer counter (R/W)

This timer is incremented at the clock frequency specified by the TAC
register (\$FF07). When the value overflows (exceeds $FF)
it is reset to the value specified in TMA (FF06) and an interrupt
is requested, as described below.

### FF06 - TMA - Timer Modulo (R/W)

When TIMA overflows, it is reset to the value in this register.

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
The "Timer Enable" bit only affects the timer (TIMA). The divider (DIV) is **always** counting
:::

### INT 50 - Timer Interrupt

Every time that the timer overflows (that is, when TIMA exceeds $FF),
an interrupt is requested by setting bit 2 in the IF register
($FF0F). As soon as that interrupt is enabled, the CPU will execute it by
calling the timer interrupt vector at $0050.
