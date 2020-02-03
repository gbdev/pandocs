When the Game Boy is powered up, a 256 byte program starting at memory location 0 is executed. This program is located in a ROM inside the GameBoy. The first thing the program does is read the cartridge locations from $104 to $133 and place this graphic of a Nintendo logo on the screen at the top.

This image is then scrolled until it is in the middle of the screen. Two musical notes are then played on the internal speaker. Again, the cartridge locations $104 to $133 are read but this time they are compared with a table in the internal rom.

If any byte fails to compare, then the Game Boy stops comparing bytes and simply halts all operations.

If all locations compare the same, then the GameBoy starts adding all of the bytes in the cartridge from $134 to $14d. A value of 25 decimal is added to this total.

If the least significant byte of the result is a not a zero, then the Game Boy will stop doing anything.

If it is a zero, then the internal ROM is disabled and cartridge program execution begins at location $100 with the following register values:

```
  AF=$01B0
  BC=$0013
  DE=$00D8
  HL=$014D
  Stack Pointer=$FFFE
  [$FF05] = $00   ; TIMA
  [$FF06] = $00   ; TMA
  [$FF07] = $00   ; TAC
  [$FF10] = $80   ; NR10
  [$FF11] = $BF   ; NR11
  [$FF12] = $F3   ; NR12
  [$FF14] = $BF   ; NR14
  [$FF16] = $3F   ; NR21
  [$FF17] = $00   ; NR22
  [$FF19] = $BF   ; NR24
  [$FF1A] = $7F   ; NR30
  [$FF1B] = $FF   ; NR31
  [$FF1C] = $9F   ; NR32
  [$FF1E] = $BF   ; NR33
  [$FF20] = $FF   ; NR41
  [$FF21] = $00   ; NR42
  [$FF22] = $00   ; NR43
  [$FF23] = $BF   ; NR44
  [$FF24] = $77   ; NR50
  [$FF25] = $F3   ; NR51
  [$FF26] = $F1-GB, $F0-SGB ; NR52
  [$FF40] = $91   ; LCDC
  [$FF42] = $00   ; SCY
  [$FF43] = $00   ; SCX
  [$FF45] = $00   ; LYC
  [$FF47] = $FC   ; BGP
  [$FF48] = $FF   ; OBP0
  [$FF49] = $FF   ; OBP1
  [$FF4A] = $00   ; WY
  [$FF4B] = $00   ; WX
  [$FFFF] = $00   ; IE
```

It is not a good idea to assume the above values will always exist. A later version Game Boy could contain different values than these at reset. Always set these registers on reset rather than assume they are as above.

Please note that Game Boy internal RAM on power up contains random data.

All of the Game Boy emulators tend to set all RAM to value $00 on entry.

Cart RAM the first time it is accessed on a real Game Boy contains random data.
It will only contain known data if the Game Boy code initializes it to some value.

