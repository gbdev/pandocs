# MBC1

(max 2MByte ROM and/or 32 KiB RAM)

This is the first MBC chip for the Game Boy. Any newer MBC chips
work similarly, so it is relatively easy to upgrade a program from one
MBC chip to another — or to make it compatible with several
types of MBCs.

In its default configuration, MBC1 supports up to 512 KiB ROM with up to 32 KiB of banked RAM.
Some cartridges wire the MBC differently, where the 2-bit RAM banking register is wired as an extension of the ROM banking register (instead of to RAM) in order to support up to 2 MiB ROM, at the cost of only supporting a fixed 8 KiB of cartridge RAM.
All MBC1 cartridges with 1 MiB of ROM or more use this alternate wiring.
Also see the note on MBC1M multi-game compilation carts below.

Note that the memory in range 0000–7FFF is used both for reading from
ROM and writing to the MBCs Control Registers.

## Memory

### 0000–3FFF — ROM Bank X0 \[read-only\]

This area normally contains the first 16 KiB (bank 00) of the cartridge
ROM.

In 1 MiB and larger cartridges (that use the 2-bit second banking register for extended ROM banking), entering mode 1 (see below) will allow that second banking register to apply to reads from this region in addition to the regular 4000–7FFF banked region, resulting in accessing banks $20/$40/$60 for regular large ROM carts, or banks $10/$20/$30 for an MBC1M multi-cart (see note below).

### 4000–7FFF — ROM Bank 01-7F \[read-only\]

This area may contain any of the further 16 KiB banks of the ROM. If the main 5-bit ROM banking register is 0, it reads the bank as if it was set to 1.

For 1 MiB+ ROM, this means any bank that is possible to accessible via the 0000–3FFF region is not accessible in this region. i.e. banks $00/$20/$40/$60 in regular large ROM carts, or banks $00/$10/$20/$30 in MBC1M multi-game compilation carts. Instead, it automatically maps to 1 bank
higher ($01/$21/$41/$61 or $01/$11/$21/$31 respectively).

### A000–BFFF — RAM Bank 00–03, if any

This area is used to address external RAM in the cartridge (if any). The RAM is only accessible if RAM is enabled, otherwise reads return open bus values (often $FF, but not guaranteed) and writes are ignored.

Available RAM sizes are 8 KiB (at $A000–BFFF) and 32 KiB (in form of four 8K banks at $A000–BFFF). 32 KiB is only available in cartridges with ROM <= 512 KiB.

External RAM is often battery-backed, allowing for the storage of game data while the Game Boy is turned off, or if the cartridge is removed from the Game Boy. External RAM is no slower than the Game Boy's internal RAM, so many games use part of the external RAM as extra working RAM, even if they use another part of it for battery-backed saves.

## Registers

All of the MBC1 registers default to $00 on power-up, which for the "ROM Bank Number" register is _treated as_ $01.

### 0000–1FFF — RAM Enable (Write Only)

Before external RAM can be read or written, it must be enabled by
writing `$A` to anywhere in this address space.
Any value with `$A` in the lower 4 bits **enables** the RAM attached to the MBC, and any
other value **disables** the RAM. It is unknown why `$A` is the value used to enable RAM.

It is recommended to disable external RAM
after accessing it, in order to protect its contents from corruption during
power down of the Game Boy or removal of the cartridge. Once the cartridge has _completely_ lost power from the Game Boy, the RAM is automatically disabled to protect it.

### 2000–3FFF — ROM Bank Number (Write Only)

This 5-bit register (range $01-$1F) selects the ROM bank number for the 4000–7FFF region. Higher
bits are discarded — writing $E1 (binary 111**00001**) to this register
would select bank $01.

If this register is set to $00, it behaves as if it is set to $01. This means you cannot duplicate bank $00 into both the 0000–3FFF and 4000–7FFF ranges by setting this register to $00.

If the ROM Bank Number is set to a higher value than the number of banks
in the cart, the bank number is masked to the required number of bits.
e.g. a 256 KiB cart only needs a 4-bit bank number to address all of its
16 banks, so this register is masked to 4 bits. The upper bit would be
ignored for bank selection.

