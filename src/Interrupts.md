# Interrupts

## IME: Interrupt master enable flag \[write only\]

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
<ISR>  ; Disables interrupts and calls the interrupt handler
```

Where \<ISR\> is the Interrupt Service Routine that is automatically executed
by the CPU when it services an interrupt request.

The effect of `ei` is delayed by one instruction. This means that `ei`
followed immediately by `di` does not allow any interrupts between them.
This interacts with the [`halt` bug](<#halt bug>) in an interesting way.

## FFFF — IE: Interrupt enable

```
Bit 0: VBlank   Interrupt Enable  (INT $40)  (1=Enable)
Bit 1: LCD STAT Interrupt Enable  (INT $48)  (1=Enable)
Bit 2: Timer    Interrupt Enable  (INT $50)  (1=Enable)
Bit 3: Serial   Interrupt Enable  (INT $58)  (1=Enable)
Bit 4: Joypad   Interrupt Enable  (INT $60)  (1=Enable)
```

## FF0F — IF: Interrupt flag

```
Bit 0: VBlank   Interrupt Request (INT $40)  (1=Request)
Bit 1: LCD STAT Interrupt Request (INT $48)  (1=Request)
Bit 2: Timer    Interrupt Request (INT $50)  (1=Request)
Bit 3: Serial   Interrupt Request (INT $58)  (1=Request)
Bit 4: Joypad   Interrupt Request (INT $60)  (1=Request)
```

When an interrupt request signal changes from low to high, the
corresponding bit in the IF register is set. For example, Bit 0
becomes set when the LCD controller enters the VBlank period.

Any set bits in the IF register are only **requesting** an interrupt.
The actual **execution** of the interrupt handler happens only if both the IME flag and
the corresponding bit in the IE register are set; otherwise the
interrupt "waits" until both IME and IE allow it to be serviced.

Since the CPU automatically sets and clears the bits in the IF register, it
is usually not necessary to write to the IF register. However, the user
may still do that in order to manually request (or discard) interrupts.
Just like real interrupts, a manually requested interrupt isn't serviced
unless/until IME and IE allow it.

## Interrupt Handling

1. The IF bit corresponding to this interrupt and the IME flag are reset by the CPU.
The former "acknowledges" the interrupt, while the latter prevents any further interrupts
from being handled until the program re-enables them, typically by using the `reti` instruction.
2. The corresponding interrupt handler (see the IE and IF register descriptions [above](<#FFFF — IE: Interrupt enable>)) is
called by the CPU. This is a regular call, exactly like what would be performed by a `call <address>` instruction (the current PC is pushed onto the stack
and then set to the address of the interrupt handler).

The following interrupt service routine is executed when control is being transferred to an interrupt handler:

1. Two wait states are executed (2 M-cycles pass while nothing happens; presumably the CPU is executing `nop`s during this time).
2. The current value of the PC register is pushed onto the stack, consuming 2 more M-cycles.
3. The PC register is set to the address of the handler (one of: $40, $48, $50, $58, $60).
This consumes one last M-cycle.

The entire routine **should** last a total of 5 M-cycles.
This has yet to be tested, but is what the Z80 datasheet implies.

## Interrupt Priorities

In the following circumstances it is possible that more than one bit in the IF register is set, requesting more than one interrupt at once:

1. More than one interrupt request signal changed from low to high at the same time.
2. Several interrupts have been requested while IME/IE didn't allow them to be serviced.
3. The user has written a value with several bits set (for example binary 00011111) to the IF register.

If IME and IE allow the servicing of more than one of the
requested interrupts, the interrupt with the highest priority
is serviced first. The priorities follow the order of the bits in the IE
and IF registers: Bit 0 (VBlank) has the highest priority, and Bit 4
(Joypad) has the lowest priority.

## Nested Interrupts

The CPU automatically disables all the other interrupts by setting IME=0
when it services an interrupt. Usually IME remains zero until the
interrupt handler returns (and sets IME=1 by means of the `reti` instruction).
However, if you want to allow the servicing of other interrupts (of any priority)
during the execution of an interrupt handler, you may do so by using the
`ei` instruction in the handler.
