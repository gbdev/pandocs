Game Shark and Gamegenie are external cartridge adapters that can be
plugged in between the Game Boy and the actual game cartridge.

### Gamegenie (ROM patches)

Gamegenie codes consist of nine-digit hex numbers, formatted as
`ABC-DEF-GHI`, the meaning of the separate digits is:


- `AB`, new data
- `FCDE`, memory address, XORed by 0F000h
- `GI`, old data, XORed by 0BAh and rotated left by two
- `H`, Unknown, maybe checksum and/or else


The address should be located in ROM area 0000h-7FFFh, the adapter
permanently compares address/old data with address/data being read by
the game, and replaces that data by new data if necessary. That method
(more or less) prohibits unwanted patching of wrong memory banks.
Eventually it is also possible to patch external RAM ? Newer devices
reportedly allow to specify only the first six digits (optionally). 
Three codes can be used at once.

Check the [Game Genie manual](http://www.digitpress.com/library/manuals/gameboy/game%20genie.pdf) for reference.

### Game Shark (RAM patches)

Game Shark codes consist of eight-digit hex numbers, formatted as
ABCDEFGH, the meaning of the separate digits is:

` AB    External RAM bank number`
` CD    New Data`
` GHEF  Memory Address (internal or external RAM, A000-DFFF)`

As far as I understand, patching is implement by hooking the original
VBlank interrupt handler, and re-writing RAM values each frame. The
downside is that this method steals some CPU time, also, it cannot be
used to patch program code in ROM. As far as I rememeber, somewhat 10-25
codes can be used simultaneously.

