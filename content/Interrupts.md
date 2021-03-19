### IME - Interrupt Master Enable Flag (Write Only)

```
0 - Disable all interrupts
1 - Enable all interrupts that are enabled in the IE register (FFFF)
```

The IME flag is used to disable all interrupts, overriding any enabled
bits in the IE register. It isn't possible to access the IME flag by
using a I/O address. IME can be modified by
the following instructions/events only:

```
EI     ;Enables interrupts  (that is, IME=1)
DI     ;Disables interrupts (that is, IME=0)
RETI   ;Enables interrupts and returns (same as the instruction sequence EI, RET)
<INT>  ;Disables interrupts and calls interrupt vector
```

where \<INT\> means the operation which is automatically executed by the
CPU when it executes an interrupt.

The effect of EI is delayed by one instruction. This means that EI
followed immediately by DI does not allow any interrupts between them.

### FFFF - IE - Interrupt Enable (R/W)

```
Bit 0: VBlank   Interrupt Enable  (INT 40h)  (1=Enable)
Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)
Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)
Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)
Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)
```

### FF0F - IF - Interrupt Flag (R/W)

```
Bit 0: VBlank   Interrupt Request (INT 40h)  (1=Request)
Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)
```

When an interrupt signal changes from low to high, the
corresponding bit in the IF register becomes set. For example, Bit 0
becomes set when the LCD controller enters the VBlank period.

### Interrupt Requests

Any set bits in the IF register are only **requesting** an interrupt to be
executed. The actual **execution** happens only if both the IME flag and
the corresponding bit in the IE register are set, otherwise the
interrupt "waits" until both IME and IE allow its execution.

### Interrupt Handling

1. The IME flag and the IF bit corresponding to this interrupt are reset by the CPU,
disabling any further interrupts until the program
re-enables them, typically by using the RETI instruction.
2. The corresponding interrupt vector (see the IE and IF register descriptions [above](#ffff-ie-interrupt-enable-rw)) is
called by the CPU. This is a regular call, exactly like what would be performed by a "call <vector>" instruction (the current PC is pushed on the stack
and then set to the address of the interrupt vector).

### Manually Requesting/Discarding Interrupts

Since the CPU automatically sets and clears the bits in the IF register, it
is usually not required to write to the IF register. However, the user
may still do that in order to manually request (or discard) interrupts.
Like with real interrupts, a manually requested interrupt isn't executed
unless/until IME and IE allow its execution.

### Interrupt Priorities

In the following three situations it might happen that more than one bit in the IF register is set, requesting more than one interrupt at once:

1. More than one interrupt signal changed from Low to High at the same time.
2. Several interrupts have been requested during a time in which IME/IE didn't allow these interrupts to be executed directly.
3. The user has written a value with several "1" bits (for example binary 00011111) to the IF register.

If IME and IE allow the execution of more than one of the
requested interrupts, the interrupt with the highest priority
is executed first. The priorities follow the same order as the bits in the IE
and IF registers: Bit 0 (VBlank) having the highest priority, and Bit 4
(Joypad) having the lowest priority.

### Nested Interrupts

The CPU automatically disables all the other interrupts by setting IME=0
when it executes an interrupt. Usually IME remains zero until the
interrupt handler returns (and sets IME=1 by means of the RETI instruction).
However, if you want any other interrupts (of any priority)
to be allowed to be executed from inside the interrupt
handler, then you can use the EI instruction in the interrupt
handler.

### Interrupt Service Routine

According to Z80 datasheets, the following occurs when control is being
transferred to an interrupt handler:

1. Two wait states are executed (2 machine cycles pass while nothing
occurs, presumably the CPU is executing NOPs during this time).
2. The current PC is pushed onto the stack, this process consumes 2 more
machine cycles.
3. The high byte of the PC is set to 0, the low byte is set to the
address of the handler ($40,$48,$50,$58,$60). This consumes one
last machine cycle.

The entire ISR **should** consume a total of 5 machine cycles. This has
yet to be tested, but is what the Z80 datasheet implies.

