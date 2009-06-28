### Comparision with 8080

Basically, the gameboy CPU works more like an older 8080 CPU rather than
like a more powerful Z80 CPU. It is, however, supporting CB-prefixed
instructions. Also, all known gameboy assemblers using the more obvious
Z80-style syntax, rather than the chaotic 8080-style syntax.

### Comparision with Z80

Any DD-, ED-, and FD-prefixed instructions are missing, that means no
IX-, IY-registers, no block commands, and some other missing commands.
All exchange instructions have been removed (including total absence of
second register set), 16bit memory accesses are mostly missing, and
16bit arithmetic functions are heavily cut-down. The gameboy has no
IN/OUT instructions, instead I/O ports are accessed directly by normal
LD instructions, or by special LD (FF00+n) opcodes. The sign and
parity/overflow flags have been removed. The gameboy operates
approximately as fast as a 4MHz Z80 (8MHz in CGB double speed mode),
execution time of all instructions has been rounded up to a multiple of
4 cycles though.

### Moved, Removed, and Added Opcodes

` Opcode  Z80             GMB`\
` ---------------------------------------`\
` 08      EX   AF,AF      LD   (nn),SP`\
` 10      DJNZ PC+dd      STOP`\
` 22      LD   (nn),HL    LDI  (HL),A`\
` 2A      LD   HL,(nn)    LDI  A,(HL)`\
` 32      LD   (nn),A     LDD  (HL),A`\
` 3A      LD   A,(nn)     LDD  A,(HL)`\
` D3      OUT  (n),A      -`\
` D9      EXX             RETI`\
` DB      IN   A,(n)      -`\
` DD      `<IX>`            -`\
` E0      RET  PO         LD   (FF00+n),A`\
` E2      JP   PO,nn      LD   (FF00+C),A`\
` E3      EX   (SP),HL    -`\
` E4      CALL P0,nn      -`\
` E8      RET  PE         ADD  SP,dd`\
` EA      JP   PE,nn      LD   (nn),A`\
` EB      EX   DE,HL      -`\
` EC      CALL PE,nn      -`\
` ED      `<pref>`          -`\
` F0      RET  P          LD   A,(FF00+n)`\
` F2      JP   P,nn       LD   A,(FF00+C)`\
` F4      CALL P,nn       -`\
` F8      RET  M          LD   HL,SP+dd`\
` FA      JP   M,nn       LD   A,(nn)`\
` FC      CALL M,nn       -`\
` FD      `<IY>`            -`\
` CB3X    SLL  r/(HL)     SWAP r/(HL)`

Note:
-----

The unused (-) opcodes will lock-up the gameboy CPU when used.

