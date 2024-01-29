# CPU Instruction Set

:::tip

If you are looking for textual explanations of what each each instruction does, please read [gbz80(7)](http://rgbds.gbdev.io/docs/gbz80.7); if you want a compact reference card/cheat sheet of each opcode and its flag effects, please consult [the optables](http://gbdev.io/gb-opcodes/optables) (whose [octal view](http://gbdev.io/gb-opcodes/optables/octal) makes most encoding patterns more apparent).

:::

<style>table td { padding: 3px 10px; overflow-wrap: break-word; }</style>

The Game Boy's SM83 processor possesses a <abbr title="Complex Instruction Set Computer">CISC</abbr>, variable-length instruction set.
This page attempts to shed some light on how the CPU decodes the raw bytes fed into it into instructions.

The first byte of each instruction is typically called the "opcode" (for "operation code").
By noticing that some instructions perform identical operations but with different parameters, they can be grouped together; for example, `inc bc`, `inc de`, `inc hl`, and `inc sp` differ only in what 16-bit register they modify.

In each table, one line represents one such grouping.
Since many groupings have some variation, the variation has to be encoded in the instruction; for example, the above four instructions will be collectively referred to as `inc r16`.
Here are the possible placeholders and their values:

{{#bits 8 <
	"r8"      0:"<code>b</code>" 1:"<code>c</code>" 2:"<code>d</code>" 3:"<code>e</code>" 4:"<code>h</code>" 5:"<code>l</code>" 6:"<code>[hl]</code>" 7:"<code>a</code>" ;
	"r16"     0:"<code>bc</code>" 1:"<code>de</code>" 2:"<code>hl</code>" 3:"<code>sp</code>" ;
	"r16stk"  0:"<code>bc</code>" 1:"<code>de</code>" 2:"<code>hl</code>" 3:"<code>af</code>" ;
	"r16mem"  0:"<code>bc</code>" 1:"<code>de</code>" 2:"<code>hl+</code>" 3:"<code>hl-</code>" ;
	"cond"    0:"<code>nz</code>" 1:"<code>z</code>" 2:"<code>nc</code>" 3:"<code>c</code>" ;
	"b3"      0-7:"A 3-bit bit index" ;
	"tgt3"    0-7:"<code>rst</code>'s target address, divided by 8" ;
	"imm8"    0-7:"The following byte" ;
	"imm16"   0-7:"The following two bytes, in little-endian order" ;
}}

These last two are a little special: if they are present in the instruction's mnemonic, it means that the instruction is 1 (`imm8`) / 2 (`imm16`) extra bytes long.

:::tip

`[hl+]` and `[hl-]` can also be notated `[hli]` and `[hld]` respectively (as in **i**ncrement and **d**ecrement).

:::

Groupings have been loosely associated based on what they do into separate tables; those have no particular ordering, and are purely for readability and convenience.
Finally, the instruction "families" have been further grouped into four "blocks", differentiated by the first two bits of the opcode.

## Block 0

{{#bits 8 >
	"<code>nop</code>"  7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2:"0" 1:"0" 0:"0"
}}

{{#bits 8 >
	"<code>ld r16, imm16</code>"   7:"0" 6:"0" 5-4:"Dest (r16)" 3:"0" 2:"0" 1:"0" 0:"1" ;
	"<code>ld [r16mem], a</code>"  7:"0" 6:"0" 5-4:"Dest (r16mem)" 3:"0" 2:"0" 1:"1" 0:"0" ;
	"<code>ld a, [r16mem]</code>"  7:"0" 6:"0" 5-4:"Source (r16mem)" 3:"1" 2:"0" 1:"1" 0:"0" ;
	"<code>ld [imm16], sp</code>"  7:"0" 6:"0" 5:"0" 4:"0" 3:"1" 2:"0" 1:"0" 0:"0" ;
}}

{{#bits 8 >
	"<code>inc r16</code>"      7:"0" 6:"0" 5-4:"Operand (r16)" 3:"0" 2:"0" 1:"1" 0:"1" ;
	"<code>dec r16</code>"      7:"0" 6:"0" 5-4:"Operand (r16)" 3:"1" 2:"0" 1:"1" 0:"1" ;
	"<code>add hl, r16</code>"  7:"0" 6:"0" 5-4:"Operand (r16)" 3:"1" 2:"0" 1:"0" 0:"1" ;
}}

{{#bits 8 >
	"<code>inc r8</code>"  7:"0" 6:"0" 5-3:"Operand (r8)" 2:"1" 1:"0" 0:"0" ;
	"<code>dec r8</code>"  7:"0" 6:"0" 5-3:"Operand (r8)" 2:"1" 1:"0" 0:"1" ;
}}

{{#bits 8 >
	"<code>ld r8, imm8</code>"  7:"0" 6:"0" 5-3:"Dest (r8)" 2:"1" 1:"1" 0:"0"
}}

{{#bits 8 >
	"<code>rlca</code>"  7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2:"1" 1:"1" 0:"1" ;
	"<code>rrca</code>"  7:"0" 6:"0" 5:"0" 4:"0" 3:"1" 2:"1" 1:"1" 0:"1" ;
	"<code>rla</code>"   7:"0" 6:"0" 5:"0" 4:"1" 3:"0" 2:"1" 1:"1" 0:"1" ;
	"<code>rra</code>"   7:"0" 6:"0" 5:"0" 4:"1" 3:"1" 2:"1" 1:"1" 0:"1" ;
	"<code>daa</code>"   7:"0" 6:"0" 5:"1" 4:"0" 3:"0" 2:"1" 1:"1" 0:"1" ;
	"<code>cpl</code>"   7:"0" 6:"0" 5:"1" 4:"0" 3:"1" 2:"1" 1:"1" 0:"1" ;
	"<code>scf</code>"   7:"0" 6:"0" 5:"1" 4:"1" 3:"0" 2:"1" 1:"1" 0:"1" ;
	"<code>ccf</code>"   7:"0" 6:"0" 5:"1" 4:"1" 3:"1" 2:"1" 1:"1" 0:"1" ;
}}

{{#bits 8 >
	"<code>jr imm8</code>"        7:"0" 6:"0" 5:"0" 4:"1" 3:"1" 2:"0" 1:"0" 0:"0" ;
	"<code>jr cond, imm8</code>"  7:"0" 6:"0" 5:"1" 4-3:"Condition (cond)" 2:"0" 1:"0" 0:"0" ;
}}

{{#bits 8 >
	"<code>stop</code>"  7:"0" 6:"0" 5:"0" 4:"1" 3:"0" 2:"0" 1:"0" 0:"0"
}}

[`stop`](<#Using the STOP Instruction>) is often considered a **two-byte** instruction, though [the second byte is not always ignored](https://gist.github.com/SonoSooS/c0055300670d678b5ae8433e20bea595#nop-and-stop).

## Block 1: 8-bit register-to-register loads

{{#bits 8 >
	"<code>ld r8, r8</code>"  7:"0" 6:"1" 5-3:"Dest (r8)" 2-0:"Source (r8)"
}}

**Exception**: trying to encode `ld [hl], [hl]` instead yields [the `halt` instruction](<#halt>):

{{#bits 8 >
	"<code>halt</code>"  7:"0" 6:"1" 5:"1" 4:"1" 3:"0" 2:"1" 1:"1" 0:"0"
}}

## Block 2: 8-bit arithmetic

{{#bits 8 >
	"<code>add a, r8</code>"  7:"1" 6:"0" 5:"0" 4:"0" 3:"0" 2-0:"Operand (r8)" ;
	"<code>adc a, r8</code>"  7:"1" 6:"0" 5:"0" 4:"0" 3:"1" 2-0:"Operand (r8)" ;
	"<code>sub a, r8</code>"  7:"1" 6:"0" 5:"0" 4:"1" 3:"0" 2-0:"Operand (r8)" ;
	"<code>sbc a, r8</code>"  7:"1" 6:"0" 5:"0" 4:"1" 3:"1" 2-0:"Operand (r8)" ;
	"<code>and a, r8</code>"  7:"1" 6:"0" 5:"1" 4:"0" 3:"0" 2-0:"Operand (r8)" ;
	"<code>xor a, r8</code>"  7:"1" 6:"0" 5:"1" 4:"0" 3:"1" 2-0:"Operand (r8)" ;
	"<code>or a, r8</code>"   7:"1" 6:"0" 5:"1" 4:"1" 3:"0" 2-0:"Operand (r8)" ;
	"<code>cp a, r8</code>"   7:"1" 6:"0" 5:"1" 4:"1" 3:"1" 2-0:"Operand (r8)" ;
}}

## Block 3

{{#bits 8 >
	"<code>add a, imm8</code>"  7:"1" 6:"1" 5:"0" 4:"0" 3:"0" 2:"1" 1:"1" 0:"0" ;
	"<code>adc a, imm8</code>"  7:"1" 6:"1" 5:"0" 4:"0" 3:"1" 2:"1" 1:"1" 0:"0" ;
	"<code>sub a, imm8</code>"  7:"1" 6:"1" 5:"0" 4:"1" 3:"0" 2:"1" 1:"1" 0:"0" ;
	"<code>sbc a, imm8</code>"  7:"1" 6:"1" 5:"0" 4:"1" 3:"1" 2:"1" 1:"1" 0:"0" ;
	"<code>and a, imm8</code>"  7:"1" 6:"1" 5:"1" 4:"0" 3:"0" 2:"1" 1:"1" 0:"0" ;
	"<code>xor a, imm8</code>"  7:"1" 6:"1" 5:"1" 4:"0" 3:"1" 2:"1" 1:"1" 0:"0" ;
	"<code>or a, imm8</code>"   7:"1" 6:"1" 5:"1" 4:"1" 3:"0" 2:"1" 1:"1" 0:"0" ;
	"<code>cp a, imm8</code>"   7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"1" 1:"1" 0:"0" ;
}}

{{#bits 8 >
	"<code>ret cond</code>"          7:"1" 6:"1" 5:"0" 4-3:"Condition (cond)" 2:"0" 1:"0" 0:"0" ;
	"<code>ret</code>"               7:"1" 6:"1" 5:"0" 4:"0" 3:"1" 2:"0" 1:"0" 0:"1" ;
	"<code>reti</code>"              7:"1" 6:"1" 5:"0" 4:"1" 3:"1" 2:"0" 1:"0" 0:"1" ;
	"<code>jp cond, imm16</code>"    7:"1" 6:"1" 5:"0" 4-3:"Condition (cond)" 2:"0" 1:"1" 0:"0" ;
	"<code>jp imm16</code>"          7:"1" 6:"1" 5:"0" 4:"0" 3:"0" 2:"0" 1:"1" 0:"1" ;
	"<code>jp hl</code>"             7:"1" 6:"1" 5:"1" 4:"0" 3:"1" 2:"0" 1:"0" 0:"1" ;
	"<code>call cond, imm16</code>"  7:"1" 6:"1" 5:"0" 4-3:"Condition (cond)" 2:"1" 1:"0" 0:"0" ;
	"<code>call imm16</code>"        7:"1" 6:"1" 5:"0" 4:"0" 3:"1" 2:"1" 1:"0" 0:"1" ;
	"<code>rst tgt3</code>"          7:"1" 6:"1" 5-3:"Target (tgt3)" 2:"1" 1:"1" 0:"1" ;
}}

{{#bits 8 >
	"<code>pop r16stk</code>"   7:"1" 6:"1" 5-4:"Register (r16stk)" 3:"0" 2:"0" 1:"0" 0:"1" ;
	"<code>push r16stk</code>"  7:"1" 6:"1" 5-4:"Register (r16stk)" 3:"0" 2:"1" 1:"0" 0:"1" ;
}}

{{#bits 8 >
	"Prefix (see block below)"  7:"1" 6:"1" 5:"0" 4:"0" 3:"1" 2:"0" 1:"1" 0:"1"
}}

{{#bits 8 >
	"<code>ldh [c], a</code>"     7:"1" 6:"1" 5:"1" 4:"0" 3:"0" 2:"0" 1:"1" 0:"0" ;
	"<code>ldh [imm8], a</code>"  7:"1" 6:"1" 5:"1" 4:"0" 3:"0" 2:"0" 1:"0" 0:"0" ;
	"<code>ld [imm16], a</code>"  7:"1" 6:"1" 5:"1" 4:"0" 3:"1" 2:"1" 1:"0" 0:"0" ;
	"<code>ldh a, [c]</code>"     7:"1" 6:"1" 5:"1" 4:"1" 3:"0" 2:"0" 1:"1" 0:"0" ;
	"<code>ldh a, [imm8]</code>"  7:"1" 6:"1" 5:"1" 4:"1" 3:"0" 2:"0" 1:"0" 0:"0" ;
	"<code>ld a, [imm16]</code>"  7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"1" 1:"0" 0:"0" ;
}}

{{#bits 8 >
	"<code>add sp, imm8</code>"      7:"1" 6:"1" 5:"1" 4:"0" 3:"1" 2:"0" 1:"0" 0:"0" ;
	"<code>ld hl, sp + imm8</code>"  7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"0" 1:"0" 0:"0" ;
	"<code>ld sp, hl</code>"         7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"0" 1:"0" 0:"1" ;
}}

{{#bits 8 >
	"<code>di</code>"  7:"1" 6:"1" 5:"1" 4:"1" 3:"0" 2:"0" 1:"1" 0:"1" ;
	"<code>ei</code>"  7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"0" 1:"1" 0:"1" ;
}}

The following opcodes are **invalid**, and [hard-lock the CPU](https://gist.github.com/SonoSooS/c0055300670d678b5ae8433e20bea595#opcode-holes-not-implemented-opcodes) until the console is powered off: \$D3, \$DB, \$DD, \$E3, \$E4, \$EB, \$EC, \$ED, \$F4, \$FC, and \$FD.

## \$CB prefix instructions

{{#bits 8 >
	"<code>rlc r8</code>"   7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2-0:"Operand (r8)" ;
	"<code>rrc r8</code>"   7:"0" 6:"0" 5:"0" 4:"0" 3:"1" 2-0:"Operand (r8)" ;
	"<code>rl r8</code>"    7:"0" 6:"0" 5:"0" 4:"1" 3:"0" 2-0:"Operand (r8)" ;
	"<code>rr r8</code>"    7:"0" 6:"0" 5:"0" 4:"1" 3:"1" 2-0:"Operand (r8)" ;
	"<code>sla r8</code>"   7:"0" 6:"0" 5:"1" 4:"0" 3:"0" 2-0:"Operand (r8)" ;
	"<code>sra r8</code>"   7:"0" 6:"0" 5:"1" 4:"0" 3:"1" 2-0:"Operand (r8)" ;
	"<code>swap r8</code>"  7:"0" 6:"0" 5:"1" 4:"1" 3:"0" 2-0:"Operand (r8)" ;
	"<code>srl r8</code>"   7:"0" 6:"0" 5:"1" 4:"1" 3:"1" 2-0:"Operand (r8)" ;
}}

{{#bits 8 >
	"<code>bit b3, r8</code>"  7:"0" 6:"1" 5-3:"Bit index (b3)" 2-0:"Operand (r8)" ;
	"<code>res b3, r8</code>"  7:"1" 6:"0" 5-3:"Bit index (b3)" 2-0:"Operand (r8)" ;
	"<code>set b3, r8</code>"  7:"1" 6:"1" 5-3:"Bit index (b3)" 2-0:"Operand (r8)" ;
}}
