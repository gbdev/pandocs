(max 2MByte ROM and/or 32KByte RAM)

This is the first MBC chip for the Game Boy. Any newer MBC chips are
working similiar, so that is relative easy to upgrade a program from one
MBC chip to another - or even to make it compatible to several different
types of MBCs.

Note that the memory in range 0000-7FFF is used for both reading from
ROM, and for writing to the MBCs Control Registers.

### 0000-3FFF - ROM Bank 00/20/40/60 (Read Only)

This area normally contains the first 16KBytes (bank 00) of the cartridge
ROM. Can contain banks 20/40/60 in mode 1 (see below), or banks 10/20/30
in mode 1 for a 1MB MBC1 multi-cart.

### 4000-7FFF - ROM Bank 01-7F (Read Only)

This area may contain any of the further 16KByte banks of the ROM. Cannot
address any banks where the main ROM banking register would be 0, which
usually means banks 00/20/40/60. Instead, it automatically maps to 1 bank
higher (01/21/41/61).

### A000-BFFF - RAM Bank 00-03, if any (Read/Write)

This area is used to address external RAM in the cartridge (if any).
External RAM is often battery buffered, allowing to store game positions
or high score tables, even if the Game Boy is turned off, or if the
cartridge is removed from the Game Boy. Available RAM sizes are: 2KByte
(at A000-A7FF), 8KByte (at A000-BFFF), and 32KByte (in form of four 8K
banks at A000-BFFF).

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
is powered off.

### 2000-3FFF - ROM Bank Number (Write Only)

Writing to this address space selects the lower 5 bits of the ROM Bank
Number (in range 01-1Fh). When 00h is written, the MBC translates that
to bank 01h also.

Not being able o select bank 00 isn't normally a problem,because it's
usually mapped to the 0000-3FFF range. But on large carts (using the
register below to specify the upper ROM Bank bits), the same happens for
Bank 20h, 40h, and 60h. Any attempt to address these ROM Banks will
select Bank 21h, 41h, and 61h instead. The only way to access banks 20h,
40h or 60h is to enter mode 1, which remaps the 0000-3FFF range. This
has its own problems for game developers as that range contains interrupt
handlers, so it's mostly only used in multi-game compilation carts.

### 4000-5FFF - RAM Bank Number - or - Upper Bits of ROM Bank Number (Write Only)

This 2 bit register can be used to select a RAM Bank in range from
00-03h (32 kB ram carts only), or to specify the upper two bits (Bit 5-6)
of the ROM Bank number (1 MB ROM or larger carts only). If neither ROM nor
RAM is large enough, this does nothing.

In 1MB MBC1 multi-carts, this 2 bit register is instead applied to bits
4-5 of the ROM bank number, and the top bit of the main bank regiser is
ignored.

### 6000-7FFF - Banking Mode Select (Write Only)

This 1bit Register selects between the two MBC1 banking modes, controlling
the behaviour of the extended 2 bit banking register (above). If the cart
is not large enough to use the 2 bit register (<= 8kB RAM / <= 512 kB ROM)
this mode select has no observable effect. The program may freely switch
between the two modes at any time.

```
00h = Simple ROM Banking Mode (default)
01h = RAM Banking Mode / Advanced ROM Banking Mode
```

In mode 0, the two bit register can only affect the 4000-7FFF banked ROM
area. If the cart is a "large ram" cart (> 8 kB RAM) then the practical
effect of this is that RAM banking is disabled and A000-BFFF is locked to
only be able to access bank 0 of RAM.

In mode 1, the behaviour differs depending on whether the current cart is
a"large RAM" cart or "large ROM" cart (1 MB or larger). For large RAM carts,
switching to mode 1 enables RAM banking and (if RAM is enabled) immediately
switches the A000-BFFF RAM area to the bank selected by the 2 bit banking
register.

For "large ROM" carts, mode 1 has the 4000-7FFF banked ROM area behave the
same as mode 0, but additionally the "unbankable" "bank 0" area 0000-3FFF
is now affected by the 2 bit upper banking register, meaning it can now be
switched between banks 00h, 20h, 40h, and 60h.

### Note for 1 MB Multi-Game Compilation Carts

Also known as MBC1m, these carts have an alternative wiring, that ignores
the top bit of the main ROM banking register (making it a 4 bit register)
and applies the 2 bit register to bits 4-5 of the bank number (instead of
the usual bits 5-6). This means that in mode 1 the 2 bit register selects
banks 00h, 10h, 20h, or 30h, rather than the usual 00h, 20h, 40h or 60h.

These carts make use of the fact that mode 1 switches the 0000-3FFF area
to switch games. The 2 bit register is used to select the game - switching
the zero bank and the region of banks that the 4000-7FFF rom area can
access to those for the selected game, and then the game only changes the
main ROM banking register. As far as the selected game knows, it's running
from a 256 kB cart!

These carts can normally be identified by having a Nintendo copyright header
in bank 0x10. A badly dumped multi-cart ROM can be identified by having
duplicate content in banks 0x10-0x1F (dupe of 0x00-0x0F) and banks 0x30-0x3F
(dupe of 0x20-0x2F).
