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