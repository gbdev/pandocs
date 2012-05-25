### IME - Interrupt Master Enable Flag (Write Only)

` 0 - Disable all Interrupts`\
` 1 - Enable all Interrupts that are enabled in IE Register (FFFF)`

The IME flag is used to disable all interrupts, overriding any enabled
bits in the IE Register. It isn\'t possible to access the IME flag by
using a I/O address, instead IME is accessed directly from the CPU, by
the following opcodes/operations:

` EI     ;Enable Interrupts  (ie. IME=1)`\
` DI     ;Disable Interrupts (ie. IME=0)`\
` RETI   ;Enable Ints & Return (same as the opcode combination EI, RET)`\
` `<INT>`  ;Disable Ints & Call to Interrupt Vector`

Whereas <INT> means the operation which is automatically executed by the
CPU when it executes an interrupt.

### FFFF - IE - Interrupt Enable (R/W)

` Bit 0: V-Blank  Interrupt Enable  (INT 40h)  (1=Enable)`\
` Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)`\
` Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)`\
` Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)`\
` Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)`

### FF0F - IF - Interrupt Flag (R/W)

` Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)`\
` Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)`\
` Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)`\
` Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)`\
` Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)`

When an interrupt signal changes from low to high, then the
corresponding bit in the IF register becomes set. For example, Bit 0
becomes set when the LCD controller enters into the V-Blank period.

### Interrupt Requests

Any set bits in the IF register are only <requesting> an interrupt to be
executed. The actual <execution> happens only if both the IME flag, and
the corresponding bit in the IE register are set, otherwise the
interrupt \'waits\' until both IME and IE allow its execution.

### Interrupt Execution

When an interrupt gets executed, the corresponding bit in the IF
register becomes automatically reset by the CPU, and the IME flag
becomes cleared (disabeling any further interrupts until the program
re-enables the interrupts, typically by using the RETI instruction), and
the corresponding Interrupt Vector (that are the addresses in range
0040h-0060h, as shown in IE and IF register decriptions above) becomes
called.

### Manually Requesting/Discarding Interrupts

As the CPU automatically sets and cleares the bits in the IF register it
is usually not required to write to the IF register. However, the user
may still do that in order to manually request (or discard) interrupts.
As for real interrupts, a manually requested interrupt isn\'t executed
unless/until IME and IE allow its execution.

### Interrupt Priorities

In the following three situations it might happen that more than 1 bit
in the IF register are set, requesting more than one interrupt at once:

` 1) More than one interrupt signal changed from Low`\
`    to High at the same time.`\
` 2) Several interrupts have been requested during a`\
`    time in which IME/IE didn't allow these interrupts`\
`    to be executed directly.`\
` 3) The user has written a value with several "1" bits`\
`    (for example 1Fh) to the IF register.`

Provided that IME and IE allow the execution of more than one of the
requested interrupts, then the interrupt with the highest priority
becomes executed first. The priorities are ordered as the bits in the IE
and IF registers, Bit 0 (V-Blank) having the highest priority, and Bit 4
(Joypad) having the lowest priority.

### Nested Interrupts

The CPU automatically disables all other interrupts by setting IME=0
when it executes an interrupt. Usually IME remains zero until the
interrupt procedure returns (and sets IME=1 by the RETI instruction).
However, if you want any other interrupts of lower or higher (or same)
priority to be allowed to be executed from inside of the interrupt
procedure, then you can place an EI instruction into the interrupt
procedure.

### Interrupt Service Routine

According to Z80 datasheets, the following occurs when control is being
transferred to an interrupt handler:

1\. Two wait states are executed (2 machine cycles pass while nothing
occurs, presumably the CPU is executing NOPs during this time). 2. The
current PC is pushed onto the stack, this process consumes 2 more
machine cycles. 3. The high byte of the PC is set to 0, the low byte is
set to the address of the handler (\$40,\$48,\$50,\$58,\$60). This
consumes one last machine cycle.

The entire ISR \*should\* consume a total of 5 machine cycles. This has
yet to be tested, but is what the Z80 datasheet implies.