Even with smaller ROMs that use less than 5 bits for bank selection, the full 5-bit register is still compared for the bank 00→01 translation logic. As a result if the ROM is 256 KiB or smaller, it _is_ possible to map bank 0 to the 4000–7FFF region — by setting the 5th bit to 1 it will prevent the 00→01 translation (which looks at the full 5-bit register, and sees the value $10, not $00), while the bits actually used for bank selection (4, in this example) are all 0, so bank $00 is selected.

On larger carts which need a >5 bit bank number, the secondary banking
register at 4000–5FFF is used to supply an additional 2 bits for the
effective bank number:
`Selected ROM Bank = (Secondary Bank << 5) + ROM Bank`.[^MBC1M_banking]

These additional two bits are ignored for the bank 00→01 translation. This causes a problem — attempting to access banks $20, $40, and $60 only set bits in the upper 2-bit register, with the lower 5-bit register set to 00. As a result, any
attempt to address these ROM Banks will select Bank $21, $41 and $61
instead. The only way to access banks $20, $40 or $60 at all is to enter mode 1,
which remaps the 0000–3FFF range. This has its own problems for game
developers as that range contains interrupt handlers, so it's usually only
used in multi-game compilation carts (see below).

[^MBC1M_banking]: MBC1M has a different formula, see below.

### 4000–5FFF — RAM Bank Number — or — Upper Bits of ROM Bank Number (Write Only)

This second 2-bit register can be used to select a RAM Bank in range from
$00–$03 (32 KiB ram carts only), or to specify the upper two bits (bits 5-6)
of the ROM Bank number (1 MiB ROM or larger carts only). If neither ROM nor
RAM is large enough, setting this register does nothing.

In 1MB MBC1 multi-carts (see below), this 2-bit register is instead
applied to bits 4-5 of the ROM bank number and the top bit of the main
5-bit main ROM banking register is ignored.

### 6000–7FFF — Banking Mode Select (Write Only)

This 1-bit register selects between the two MBC1 banking modes, controlling
the behaviour of the secondary 2-bit banking register (above). If the cart
is not large enough to use the 2-bit register (≤ 8 KiB RAM and ≤ 512 KiB ROM)
this mode select has no observable effect. The program may freely switch
between the two modes at any time.

{{#bits 8 >
  "Value written" 0:"Banking mode"
}}

