# `halt`

`halt` is an instruction that pauses the CPU (during which [less power is
consumed](<#Using the HALT Instruction>)) when executed. The CPU wakes up as soon as an interrupt is pending,
that is, when the bitwise AND of [`IE`](<#FFFF — IE: Interrupt enable>)
and [`IF`](<#FF0F — IF: Interrupt flag>) is non-zero.

Most commonly, [`IME`](<#IME: Interrupt master enable flag \[write only\]>) is
set. In this case, the CPU simply wakes up, and before executing the instruction
after the `halt`, the [interrupt handler is called](<#Interrupt Handling>)
normally.

If `IME` is *not* set, there are two distinct cases, depending on whether an
interrupt is pending as the `halt` instruction is first executed.

- If no interrupt is pending, `halt` executes as normal, and the CPU resumes
  regular execution as soon as an interrupt becomes pending. However, since
  `IME`=0, the interrupt is not handled.
- If an interrupt is pending, `halt` immediately exits, as expected, however
  the "`halt` bug", explained below, is triggered.

## `halt` bug

When a `halt` instruction is executed with `IME = 0` and `[IE] & [IF] != 0`, the `halt` instruction ends immediately, but [`pc` fails to be normally incremented](https://github.com/nitro2k01/little-things-gb/tree/main/double-halt-cancel).

Under most circumstances, this causes the byte after the `halt` to be read a second time (and this behaviour can repeat if said byte executes another `halt` instruction).
But, if the `halt` is immediately followed by a jump to elsewhere, then the behaviour will be slightly different; this is possible in only one of two ways:

- The `halt` comes immediately after a `ei` instruction (whose effect is typically delayed by one instruction, hence `IME` still being zero for the `halt`): the interrupt is serviced and the handler called, but the interrupt returns to the `halt`, which is executed again, and thus
waits for another interrupt.
([Source](https://github.com/LIJI32/SameSuite/blob/master/interrupt/ei_delay_halt.asm))
- The `halt` is immediately followed by a `rst` instruction: the `rst` instruction's return address will point at the `rst` itself, instead of the byte after it.
  Notably, a `ret` would return to the `rst` an execute it again.

If the bugged `halt` is preceded by a `ei` and followed by a `rst`, the former "wins".
