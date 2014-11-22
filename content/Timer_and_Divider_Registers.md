### FF04 - DIV - Divider Register (R/W)

This register is incremented at rate of 16384Hz (\~16779Hz on SGB).
Writing any value to this register resets it to 00h.

Note: The divider is affected by CGB double speed mode, and will
increment at 32768Hz in double speed.

### FF05 - TIMA - Timer counter (R/W)

This timer is incremented by a clock frequency specified by the TAC
register (\$FF07). When the value overflows (gets bigger than FFh) then
it will be reset to the value specified in TMA (FF06), and an interrupt
will be requested, as described below.

### FF06 - TMA - Timer Modulo (R/W)

When the TIMA overflows, this data will be loaded.

### FF07 - TAC - Timer Control (R/W)

` Bit  2   - Timer Enable`\
` Bits 1-0 - Input Clock Select`\
`            00: CPU Clock / 1024 (DMG, CGB:   4096 Hz, SGB:   ~4194 Hz)`\
`            01: CPU Clock / 16   (DMG, CGB: 262144 Hz, SGB: ~268400 Hz)`\
`            10: CPU Clock / 64   (DMG, CGB:  65536 Hz, SGB:  ~67110 Hz)`\
`            11: CPU Clock / 256  (DMG, CGB:  16384 Hz, SGB:  ~16780 Hz)`\
` `\
` Note: The "Timer Enable" bit only affects the timer, the divider is `**`ALWAYS`**` counting.`

### INT 50 - Timer Interrupt

Each time when the timer overflows (ie. when TIMA gets bigger than FFh),
then an interrupt is requested by setting Bit 2 in the IF Register
(FF0F). When that interrupt is enabled, then the CPU will execute it by
calling the timer interrupt vector at 0050h.

### Timer Obscure Behaviour

Read this page for a more detailed description of what the registers do:
[Timer Obscure Behaviour](Timer_Obscure_Behaviour "wikilink")

Note
----

The above described Timer is the built-in timer in the gameboy. It has
nothing to do with the MBC3s battery buffered Real Time Clock - that\'s
a completely different thing, described in the chapter about Memory
Banking Controllers.