The **banking mode** can be:
- `0` = *simple* (default): 0000–3FFF and A000–BFFF are locked to bank 0 of ROM and SRAM respectively.
- `1` = *advanced*: 0000–3FFF and A000-BFFF can be bank-switched via the [4000–5FFF register](<#4000–5FFF — RAM Bank Number — or — Upper Bits of ROM Bank Number (Write Only)>).


:::tip

Technically, the MBC1 has AND gates between the both bank registers and the second-highest bit of the address. This is intended to cause accesses to the 0000–3FFF region (which has that address bit set to 0) to treat both registers as always 0, so that only bank 0 is accessible through this address.

However, when the second bank register is connected to RAM, this has the side effect of also locking RAM to bank 0, as the RAM address space (A000–BFFF) _also_ has the second-highest address bit set to 0.

Setting the mode to 1 disables these AND gates, allowing the two-bit register to switch the selected bank in both these regions.

:::

## Addressing diagrams

The following diagrams show how the address within the ROM/RAM chips are calculated from the accessed address and banking registers

### 0000–3FFF

<div class=table-wrapper><table class=bit-descrs><thead><tr>
  <th></th><th>20</th><th>19</th><th>18</th><th>17</th><th>16</th><th>15</th><th>14</th><th>13</th><th>12</th><th>...</th><th>1</th><th>0</th>
</tr></thead><tbody><tr>
  <td><strong>Mode 0</strong></td><td colspan=2>0</td><td rowspan=2 colspan=5>0</td><td rowspan=2 colspan=5>From Game Boy address</td>
</tr><tr>
  <td><strong>Mode 1</strong></td><td colspan=2>From 4000–5FFF bank register</td>
</tr></tbody></table></div>

### 4000–7FFF

<div class=table-wrapper><table class=bit-descrs><thead><tr>
  <th></th><th>20</th><th>19</th><th>18</th><th>17</th><th>16</th><th>15</th><th>14</th><th>13</th><th>12</th><th>...</th><th>1</th><th>0</th>
</tr></thead><tbody><tr>
  <td><strong>Mode 0 / Mode 1</strong></td><td colspan=2>From 4000–5FFF bank register</td><td colspan=5>From 2000–3FFF bank register</td><td colspan=5>From Game Boy address</td>
</tr></tbody></table></div>

:::tip

In a smaller cartridge, some of the upper bits are ignored.
(For example, a 128 KiB = 2<sup>17</sup> bytes ROM only uses bits 0–16.)

:::

### A000–BFFF

<div class=table-wrapper><table class=bit-descrs><thead><tr>
  <th></th><th>14</th><th>13</th><th>12</th><th>...</th><th>1</th><th>0</th>
</tr></thead><tbody><tr>
  <td><strong>Mode 0</strong></td><td colspan=2>0</td><td rowspan=2 colspan=4>From Game Boy address</td>
</tr><tr>
  <td><strong>Mode 1</strong></td><td colspan=2>From 4000–5FFF bank register</td>
</tr></tbody></table></div>

## "MBC1M": 1 MiB Multi-Game Compilation Carts

Known as MBC1M, these carts have an alternative wiring, that ignores
the top bit of the main ROM banking register (making it effectively a 4-bit register for banking, though the full 5 bit register is still used for 00→01 translation)
and applies the 2-bit register to bits 4-5 of the bank number (instead of
the usual bits 5-6). This means that in mode 1 the 2-bit register selects
banks $00, $10, $20, or $30, rather than the usual $00, $20, $40 or $60.

These carts make use of the fact that mode 1 remaps the 0000–3FFF area
to switch games. The 2-bit register is used to select the game — switching
the zero bank and the region of banks that the 4000–7FFF ROM area can
access to those for the selected game and then the game only changes the
main ROM banking register. As far as the selected game knows, it's running
from a 256 KiB cart!

These carts can normally be identified by having a Nintendo copyright
header in bank $10. A badly dumped multi-cart ROM can be identified by
having duplicate content in banks $10-$1F (dupe of $00–$0F) and banks $30-$3F
(dupe of $20-$2F).
There is a known bad dump of the Mortal Kombat I & II collection around.

An "MBC1M" compilation cart ROM can be converted into a regular MBC1 ROM
by increasing the ROM size to 2 MiB and duplicating each sub-ROM — $00–$0F
duplicated into $10-$1F, the original $10-$1F placed in $20-$2F and
duplicated into $30-$3F and so on.

### MBC1M addressing diagrams

#### 0000–3FFF

<div class=table-wrapper><table class=bit-descrs><thead><tr>
  <th></th><th>19</th><th>18</th><th>17</th><th>16</th><th>15</th><th>14</th><th>13</th><th>12</th><th>..</th><th>1</th><th>0</th>
</tr></thead><tbody><tr>
  <td><strong>Mode 0</strong></td><td colspan=2>0</td><td colspan=4>0</td><td rowspan=2 colspan=5>From Game Boy address</td>
</tr><tr>
  <td><strong>Mode 1</strong></td><td colspan=2>From 4000–5FFF bank register</td><td colspan=4>From 2000–3FFF bank register (bit 4 unused)</td>
</tr></tbody></table></div>

#### 4000–7FFF

<div class=table-wrapper><table class=bit-descrs><thead><tr>
  <th></th><th>19</th><th>18</th><th>17</th><th>16</th><th>15</th><th>14</th><th>13</th><th>12</th><th>..</th><th>1</th><th>0</th>
</tr></thead><tbody><tr>
  <td><strong>Mode 0 / Mode 1</strong></td><td colspan=2>From 4000–5FFF bank register</td><td colspan=4>From 2000–3FFF bank register (bit 4 unused)</td><td colspan=5>From Game Boy address</td>
</tr></tbody></table></div>
