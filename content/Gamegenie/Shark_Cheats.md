Game Shark and Gamegenie are external cartridge adapters that can be
plugged between the Game Boy and the actual game cartridge. Hexadecimal
codes can be then entered for specific games, typically providing things
like Infinite Sex, 255 Cigarettes, or Starting directly in Wonderland
Level PRO, etc.

### Gamegenie (ROM patches)

Gamegenie codes consist of nine-digit hex numbers, formatted as
ABC-DEF-GHI, the meaning of the separate digits is:

` AB    New data`\
` FCDE  Memory address, XORed by 0F000h`\
` GI    Old data, XORed by 0BAh and rotated left by two`\
` H     Don't know, maybe checksum and/or else`

The address should be located in ROM area 0000h-7FFFh, the adapter
permanently compares address/old data with address/data being read by
the game, and replaces that data by new data if necessary. That method
(more or less) prohibits unwanted patching of wrong memory banks.
Eventually it is also possible to patch external RAM ? Newer devices
reportedly allow to specify only the first six digits (optionally). As
far as I rememeber, around three or four codes can be used
simultaneously.

### Game Shark (RAM patches)

Game Shark codes consist of eight-digit hex numbers, formatted as
ABCDEFGH, the meaning of the separate digits is:

` AB    External RAM bank number`\
` CD    New Data`\
` GHEF  Memory Address (internal or external RAM, A000-DFFF)`

As far as I understand, patching is implement by hooking the original
VBlank interrupt handler, and re-writing RAM values each frame. The
downside is that this method steals some CPU time, also, it cannot be
used to patch program code in ROM. As far as I rememeber, somewhat 10-25
codes can be used simultaneously.

