Game Shark and Game Genie are external cartridge adapters that can be
plugged in between the Game Boy and the actual game cartridge.

## Game Genie (ROM patches)

Game Genie codes consist of nine-digit hex numbers, formatted as
`ABC-DEF-GHI`, the meaning of the separate digits is:


- `AB`, new data
- `FCDE`, memory address, XORed by $F000
- `GI`, old data, XORed by $BA and rotated left by two
- `H`, Unknown, maybe checksum and/or else


The address should be located in ROM area $0000-7FFF, the adapter
permanently compares address/old data with address/data being read by
the game, and replaces that data by new data if necessary. That method
(more or less) prohibits unwanted patching of wrong memory banks.
Eventually it is also possible to patch external RAM ? Newer devices
reportedly allow to specify only the first six digits (optionally). 
Three codes can be used at once.

Check the [Game Genie manual](http://www.digitpress.com/library/manuals/gameboy/game%20genie.pdf) for reference.

## Game Shark (RAM patches)

Game Shark codes consist of eight hexadecimal digits, with the following meaning:

{{#bits 8 <
  "" 0-1:"SRAM bank" 2-3:"New value" 4-7:"Address"
}}

So, for example, cheat code `010238CD` switches to SRAM bank $01, and writes $02 at address $CD38.

As far as it is understood, patching is implemented by hooking the original
VBlank interrupt handler, and re-writing RAM values each frame. The
downside is that this method steals some CPU time, also, it cannot be
used to patch program code in ROM. 10-25 codes can be used simultaneously.

