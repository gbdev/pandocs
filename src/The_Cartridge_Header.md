# The Cartridge Header

Each cartridge contains a header, located at the address range `$0100`—`$014F`.
The cartridge header provides the following information about the game itself and the hardware it expects to run on:

## 0100-0103 — Entry point

After displaying the Nintendo logo, the built-in [boot ROM](<#Power-Up Sequence>) jumps to the address `$0100`, which should then jump to the actual main program in the cartridge.
Most commercial games fill this 4-byte area with a [`nop` instruction](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7/#NOP) followed by a [`jp $0150`](https://rgbds.gbdev.io/docs/v0.5.2/gbz80.7/#JP_n16).

## 0104-0133 — Nintendo logo

This area contains a bitmap image that is displayed when the Game Boy is powered on.
It must match the following (hexadecimal) dump, otherwise [the boot ROM](<#Power-Up Sequence>) won't allow the game to run:

```
CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
```

The way the pixels are encoded is as follows: ([more visual aid](https://github.com/ISSOtm/gb-bootroms/blob/2dce25910043ce2ad1d1d3691436f2c7aabbda00/src/dmg.asm#L259-L269))

- The bytes `$0104`—`$011B` encode the top half of the logo while the bytes `$011C`–`$0133` encode the bottom half.
- For each half, each nibble encodes 4 pixels (the MSB corresponds to the leftmost pixel, the LSB to the rightmost); a pixel is lit if the corresponding bit is set.
- The 4-pixel "groups" are laid out top to bottom, left to right.
- Finally, the monochrome models upscale the entire thing by a factor of 2 (leading to somewhat chunky pixels).

The Game Boy's boot procedure [first displays the logo and then checks](<#Bypass>) that it matches the dump above.
If it doesn't, the boot ROM **locks itself up**.

The CGB and later models [only check the top half of the logo](Power_Up_Sequence.html?highlight=half#behavior) (the first `$18` bytes).

## 0134-0143 — Title

These bytes contain the title of the game in upper case ASCII.
If the title is less than 16 characters long, the remaining bytes should be padded with `$00`s.

Parts of this area actually have a different meaning on later cartridges, reducing the actual title size to 15 (`$0134`–`$0142`) or 11 (`$0134`–`$013E`) characters; see below.

## 013F-0142 — Manufacturer code

In older cartridges these bytes were part of the Title (see above).
In newer cartridges they contain a 4-character manufacturer code (in uppercase ASCII).
The purpose of the manufacturer code is unknown.

## 0143 — CGB flag

In older cartridges this byte was part of the Title (see above).
The CGB and later models interpret this byte to decide whether to enable Color mode ("CGB Mode") or to fall back to monochrome compatibility mode ("Non-CGB Mode").

Typical values are:

Value | Meaning
------|----------------------------------------------------------------------------------------------------
`$80` | The game supports CGB enhancements, but is backwards compatible with monochrome Game Boys
`$C0` | The game works on CGB only (the hardware ignores bit 6, so this really functions the same as `$80`)

Values with bit 7 and either bit 2 or 3 set will switch the Game Boy into a special non-CGB-mode called "PGB mode".

:::tip Research needed

The PGB mode is not well researched or documented yet.
Help is welcome!

:::

## 0144–0145 — New licensee code

This area contains a two-character ASCII "licensee code" indicating the game's publisher.
It is only meaningful if the [Old licensee](<#014B — Old licensee code>) is exactly `$33` (which is the case for essentially all games made after the SGB was released); otherwise, the old code must be considered.

Sample licensee codes:

Code | Publisher
-----|-------------------------
`00` | None
`01` | Nintendo R&D1
`08` | Capcom
`13` | Electronic Arts
`18` | Hudson Soft
`19` | b-ai
`20` | kss
`22` | pow
`24` | PCM Complete
`25` | san-x
`28` | Kemco Japan
`29` | seta
`30` | Viacom
`31` | Nintendo
`32` | Bandai
`33` | Ocean/Acclaim
`34` | Konami
`35` | Hector
`37` | Taito
`38` | Hudson
`39` | Banpresto
`41` | Ubi Soft
`42` | Atlus
`44` | Malibu
`46` | angel
`47` | Bullet-Proof
`49` | irem
`50` | Absolute
`51` | Acclaim
`52` | Activision
`53` | American sammy
`54` | Konami
`55` | Hi tech entertainment
`56` | LJN
`57` | Matchbox
`58` | Mattel
`59` | Milton Bradley
`60` | Titus
`61` | Virgin
`64` | LucasArts
`67` | Ocean
`69` | Electronic Arts
`70` | Infogrames
`71` | Interplay
`72` | Broderbund
`73` | sculptured
`75` | sci
`78` | THQ
`79` | Accolade
`80` | misawa
`83` | lozc
`86` | Tokuma Shoten Intermedia
`87` | Tsukuda Original
`91` | Chunsoft
`92` | Video system
`93` | Ocean/Acclaim
`95` | Varie
`96` | Yonezawa/s'pal
`97` | Kaneko
`99` | Pack in soft
`9H` | Bottom Up
`A4` | Konami (Yu-Gi-Oh!)

## 0146 — SGB flag

This byte specifies whether the game supports SGB functions.
The SGB will ignore any [command packets](<#Command Packet Transfers>) if this byte is set to a value other than `$03` (typically `$00`).

## 0147 — Cartridge type

This byte indicates what kind of hardware is present on the cartridge — most notably its [mapper](#MBCs).

Code  | Type
------|--------------------------------
`$00` | ROM ONLY
`$01` | MBC1
`$02` | MBC1+RAM
`$03` | MBC1+RAM+BATTERY
`$05` | MBC2
`$06` | MBC2+BATTERY
`$08` | ROM+RAM [^rom_ram]
`$09` | ROM+RAM+BATTERY [^rom_ram]
`$0B` | MMM01
`$0C` | MMM01+RAM
`$0D` | MMM01+RAM+BATTERY
`$0F` | MBC3+TIMER+BATTERY
`$10` | MBC3+TIMER+RAM+BATTERY [^mbc30]
`$11` | MBC3
`$12` | MBC3+RAM [^mbc30]
`$13` | MBC3+RAM+BATTERY [^mbc30]
`$19` | MBC5
`$1A` | MBC5+RAM
`$1B` | MBC5+RAM+BATTERY
`$1C` | MBC5+RUMBLE
`$1D` | MBC5+RUMBLE+RAM
`$1E` | MBC5+RUMBLE+RAM+BATTERY
`$20` | MBC6
`$22` | MBC7+SENSOR+RUMBLE+RAM+BATTERY
`$FC` | POCKET CAMERA
`$FD` | BANDAI TAMA5
`$FE` | HuC3
`$FF` | HuC1+RAM+BATTERY

[^rom_ram]:
No licensed cartridge makes use of this option. The exact behavior is unknown.

[^mbc30]:
MBC3 with 64 KiB of SRAM refers to MBC30, used only in _Pocket Monsters: Crystal Version_ (the Japanese version of _Pokémon Crystal Version_).

## 0148 — ROM size

This byte indicates how much ROM is present on the cartridge.
In most cases, the ROM size is given by `32 KiB × (1 << <value>)`:

Value | ROM size  | Number of ROM banks
------|-----------|----------------------
`$00` |  32 KiB   | 2 (no banking)
`$01` |  64 KiB   | 4
`$02` | 128 KiB   | 8
`$03` | 256 KiB   | 16
`$04` | 512 KiB   | 32
`$05` |   1 MiB   | 64
`$06` |   2 MiB   | 128
`$07` |   4 MiB   | 256
`$08` |   8 MiB   | 512
`$52` | 1.1 MiB   | 72 [^weird_rom_sizes]
`$53` | 1.2 MiB   | 80 [^weird_rom_sizes]
`$54` | 1.5 MiB   | 96 [^weird_rom_sizes]

[^weird_rom_sizes]:
Only listed in unofficial docs. No cartridges or ROM files using these sizes are known.
As the other ROM sizes are all powers of 2, these are likely inaccurate.
The source of these values is unknown.

## 0149 — RAM size

This byte indicates how much RAM is present on the cartridge, if any.

If the [cartridge type](<#0147 — Cartridge type>) does not include "RAM" in its name, this should be set to 0.
This includes MBC2, since its 512 × 4 bits of memory are built directly into the mapper.

Code  | SRAM size | Comment
------|-----------|-----------------------
`$00` |   0       | No RAM
`$01` |   –       | Unused [^2kib_sram]
`$02` |   8 KiB   |  1 bank
`$03` |  32 KiB   |  4 banks of 8 KiB each
`$04` | 128 KiB   | 16 banks of 8 KiB each
`$05` |  64 KiB   |  8 banks of 8 KiB each

[^2kib_sram]:
Listed in various unofficial docs as 2 KiB.
However, a 2 KiB RAM chip was never used in a cartridge.
The source of this value is unknown.

Various "PD" ROMs ("Public Domain" homebrew ROMs, generally tagged with `(PD)` in the filename) are known to use the `$01` RAM Size tag, but this is believed to have been a mistake with early homebrew tools, and the PD ROMs often don't use cartridge RAM at all.

## 014A — Destination code

This byte specifies whether this version of the game is intended to be sold in Japan or elsewhere.

Only two values are defined:

Code  | Destination
------|------------------------------
`$00` | Japan (and possibly overseas)
`$01` | Overseas only

## 014B — Old licensee code

This byte is used in older (pre-SGB) cartridges to specify the game's publisher.
However, the value `$33` indicates that the [New licensee codes](<#0144–0145 — New licensee code>) must be considered instead.
(The SGB will ignore any [command packets](<#Command Packet Transfers>) unless this value is `$33`.)

Here is a list of known Old licensee codes ([source](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt)).

HEX   | Licensee
------|------------
00    | None               
01    | Nintendo           
08    | Capcom
09    | Hot-B              
0A    | Jaleco             
0B    | Coconuts Japan
0C    | Elite Systems      
13    | EA (Electronic Arts)    
18    | Hudsonsoft
19    | ITC Entertainment  
1A    | Yanoman            
1D    | Japan Clary
1F    | Virgin Interactive             
24    | PCM Complete       
25    | San-X
28    | Kotobuki Systems   
29    | Seta               
30    | Infogrames
31    | Nintendo           
32    | Bandai             
33    | Indicates that the [New licensee code](<#0144–0145 — New licensee code>) should be used instead.
34    | Konami             
35    | HectorSoft            
38    | Capcom
39    | Banpresto          
3C    | .Entertainment i   
3E    | Gremlin
41    | Ubisoft           
42    | Atlus              
44    | Malibu
46    | Angel              
47    | Spectrum Holoby    
49    | Irem
4A    | Virgin Interactive            
4D    | Malibu             
4F    | U.S. Gold
50    | Absolute           
51    | Acclaim            
52    | Activision
53    | American Sammy     
54    | GameTek           
55    | Park Place
56    | LJN                
57    | Matchbox           
59    | Milton Bradley
5A    | Mindscape          
5B    | Romstar            
5C    | Naxat Soft
5D    | Tradewest          
60    | Titus              
61    | Virgin Interactive
67    | Ocean Interactive              
69    | EA (Electronic Arts)    
6E    | Elite Systems
6F    | Electro Brain      
70    | Infogrames         
71    | Interplay
72    | Broderbund         
73    | Sculptered Soft   
75    | The Sales Curve
78    | t.hq               
79    | Accolade           
7A    | Triffix Entertainment
7C    | Microprose         
7F    | Kemco              
80    | Misawa Entertainment
83    | Lozc               
86    | Tokuma Shoten Intermedia   
8B    | Bullet-Proof Software
8C    | Vic Tokai          
8E    | Ape                
8F    | I'Max
91    | Chunsoft Co.         
92    | Video System       
93    | Tsubaraya Productions Co.
95    | Varie Corporation              
96    | Yonezawa/S'Pal     
97    | Kaneko
99    | Arc                
9A    | Nihon Bussan       
9B    | Tecmo
9C    | Imagineer          
9D    | Banpresto          
9F    | Nova
A1    | Hori Electric      
A2    | Bandai             
A4    | Konami
A6    | Kawada             
A7    | Takara             
A9    | Technos Japan
AA    | Broderbund         
AC    | Toei Animation    
AD    | Toho
AF    | Namco             
B0    | acclaim           
B1    | ASCII or Nexsoft
B2    | Bandai             
B4    | Square Enix               
B6    | HAL Laboratory
B7    | SNK                
B9    | Pony Canyon        
BA    | Culture Brain
BB    | Sunsoft            
BD    | Sony Imagesoft    
BF    | Sammy
C0    | Taito             
C2    | Kemco              
C3    | Squaresoft
C4    | Tokuma Shoten Intermedia   
C5    | Data East          
C6    | Tonkinhouse
C8    | Koei               
C9    | UFL                
CA    | Ultra
CB    | Vap                
CC    | Use Corporation               
CD    | Meldac
CE    | .Pony Canyon or    
CF    | Angel              
D0    | Taito
D1    | Sofel              
D2    | Quest              
D3    | Sigma Enterprises
D4    | ASK Kodansha Co.       
D6    | Naxat Soft         
D7    | Copya System
D9    | Banpresto          
DA    | Tomy               
DB    | LJN
DD    | NCS                
DE    | Human              
DF    | Altron
E0    | Jaleco             
E1    | Towa Chiki          
E2    | Yutaka
E3    | Varie              
E5    | Epcoh              
E7    | Athena
E8    | Asmik ACE Entertainment             
E9    | Natsume            
EA    | King Records
EB    | Atlus              
EC    | Epic/Sony Records  
EE    | IGS
F0    | A Wave         
F3    | Extreme Entertainment  
FF    | LJN

## 014C — Mask ROM version number

This byte specifies the version number of the game.
It is usually `$00`.

## 014D — Header checksum

This byte contains an 8-bit checksum computed from the cartridge header bytes $0134–014C.
The boot ROM computes the checksum as follows:

```c
uint8_t checksum = 0;
for (uint16_t address = 0x0134; address <= 0x014C; address++) {
    checksum = checksum - rom[address] - 1;
}
```

The boot ROM verifies this checksum.
If the byte at `$014D` does not match the lower 8 bits of `checksum`, the boot ROM will lock up and the program in the
cartridge **won't run**.

## 014E-014F — Global checksum

These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of
all the bytes of the cartridge ROM (except these two checksum bytes).

This checksum is not verified, except by Pokémon Stadium's "GB Tower" emulator (presumably to detect Transfer Pak errors).
