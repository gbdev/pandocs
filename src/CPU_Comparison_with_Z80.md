# CPU Comparison with Z80

## Comparison with 8080

The Game Boy CPU has a bit more in common with an older Intel 8080 CPU
than the more powerful Zilog Z80 CPU. It is missing a handful of 8080
instructions but does support JR and almost all CB-prefixed
instructions. Also, all known Game Boy assemblers use the more obvious
Z80-style syntax, rather than the chaotic 8080-style syntax.

Unlike the 8080 and Z80, the Game Boy has no dedicated I/O bus and no
IN/OUT opcodes. Instead, I/O ports are accessed directly by normal LD
instructions, or by new LD (FF00+n) opcodes.

The sign and parity/overflow flags have been removed, as have the 12
RET, CALL, and JP instructions conditioned on them. So have EX (SP),HL
(XTHL) and EX DE,HL (XCHG).

## Comparison with Z80

In addition to the removed 8080 instructions, the other exchange
instructions have been removed (including total absence of second
register set).

All DD- and FD-prefixed instructions are missing. That means no IX- or
IY-registers.

All ED-prefixed instructions are missing. That means 16-bit memory
accesses are mostly missing, 16-bit arithmetic functions are heavily
cut-down, and some other missing instructions. IN/OUT (C) are replaced with
new LD (\$FF00+C) opcodes. Block instructions are gone, but autoincrementing
HL accesses are added.

The Game Boy operates approximately as fast as a 4 MHz Z80 (8 MHz in CGB
double speed mode), with execution time of all instructions having been
rounded up to a multiple of 4 cycles.

## Moved, Removed, and Added Opcodes


 Opcode | Z80            | GB CPU
--------|----------------|-------------
 08     | EX   AF,AF     | LD   (nn),SP
 10     | DJNZ PC+dd     | STOP
 22     | LD   (nn),HL   | LDI  (HL),A
 2A     | LD   HL,(nn)   | LDI  A,(HL)
 32     | LD   (nn),A    | LDD  (HL),A
 3A     | LD   A,(nn)    | LDD  A,(HL)
 D3     | OUT  (n),A     | -
 D9     | EXX            | RETI
 DB     | IN   A,(n)     | -
 DD     | \<IX\> prefix  | -
 E0     | RET  PO        | LD   (FF00+n),A
 E2     | JP   PO,nn     | LD   (FF00+C),A
 E3     | EX   (SP),HL   | -
 E4     | CALL P0,nn     | -
 E8     | RET  PE        | ADD  SP,dd
 EA     | JP   PE,nn     | LD   (nn),A
 EB     | EX   DE,HL     | -
 EC     | CALL PE,nn     | -
 ED     | \<prefix\>     | -
 F0     | RET  P         | LD   A,(FF00+n)
 F2     | JP   P,nn      | LD   A,(FF00+C)
 F4     | CALL P,nn      | -
 F8     | RET  M         | LD   HL,SP+dd
 FA     | JP   M,nn      | LD   A,(nn)
 FC     | CALL M,nn      | -
 FD     | \<IY\> prefix  | -
 CB 3X  | SLL  r/(HL)    | SWAP r/(HL)

Note: The unused (-) opcodes will lock up the Game Boy CPU when used.

