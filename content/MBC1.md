(max 2MByte ROM and/or 32KByte RAM)

This is the first MBC chip for the Game Boy. Any newer MBC chips are
working similiar, so that is relative easy to upgrade a program from one
MBC chip to another - or even to make it compatible to several different
types of MBCs.

Note that the memory in range 0000-7FFF is used for both reading from
ROM, and for writing to the MBCs Control Registers.

## ROM/RAM access

### 0000-3FFF - ROM Bank 00/20/40/60 (Read Only)

This area normally contains the first 16KBytes (bank 00) of the cartridge
ROM. Can contain banks 20/40/60 in mode 1 (see below), or banks 10/20/30
in mode 1 for a 1MB MBC1 multi-cart.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

This area may contain any of the further 16KByte banks of the ROM. Cannot
address any banks where the main ROM banking register would be 00h, which
usually means banks 00/20/40/60. Instead, it automatically maps to 1 bank
higher (01/21/41/61).

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

This area is used to address external RAM in the cartridge (if any).
External RAM is often battery buffered, allowing to store game positions
or high score tables, even if the Game Boy is turned off, or if the
cartridge is removed from the Game Boy. Available RAM sizes are: 2KByte
(at A000-A7FF), 8KByte (at A000-BFFF), and 32KByte (in form of four 8K
banks at A000-BFFF).

## Control Registers

### 0000-1FFF - RAM Enable (Write Only)

Before external RAM can be read or written, it must be enabled by
writing to this address space. It is recommended to disable external RAM
after accessing it, in order to protect its contents from damage during
power down of the Game Boy. Usually the following values are used:

```
00h  Disable RAM (default)
0Ah  Enable RAM`
```

Practically any value with 0Ah in the lower 4 bits enables RAM, and any
other value disables RAM. RAM is automatically disabled when the gameboy
is powered off. It is unknown why Ah is the value used to enable RAM.

### 2000-3FFF - ROM Bank Number (Write Only)

This 5 bit register (range 01h-1Fh) selects the ROM bank number. Higher
bits are discarded - E1h (binary ~~111~~00001) would select bank 01h.
If the ROM Bank Number is set to a higher value than the number of banks
in the cart, the bank number is masked to the required number of bits.
e.g. a 256 kB cart only needs a 4 bit bank number to address all of its
16 banks, so this register is masked to 4 bits. The upper bit would be
ignored.

On larger carts which need a >5 bit bank number, the secondary banking
register at 4000-5FFF is used to supply an additional 2 bits for the
effective bank number:
`Selected ROM Bank = (Secondary Bank << 5) + ROM Bank`.

The ROM Bank Number defaults to 01h at power-on. When 00h is written,
the MBC translates that to bank 01h also. Not being able to select bank
00h isn't normally a problem, because bank 00h is usually mapped to the
0000-3FFF range. But on large carts (using the secondary banking register
to specify the upper ROM Bank bits), the same happens for banks 20h, 40h,
and 60h, as this register would need to be 00h for those addresses. Any
attempt to address these ROM Banks will select Bank 21h, 41h, and 61h
instead. The only way to access banks 20h, 40h or 60h is to enter mode 1,
which remaps the 0000-3FFF range. This has its own problems for game
developers as that range contains interrupt handlers, so it's mostly only
used in multi-game compilation carts (see below).

### 4000-5FFF - RAM Bank Number - or - Upper Bits of ROM Bank Number (Write Only)

This second 2 bit register can be used to select a RAM Bank in range from
00-03h (32 kB ram carts only), or to specify the upper two bits (Bit 5-6)
of the ROM Bank number (1 MB ROM or larger carts only). If neither ROM nor
RAM is large enough, setting this register does nothing.

In 1MB MBC1 multi-carts (see below), this 2 bit register is instead
applied to bits 4-5 of the ROM bank number, and the top bit of the main
5-bit main ROM banking register is ignored.

### 6000-7FFF - Banking Mode Select (Write Only)

This 1bit Register selects between the two MBC1 banking modes, controlling
the behaviour of the secondary 2 bit banking register (above). If the cart
is not large enough to use the 2 bit register (<= 8kB RAM / <= 512 kB ROM)
this mode select has no observable effect. The program may freely switch
between the two modes at any time.

```
00h = Simple ROM Banking Mode (default)
01h = RAM Banking Mode / Advanced ROM Banking Mode
```

In mode 0, the 2-bit secondary banking register can only affect the
4000-7FFF banked ROM area. If the cart is a "small ROM"/"large RAM" cart
(<1 MB ROM, >8kB RAM) then 4000-7FFF is unaffected by this register anyway,
so the practical effect is that RAM banking is disabled and A000-BFFF is
locked to only be able to access bank 0 of RAM, with the 2-bit secondary
banking register entirely ignored. For large ROM carts, the 

In mode 1, the behaviour differs depending on whether the current cart is
a "large RAM" cart (>8kB RAM) or "large ROM" cart (1 MB or larger). For
large RAM carts, switching to mode 1 enables RAM banking and (if RAM is
enabled) immediately switches the A000-BFFF RAM area to the bank selected
by the 2-bit secondary banking register.

For "large ROM" carts, mode 1 has the 4000-7FFF banked ROM area behave the
same as mode 0, but additionally the "unbankable" "bank 0" area 0000-3FFF
is now also affected by the 2-bit secondary banking register, meaning it
can now be switched between banks 00h, 20h, 40h, and 60h. These banks are
inaccessible in mode 0 - they cannot be mapped to the 4000-7FFF banked ROM
area.

### Note for 1 MB Multi-Game Compilation Carts

Also known as MBC1m, these carts have an alternative wiring, that ignores
the top bit of the main ROM banking register (making it a 4 bit register)
and applies the 2 bit register to bits 4-5 of the bank number (instead of
the usual bits 5-6). This means that in mode 1 the 2 bit register selects
banks 00h, 10h, 20h, or 30h, rather than the usual 00h, 20h, 40h or 60h.

These carts make use of the fact that mode 1 remaps the 0000-3FFF area
to switch games. The 2 bit register is used to select the game - switching
the zero bank and the region of banks that the 4000-7FFF rom area can
access to those for the selected game, and then the game only changes the
main ROM banking register. As far as the selected game knows, it's running
from a 256 kB cart!

These carts can normally be identified by having a Nintendo copyright
header in bank 0x10. A badly dumped multi-cart ROM can be identified by
having duplicate content in banks 10-1Fh (dupe of 00-0Fh) and banks 30-3Fh
(dupe of 20-2Fh).
There is a known bad dump of the Mortal Kombat I & II collection around.

An "MBC1M" compilation cart ROM can be converted into a regular MBC1 ROM
by increasing the ROM size to 2MB, and duplicating each sub-rom - 00-0Fh
duplicated into 10-1Fh, the original 10-1Fh placed in 20-2Fh and
duplicated into 30-3Fh, and so on.
