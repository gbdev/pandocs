### Registers


 16-bit|Hi|Lo|Name/Function
:-----:|:-----:|:-----:|:-----:
 AF| A| -| Accumulator & Flags
 BC| B| C| BC
 DE| D| E| DE
 HL| H| L| HL
 SP| -| -| Stack Pointer
 PC| -| -| Program Counter/Pointer


As shown above, most registers can be accessed either as one 16-bit
register, or as two separate 8-bit registers.

### The Flag Register (lower 8-bit of AF register)

Bit|Name|Set|Clr|Expl.
:-----:|:-----:|:-----:|:-----:|:-----:
7|zf|Z|NZ|Zero Flag
6|n| -|-|Add/Sub-Flag (BCD)
5|h| -|-|Half Carry Flag (BCD)
4|cy|C|NC|Carry Flag


Contains the result from the recent instruction which has affected
flags.

### The Zero Flag (Z)

This bit becomes set (1) if the result of an operation has been zero
(0). Used for conditional jumps.

### The Carry Flag (C, or Cy)

Becomes set when the result of an addition became bigger than FFh (8-bit)
or FFFFh (16-bit). Or when the result of a subtraction or comparison
became less than zero (much as for Z80 and 80x86 CPUs, but unlike as for
65XX and ARM CPUs). Also the flag becomes set when a rotate/shift
operation has shifted-out a \"1\"-bit. Used for conditional jumps, and
for instructions such like ADC, SBC, RL, RLA, etc.

### The BCD Flags (N, H)

These flags are (rarely) used for the DAA instruction only, N Indicates
whether the previous instruction has been an addition or subtraction,
and H indicates carry for lower 4 bits of the result, also for DAA, the C
flag must indicate carry for upper 8 bits. After adding/subtracting two
BCD numbers, DAA is intended to convert the result into BCD format; BCD
numbers are ranged from 00h to 99h rather than 00h to FFh. Because C and
H flags must contain carry-outs for each digit, DAA cannot be used for
16-bit operations (which have 4 digits), and use for INC/DEC operations
(which do not affect C-flag) has limits.

