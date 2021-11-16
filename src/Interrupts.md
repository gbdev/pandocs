# Interrupts

## IME - Interrupt Master Enable Flag (Write Only)

```
0 - Disable all interrupts
1 - Enable all interrupts that are enabled in the IE register (FFFF)
```

The IME flag is used to disable all interrupts, overriding any enabled
bits in the IE register. It isn't possible to access the IME flag by
using an I/O address. IME can be modified by
the following instructions/events only:

```
EI     ; Enables interrupts  (that is, IME=1)
DI     ; Disables interrupts (that is, IME=0)
RETI   ; Enables interrupts and returns (same as the instruction sequence EI, RET)
<INT>  ; Disables interrupts and calls interrupt vector
```

where \<INT\> means the operation which is automatically executed by the
CPU when it executes an interrupt.

The effect of `ei` is delayed by one instruction. This means that `ei`
followed immediately by DI does not allow any interrupts between them.
This interacts with the [`halt` bug](<#halt bug>) in an interesting way.

## FFFF - IE - Interrupt Enable (R/W)

```
Bit 0: VBlank   Interrupt Enable  (INT $40)  (1=Enable)
Bit 1: LCD STAT Interrupt Enable  (INT $48)  (1=Enable)
Bit 2: Timer    Interrupt Enable  (INT $50)  (1=Enable)
Bit 3: Serial   Interrupt Enable  (INT $58)  (1=Enable)
Bit 4: Joypad   Interrupt Enable  (INT $60)  (1=Enable)
```

## FF0F - IF - Interrupt Flag (R/W)

```
Bit 0: VBlank   Interrupt Request (INT $40)  (1=Request)
Bit 1: LCD STAT Interrupt Request (INT $48)  (1=Request)
Bit 2: Timer    Interrupt Request (INT $50)  (1=Request)
Bit 3: Serial   Interrupt Request (INT $58)  (1=Request)
Bit 4: Joypad   Interrupt Request (INT $60)  (1=Request)
```

When an interrupt signal changes from low to high, the
corresponding bit in the IF register becomes set. For example, Bit 0
becomes set when the LCD controller enters the VBlank period.

Any set bits in the IF register are only **requesting** an interrupt to be
executed. The actual **execution** happens only if both the IME flag and
the corresponding bit in the IE register are set, otherwise the
interrupt "waits" until both IME and IE allow its execution.

Since the CPU automatically sets and clears the bits in the IF register, it
is usually not required to write to the IF register. However, the user
may still do that in order to manually request (or discard) interrupts.
Like with real interrupts, a manually requested interrupt isn't executed
unless/until IME and IE allow its execution.

## Interrupt Handling

1. The IF bit corresponding to this interrupt and the IME flag are reset by the CPU.
The former "acknowledges" the interrupt, while the latter prevents any further interrupts
from being handled until the program re-enables them, typically by using the `reti` instruction.
2. The corresponding interrupt handler (see the IE and IF register descriptions [above](<#FFFF - IE - Interrupt Enable (R/W)>)) is
called by the CPU. This is a regular call, exactly like what would be performed by a `call <vector>` instruction (the current PC is pushed on the stack
and then set to the address of the interrupt vector).

The following occurs when control is being transferred to an interrupt handler:

1. Two wait states are executed (2 M-cycles pass while nothing
occurs, presumably the CPU is executing `nop`s during this time).
2. The current PC is pushed to the stack, consuming 2 more M-cycles.
3. The PC register is set to the address of the handler ($40, $48, $50, $58, $60).
This consumes one last M-cycle.

The entire ISR **should** last a total of 5 M-cycles.
This has yet to be tested, but is what the Z80 datasheet implies.

## Interrupt Priorities

In the following three situations it might happen that more than one bit in the IF register is set, requesting more than one interrupt at once:

1. More than one interrupt signal changed from Low to High at the same time.
2. Several interrupts have been requested while IME/IE didn't allow them to be handled directly.
3. The user has written a value with several bits set (for example binary 00011111) to the IF register.

If IME and IE allow the execution of more than one of the
requested interrupts, the interrupt with the highest priority
is executed first. The priorities follow the same order as the bits in the IE
and IF registers: Bit 0 (VBlank) has the highest priority, and Bit 4
(Joypad) has the lowest priority.

## Nested Interrupts

The CPU automatically disables all the other interrupts by setting IME=0
when it executes an interrupt. Usually IME remains zero until the
interrupt handler returns (and sets IME=1 by means of the RETI instruction).
However, if you want any other interrupts (of any priority)
to be allowed to be executed from inside the interrupt
handler, then you can use the EI instruction in the interrupt
handler.
