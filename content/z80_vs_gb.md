The CPU is a Z80 workalike running at 4.19 MHz. The CPU has several
registers missing and some instructions changes. These are as follows:

* The "shadow" set of registers [BC',DE',HL',AF'] and the index registers
  [IX,IY] are missing and, consequently, there are no DD and FD opcode
tables.

- The I/O ports are gone and so are all IN/OUT opcodes.

- HALT is interrupted even when interrupts are disabled.

- Following Z80 opcodes are changed:

|Code   |    Z80 operation | GameBoy operation |   |
|-------|------------------|-------------------|---|
| 08 xx xx   |EX AF,AF'      |LD (word),SP|     Save SP at given address
| 10 xx      |DJNZ offset    |STOP        |     Meaning unknown
| 22         |LD (word),HL   |LD (HLI),A  |     Save A at (HL) and increment HL
| 2A         |LD HL,(word)   |LD A,(HLI)  |     Load A from (HL) and increment HL
| 32         |LD (word),A    |LD (HLD),A  |     Save A at (HL) and decrement HL
| 3A         |LD A,(word)    |LD A,(HLD)  |     Load A from (HL) and decrement HL
| D3         |OUTA (byte)    |No operation|
| D9         |EXX            |RETI        |     Enable interrupts and return
| DB         |INA (byte)     |No operation|
| DD         |Prefix DD      |No operation|
| E0 xx      |RET PO         |LD (byte),A |     Save A at (FF00+byte)
| E2         |JP PO,word     |LD (C),A    |     Save A at (FF00+C)
| E3         |EX HL,(SP)     |No operation|
| E4         |CALL PO,word   |No operation|
| E8 xx      |RET PE         |ADD SP,offset |   Add signed offset to SP
| EA xx xx   |JP PE,word     |LD (word),A |     Save A at given address
| EB         |EX DE,HL       |No operation|
| EC         |CALL PE,word   |No operation|
| F0 xx      |RET P          |LD A,(byte) |     Load A from (FF00+byte)
| F2         |JP P,word      |No operation|
| F4         |CALL P,word    |No operation|
| F8 xx      |RET M          |LDHL SP,offset |  Load HL with SP + signed offset
| FA xx xx   |JP M,word      |LD A,(word) |     Load A from given address
| FC         |CALL M,word    |No operation|
| FD         |Prefix FD      |No operation|
