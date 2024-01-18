# Timer obscure behaviour

:::tip System counter

DIV is just the visible part of the **system counter**.

The **system counter** is constantly incrementing every M-cycle, unless the CPU is in [STOP mode](<#Using the STOP Instruction>).

:::

## Timer Global Circuit

{{#include imgs/src/timer_simplified.svg:2:}}

## Relation between Timer and Divider register

This is a schematic of the circuit involving TAC and DIV:

<figure><figcaption>

On **DMG**:

</figcaption>
{{#include imgs/src/timer_tac_bug_dmg.svg:2:}}
</figure>

<figure><figcaption>

On **CGB**:

</figcaption>
{{#include imgs/src/timer_tac_bug_gbc.svg:2:}}
</figure>

Notice how the values that are connected to the inputs of the
multiplexer are the values of those bits, not the carry of those bits.
This is the reason of a few things:

- When writing to DIV, the system counter is reset to zero, so the timer is
also affected.

- When writing to DIV, if the current output is 1 and timer is
enabled, as the new value after reseting the system counter will be 0, the falling
edge detector will detect a falling edge and TIMA will increase.

- When writing to TAC, if the previously selected multiplexer input was
1 and the new input is 0, TIMA will increase too. This doesn't
happen when the timer is disabled, but it also happens when disabling
the timer (the same effect as writing to DIV). The following code explains the behavior in DMG and MGB.

```
    clocks_array[4] = {256, 4, 16, 64}

    old_clocks = clocks_array[old_TAC&3]
    new_clocks = clocks_array[new_TAC&3]

    old_enable = old_TAC & BIT(2)
    new_enable = new_TAC & BIT(2)

    sys_clocks = system counter

    IF old_enable == 0 THEN
        glitch = 0 (*)
    ELSE
        IF new_enable == 0 THEN
            glitch = (sys_clocks & (old_clocks/2)) != 0
        ELSE
            glitch = ((sys_clocks & (old_clocks/2)) != 0) && ((sys_clocks & (new_clocks/2)) == 0)
        END IF
    END IF
```

The sentence marked with a (\*) has a different behaviour in GBC (AGB
and AGS seem to have strange behaviour even in the other statements).
When enabling the timer and maintaining the same frequency it doesn't
glitch. When disabling the timer it doesn't glitch either. When another
change of value happens (so timer is enabled after the write), the
behaviour depends on a race condition, so it cannot be predicted for
every device.

## Timer overflow behavior

When `TIMA` overflows, the value from `TMA` is copied, and the timer flag is set in [`IF`](<#FF0F — IF: Interrupt flag>), but **one M-cycle later**.
This means that `TIMA` is equal to \$00 for the M-cycle after it overflows.

This only happens when `TIMA` overflows from incrementing, it cannot be made to happen by manually writing to `TIMA`.

Here is an example; `SYS` represents the lower 8 bits of the system counter, and `TAC` is \$FD (timer enabled, bit 1 of `SYS` selected as source):

<figure><figcaption>

`TIMA` overflows on cycle <var>A</var>, but the interrupt is only requested on cycle <var>B</var>:

</figcaption>

M-cycle |    |    ||<var>A</var>|<var>B</var>||&#8203;
--------|----|----|----|--------|----|----|---
`SYS`   | 2B | 2C | 2D |   2E   | 2F | 30 | 31
`TIMA`  | FE | FF | FF | **00** | 23 | 24 | 24
`TMA`   | 23 | 23 | 23 |   23   | 23 | 23 | 23
`IF`    | E0 | E0 | E0 | **E0** | E4 | E4 | E4

</figure>

Here are some unexpected behaviors:

1. Writing to `TIMA` during cycle <var>A</var> acts as if the overflow **didn't happen**!
   `TMA` will not be copied to `TIMA` (the value written will therefore stay), and bit 2 of `IF` will not be set.
   Writing to `DIV`, `TAC`, or other registers won't prevent the `IF` flag from being set or `TIMA` from being reloaded.
2. Writing to `TIMA` during cycle <var>B</var> will be ignored; `TIMA` will be equal to `TIMA` at the end of the cycle anyway.
3. Writing to `TMA` during cycle <var>B</var> will have the same value copied to `TIMA` as well, on the same cycle.

Here is how `TIMA` and `TMA` interact:

{{#include imgs/src/timer_tima_tma_detailed.svg:2:}}

<details><summary>Explanation of the above behaviors:</summary>

1. Writing to `TIMA` blocks the falling edge from the increment from being detected (see the `AND` gate)[^write_edge].
2. The "Load" signal stays enabled for the entirety of cycle <var>B</var>, and since `TIMA` is made of <abbr title="T-flip-flop with Asynchronous Load">TAL</abbr> cells, it's constantly copying its input.
   However, the "Write to TIMA" signal gets reset in the middle of the cycle, thus the multiplexer emits `TMA`'s value again; in essence, the CPU's write to `TIMA` *does* go through, but it's overwritten right after.
3. As mentioned in the above bullet point, `TIMA` constantly copies its input, so it updates together with `TMA`.
   This and the previous bullet point can be emulated as if `TMA` was copied to `TIMA` at the very end of the cycle, though this is not quite what's happening in hardware.

[^write_edge]: This is necessary, because otherwise writing a number with bit 7 reset (either from the CPU or from `TMA`) when `TIMA`'s bit 7 is set, would trigger the bit 7 falling edge detector and thus schedule a spurious interrupt.

</details>
