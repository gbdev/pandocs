# MBCs

As the Game Boy's 16-bit address bus offers only limited space for
ROM and RAM addressing, many games are using Memory Bank Controllers
(MBCs) to expand the available address space by bank switching.
These MBC chips are located in the game cartridge (that is, not in
the Game Boy itself).

In each cartridge, the required (or preferred) MBC type should be
specified in the byte at $0147 of the ROM, as described
[in the cartridge header](<#0147 — Cartridge type>).  Several MBC
types are available:

## MBC Timing Issues

Among Nintendo MBCs, only the MBC5 is guaranteed by Nintendo to support
the tighter timing of CGB Double Speed Mode. There have been rumours
that older MBCs (like MBC1-3) wouldn't be fast enough in that mode. If
so, it might be nevertheless possible to use Double Speed during periods
which use only code and data which is located in internal RAM. Despite the 
above, a self-made MBC1-EPROM card appears to work stable and fine even in 
Double Speed Mode.

## MBC Unmapped RAM Bank Access

In most MBCs, if an unmapped RAM bank is selected (which would be translate to an out of bounds RAM address by the MBC controller), 
the MBC will simply wrap around the internal ram address and would access a valid RAM address.

The MBC internal address being accessed can be calculated using this formula: `((address - external_ram_start_address) + (active_ram_bank * ram_bank_size)) % max_external_ram_size`.

## How mappers work

This section explains how MBCs (and mappers in general) work, why they are able to do what they do, and the causes of some of their quirks.

### Cartridge board

First, we must introduce how the Game Boy interacts with the components in the cartridge.

Interestingly, a cartridge is pretty much a circuit board that directly extends the Game Boy; contrast this with a CD, for example, which is merely a passive disc that stores data.
This is why CD-based consoles always use separate media to store save files, but cartridges can[^cheap] store save data themselves, or even embark more exotic features, such as [a whole-ass processor][SuperFX].

The Game Boy provides a cartridge several interface pins, each carrying a 0 or a 1[^vin]; here, we will be interested in the 16 address pins ("address bus"), 8 data pins ("data bus"), and the "Read" and "Write" signals.
In the simplest configuration (ROM-only cartridges), the 16 address bus "lanes" are connected directly to the ROM's address pins (with the 16th serving as an "enable" signal rather than an address bit), and as well as the data bus and Read signal[^read-only]:

{{#include imgs/src/mappers/rom-only.svg}}

A **mapper** is a chip within a cartridge that is designed to allow more memory to be accessed than just the base 15-bit address space.[^extras]
They provide extra address lines to the ROM chip based on the ROM bank number that they store, as in this very simplistic example:

{{#include imgs/src/mappers/wisdom-treeish.svg}}

(The above diagram actually describes the [Wisdom Tree] mapper, save for one wiring difference that would make the diagram more confusing.)
Actual MBCs exhibit more complex behaviour, but the principle of "supplying extra address lines to the ROM chip" remains core to their operation.

## Address and data masking

The above diagram also suffices to explain a common behaviour of mappers, including all MBCs: their "control registers" are usually mapped to *ranges* of addresses, unlike [the Game Boy's own](<#Hardware Registers>) which are mapped to a single address each; and, often, several data bits get ignored.

The reason for this is very simple: only a *subset* of the address and/or data lines are connected to the chip\![^hwreg-subset]
Thus, for example, from the point of the above mapper, the values `$00` and `$08` on the data bus are equivalent: the mapper only sees the low 3 bits, which are `%000` in both cases.
Likewise, it accepts writes made to *any* ROM address, since it only checks bit 15 of the address for whether it's concerned at all.[^ext-bus]

To take the example of [MBC3's ROM bank switch], we describe it as the address range \$2000-3FFF, but what the chip really checks is that the top 3 address bits are `001`.
Examining all addresses in the range and outside of it show that these conditions are equivalent.

{{#bits 16 >
	"$2000" 15:"0" 14:"0" 13:"1" 12:"0" 11:"0" 10:"0" 9:"0" 8:"0" 7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2:"0" 1:"0" 0:"0";
	"$3FFF" 15:"0" 14:"0" 13:"1" 12:"1" 11:"1" 10:"1" 9:"1" 8:"1" 7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"1" 1:"1" 0:"1";
	"Pin name" 15:"A15" 14:"A14" 13:"A13" 12-0:"etc."
}}

Different MBC chips have a different set of inputs.
For example, MBC5 has the ROM bank 9<sup>th</sup> bit register in the range `3000-3FFF`.
It thus needs to have one additional address bit available compared to MBC3's `A13`–`A15`, to be able to distinguish `3000-3FFF` from `2000-2FFF`.
The check then becomes whether the top 4 bits are `0010` for the range `2000-2FFF` or `0011` for the range `3000-3FFF`:

{{#bits 16 >
	"$2000" 15:"0" 14:"0" 13:"1" 12:"0" 11:"0" 10:"0" 9:"0" 8:"0" 7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2:"0" 1:"0" 0:"0";
	"$2FFF" 15:"0" 14:"0" 13:"1" 12:"0" 11:"1" 10:"1" 9:"1" 8:"1" 7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"1" 1:"1" 0:"1";
	"$3000" 15:"0" 14:"0" 13:"1" 12:"1" 11:"0" 10:"0" 9:"0" 8:"0" 7:"0" 6:"0" 5:"0" 4:"0" 3:"0" 2:"0" 1:"0" 0:"0";
	"$3FFF" 15:"0" 14:"0" 13:"1" 12:"1" 11:"1" 10:"1" 9:"1" 8:"1" 7:"1" 6:"1" 5:"1" 4:"1" 3:"1" 2:"1" 1:"1" 0:"1";
}}

As for why not all cartridge bus pins are connected to the mappers: cost!
The more pins must be connected to a chip, the more expensive the chip's "package" is, *and* the more difficult the job of "routing" all of the connections on the board.

## MBC2's quirks

[MBC2] is known to be an oddball among MBCs: unusual register addresses, only storing 1 nibble (4 bits, or a half byte) per addressable byte of save RAM...
We can explain *why* it came to be this way by trying to design its functionality ourselves while following the same cost-optimisation constraints.

First the data lines:

- We want to address 256 KiB of ROM, which divides into 16 ROM banks of 16 KiB each.
  The 16 ROM bank IDs all fit in just 4 data bits, thus we might consider connecting only 4 data bits from the cartridge bus.
- To save cost by not requiring an external RAM chip for saving, the MBC2 chip itself should include some persistent RAM.
  You could let the RAM store a full byte at each address; but following the hint from the previous point and only connecting 4 data bits would lose half of the addressable memory.

  However, adding one more address line doubles the number of addressable positions and get 512 nibbles (half bytes) instead of 256 bytes.
  This removes 4 data signals and adds 1 address signal, which effectively removes the need for 3 pins on the chip, at the cost of some extra software complexity.

Then the address lines:

- The chip needs to decode whether the current memory access is for ROM, so definitely needs A15[^romsel].
- It also needs to know if the request is for the fixed bank (\$0000-3FFF) or the selectable bank (\$4000-7FFF).
  A14 is definitely required for this, and it is also useful for decoding the SRAM (\$A000-BFFF) area together with the `/RAMCS` signal from the CPU[^ramcs].
- The save RAM is inside the MBC2 chip, so for 512 addressable (half) bytes, you need the 9 lowest address lines, A0-A8.

All of the above so far is absolutely necessary.
We also want to support writing to \$0000-1FFF to enable/protect the save RAM, and \$2000-3FFF to select the ROM bank.
At this point, we would really want to add A13 as well, to use the top 3 address bits for decoding those ranges cleanly like in the MBC3 example above; however, all of the above pins plus some extras (power supply, clocks, outputs, etc.), already reaches the target of 28 pins on the chip package that Nintendo wanted to go with at the time.

Instead, the designers reused an address line that was already required for addressing the built-in SRAM, `A8`, to determine whether a write goes to the SRAM protect register or the ROM bank select register:

- If a write to an address has the top two bits set to `00` (\$0000-3FFF), then...
  - If A8 is 0, the write goes to the save RAM enable/protect register.
    This means addresses \$0000-00FF, \$0200-02FF, \$0040-04FF... all go to that register.
  - If A8 is 1, the write goes to the save ROM bank select register.
    This means addresses \$0100-01FF, \$0300-03FF, \$0050-05FF... all go to that register.

This is why the two registers accept writes in 256 byte alternating "stripes" of memory.

[^cheap]: Some cartridge-based systems nonetheless externalise saving (e.g. N64 games using a Controller Pak, or all Switch cartridges), usually for cost reasons: adding a chip to store saves and another to dispatch between that and the ROM, is expensive.
[^vin]: Except [the `VIN` pin], which provides analog audio directly to the audio mixing stage.
[^read-only]: Thus, writing is fully ignored, since that signal is not wired up *at all*.
[^extras]: Though, since the cartridge now contains an extra chip, it's also possible for the mapper to provide extra functionality; this is why, for example, rumble is controlled [by writes to the MBC5](#Rumble).
[^hwreg-subset]: The "data bus" portion of this explanation also applies to the GB's hardware registers and why some of their bits are unwritable.
                 The "address bus" portion is also the reason why Echo RAM ends up mirroring WRAM: the WRAM chip sees the same (partial) addresses, and thus responds identically. 
[^ext-bus]: Checking for bit 15 is necessary because the Game Boy puts *some* addresses outside of \$0000–7FFF on the cartridge bus: by design, \$A000–BFFF are also forwarded to the cartridge, but [Echo RAM] also is (\$FE00–FFFF are intentionally kept internal-only).
            Accessing Echo RAM makes some cheap cartridges actually assume, due to not connecting enough pins or omitting the necessary logic, that SRAM is being accessed, and thus they drive the data bus at the same time as WRAM does, creating bus conflicts.
[^romsel]: A15 (again, the 16th address bit) is sometimes referred to as `ROMSEL` since it perfectly maps to whether the ROM chip should respond.
[^ramcs]: `/RAMCS` was omitted from this page's diagrams for simplicity.
          They do not aim to mirror the actual cartridge pinout.

[SuperFX]: https://en.wikipedia.org/wiki/Super_FX
[Wisdom Tree]: <#Wisdom Tree>
[MBC3's ROM bank switch]: <MBC3.md#2000-3FFF - ROM Bank Number (Write Only)>
[MBC2]: #MBC2
[the `VIN` pin]: <#FF24 — NR50: Master volume & VIN panning>
[Echo RAM]: <#Echo RAM>
