# The Cartridge Header

Each cartridge contains a header, located at the address range `$0100`—`$014F`.
The cartridge header provides the following information about the game itself and the hardware it expects to run on:

## 0100-0103 — Entry point

After displaying the Nintendo logo, the built-in [boot ROM](<#Power-Up Sequence>) jumps to the address `$0100`, which should then jump to the actual main program in the cartridge.
Most commercial games fill this 4-byte area with a [`nop` instruction](https://rgbds.gbdev.io/docs/gbz80.7#NOP) followed by a [`jp $0150`](https://rgbds.gbdev.io/docs/gbz80.7#JP_n16).

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

Setting bit 7 will trigger a write of this register value to [KEY0 register](<#FF4C — KEY0/SYS (CGB Mode only): CPU mode select>) which sets the CPU mode.

## 0144–0145 — New licensee code

This area contains a two-character ASCII "licensee code" indicating the game's publisher.
It is only meaningful if the [Old licensee](<#014B — Old licensee code>) is exactly `$33` (which is the case for essentially all games made after the SGB was released); otherwise, the old code must be considered.

Sample licensee codes:

Code | Publisher
-----|-------------------------
`00` | None
`01` | [Nintendo Research & Development 1](https://en.wikipedia.org/wiki/Nintendo_Research_%26_Development_1)
`08` | [Capcom](https://en.wikipedia.org/wiki/Capcom)
`13` | [EA (Electronic Arts)](https://en.wikipedia.org/wiki/Electronic_Arts)
`18` | [Hudson Soft](https://en.wikipedia.org/wiki/Hudson_Soft)
`19` | [B-AI](https://www.giantbomb.com/b-ai/3010-8160)
`20` | [KSS](https://en.wikipedia.org/wiki/KSS_(company))
`22` | [Planning Office WADA](https://www.mobygames.com/company/15869/planning-office-wada)
`24` | [PCM Complete](https://www.mobygames.com/company/9489/pcm-complete)
`25` | San-X
`28` | [Kemco](https://en.wikipedia.org/wiki/Kemco)
`29` | [SETA Corporation](https://en.wikipedia.org/wiki/SETA_Corporation)
`30` | [Viacom](https://en.wikipedia.org/wiki/Viacom_(1952%E2%80%932005))
`31` | [Nintendo](https://en.wikipedia.org/wiki/Nintendo)
`32` | [Bandai](https://en.wikipedia.org/wiki/Bandai)
`33` | [Ocean Software](https://en.wikipedia.org/wiki/Ocean_Software)/[Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment)
`34` | [Konami](https://en.wikipedia.org/wiki/Konami)
`35` | [HectorSoft](https://www.mobygames.com/company/12239/hectorsoft)
`37` | [Taito](https://en.wikipedia.org/wiki/Taito)
`38` | [Hudson Soft](https://en.wikipedia.org/wiki/Hudson_Soft)
`39` | [Banpresto](https://en.wikipedia.org/wiki/Banpresto)
`41` | [Ubi Soft](https://en.wikipedia.org/wiki/Ubisoft)[^ubisoft]
`42` | [Atlus](https://en.wikipedia.org/wiki/Atlus)
`44` | [Malibu Interactive](https://en.wikipedia.org/wiki/Malibu_Comics)
`46` | [Angel](https://www.mobygames.com/company/5083/angel)
`47` | [Bullet-Proof Software](https://en.wikipedia.org/wiki/Blue_Planet_Software)[^blueplanet]
`49` | [Irem](https://en.wikipedia.org/wiki/Irem)
`50` | Absolute
`51` | [Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment)
`52` | [Activision](https://en.wikipedia.org/wiki/Activision)
`53` | [Sammy USA Corporation](https://en.wikipedia.org/wiki/Sammy_Corporation)
`54` | [Konami](https://en.wikipedia.org/wiki/Konami)
`55` | [Hi Tech Expressions](https://en.wikipedia.org/wiki/Hi_Tech_Expressions)
`56` | [LJN](https://en.wikipedia.org/wiki/LJN)
`57` | [Matchbox](https://en.wikipedia.org/wiki/Matchbox_(brand))
`58` | [Mattel](https://en.wikipedia.org/wiki/Mattel)
`59` | [Milton Bradley Company](https://en.wikipedia.org/wiki/Milton_Bradley_Company)
`60` | [Titus Interactive](https://en.wikipedia.org/wiki/Titus_Interactive)
`61` | [Virgin Games Ltd.](https://en.wikipedia.org/wiki/Virgin_Interactive_Entertainment)[^virgin]
`64` | [Lucasfilm Games](https://en.wikipedia.org/wiki/Lucasfilm_Games)[^lucasfilm]
`67` | [Ocean Software](https://en.wikipedia.org/wiki/Ocean_Software)
`69` | [EA (Electronic Arts)](https://en.wikipedia.org/wiki/Electronic_Arts)
`70` | [Infogrames](https://en.wikipedia.org/wiki/Atari_SA)[^atari]
`71` | [Interplay Entertainment](https://en.wikipedia.org/wiki/Interplay_Entertainment)
`72` | [Broderbund](https://en.wikipedia.org/wiki/Broderbund)
`73` | [Sculptured Software](https://en.wikipedia.org/wiki/Iguana_Entertainment)[^sculptured]
`75` | [The Sales Curve Limited](https://en.wikipedia.org/wiki/SCi_Games)[^sci]
`78` | [THQ](https://en.wikipedia.org/wiki/THQ)
`79` | Accolade
`80` | [Misawa Entertainment](https://www.mobygames.com/company/8225/misawa-entertainment-coltd)
`83` | lozc
`86` | [Tokuma Shoten](https://en.wikipedia.org/wiki/Tokuma_Shoten)
`87` | Tsukuda Original
`91` | [Chunsoft Co.](https://en.wikipedia.org/wiki/Spike_Chunsoft)[^spike]
`92` | Video System
`93` | [Ocean Software](https://en.wikipedia.org/wiki/Ocean_Software)/[Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment)
`95` | [Varie](https://en.wikipedia.org/wiki/Varie)
`96` | Yonezawa/s'pal
`97` | [Kaneko](https://en.wikipedia.org/wiki/Kaneko)
`99` | [Pack-In-Video](https://en.wikipedia.org/wiki/Pack-In-Video)
`9H` | Bottom Up
`A4` | [Konami](https://en.wikipedia.org/wiki/Konami) (Yu-Gi-Oh!)
`BL` | [MTO](https://en.wikipedia.org/wiki/MTO_(video_game_company))
`DK` | [Kodansha](https://en.wikipedia.org/wiki/Kodansha)

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
`00`  | None
`01`  | [Nintendo](https://en.wikipedia.org/wiki/Nintendo)
`08`  | [Capcom](https://en.wikipedia.org/wiki/Capcom)
`09`  | [HOT-B](https://en.wikipedia.org/wiki/Category:Hot_B_games)
`0A`  | [Jaleco](https://en.wikipedia.org/wiki/Jaleco)
`0B`  | [Coconuts Japan](https://en.wikipedia.org/wiki/Category:Coconuts_Japan_games)
`0C`  | [Elite Systems](https://en.wikipedia.org/wiki/Elite_Systems)
`13`  | [EA (Electronic Arts)](https://en.wikipedia.org/wiki/Electronic_Arts)
`18`  | [Hudson Soft](https://en.wikipedia.org/wiki/Hudson_Soft)
`19`  | [ITC Entertainment](https://en.wikipedia.org/wiki/ITC_Entertainment)
`1A`  | [Yanoman](https://en.wikipedia.org/wiki/Category:Yanoman_games)
`1D`  | [Japan Clary](https://www.mobygames.com/company/7639/japan-clary-business/)
`1F`  | [Virgin Games Ltd.](https://en.wikipedia.org/wiki/Virgin_Interactive_Entertainment)[^virgin]
`24`  | [PCM Complete](https://www.mobygames.com/company/9489/pcm-complete)
`25`  | [San-X](https://en.wikipedia.org/wiki/San-X)
`28`  | [Kemco](https://en.wikipedia.org/wiki/Kemco)
`29`  | [SETA Corporation](https://en.wikipedia.org/wiki/SETA_Corporation)
`30`  | [Infogrames](https://en.wikipedia.org/wiki/Atari_SA)[^atari]
`31`  | [Nintendo](https://en.wikipedia.org/wiki/Nintendo)
`32`  | [Bandai](https://en.wikipedia.org/wiki/Bandai)
`33`  | Indicates that the [New licensee code](<#0144–0145 — New licensee code>) should be used instead.
`34`  | [Konami](https://en.wikipedia.org/wiki/Konami)
`35`  | [HectorSoft](https://www.mobygames.com/company/12239/hectorsoft)
`38`  | [Capcom](https://en.wikipedia.org/wiki/Capcom)
`39`  | [Banpresto](https://en.wikipedia.org/wiki/Banpresto)
`3C`  | Entertainment Interactive (stub)
`3E`  | [Gremlin](https://en.wikipedia.org/wiki/Gremlin_Interactive)
`41`  | [Ubi Soft](https://en.wikipedia.org/wiki/Ubisoft)[^ubisoft]
`42`  | [Atlus](https://en.wikipedia.org/wiki/Atlus)
`44`  | [Malibu Interactive](https://en.wikipedia.org/wiki/Malibu_Comics)
`46`  | [Angel](https://www.mobygames.com/company/5083/angel)
`47`  | [Spectrum HoloByte](https://en.wikipedia.org/wiki/Spectrum_HoloByte)
`49`  | [Irem](https://en.wikipedia.org/wiki/Irem)
`4A`  | [Virgin Games Ltd.](https://en.wikipedia.org/wiki/Virgin_Interactive_Entertainment)[^virgin]
`4D`  | [Malibu Interactive](https://en.wikipedia.org/wiki/Malibu_Comics)
`4F`  | [U.S. Gold](https://en.wikipedia.org/wiki/U.S._Gold)
`50`  | [Absolute](https://en.wikipedia.org/wiki/Absolute_Entertainment)
`51`  | [Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment)
`52`  | [Activision](https://en.wikipedia.org/wiki/Activision)
`53`  | [Sammy USA Corporation](https://en.wikipedia.org/wiki/Sammy_Corporation)
`54`  | [GameTek](https://en.wikipedia.org/wiki/GameTek)
`55`  | [Park Place](https://en.wikipedia.org/wiki/Park_Place_Entertainment)[^caesars]
`56`  | [LJN](https://en.wikipedia.org/wiki/LJN)
`57`  | [Matchbox](https://en.wikipedia.org/wiki/Matchbox_(brand))
`59`  | [Milton Bradley Company](https://en.wikipedia.org/wiki/Milton_Bradley_Company)
`5A`  | [Mindscape](https://en.wikipedia.org/wiki/Mindscape_(company))
`5B`  | [Romstar](https://en.wikipedia.org/wiki/Romstar)
`5C`  | [Naxat Soft](https://en.wikipedia.org/wiki/Kaga_Create)[^kaga]
`5D`  | [Tradewest](https://en.wikipedia.org/wiki/Tradewest)
`60`  | [Titus Interactive](https://en.wikipedia.org/wiki/Titus_Interactive)
`61`  | [Virgin Games Ltd.](https://en.wikipedia.org/wiki/Virgin_Interactive_Entertainment)[^virgin]
`67`  | [Ocean Software](https://en.wikipedia.org/wiki/Ocean_Software)
`69`  | [EA (Electronic Arts)](https://en.wikipedia.org/wiki/Electronic_Arts)
`6E`  | [Elite Systems](https://en.wikipedia.org/wiki/Elite_Systems)
`6F`  | [Electro Brain](https://en.wikipedia.org/wiki/Electro_Brain)
`70`  | [Infogrames](https://en.wikipedia.org/wiki/Atari_SA)[^atari]
`71`  | [Interplay Entertainment](https://en.wikipedia.org/wiki/Interplay_Entertainment)
`72`  | [Broderbund](https://en.wikipedia.org/wiki/Broderbund)
`73`  | [Sculptured Software](https://en.wikipedia.org/wiki/Iguana_Entertainment)[^sculptured]
`75`  | [The Sales Curve Limited](https://en.wikipedia.org/wiki/SCi_Games)[^sci]
`78`  | [THQ](https://en.wikipedia.org/wiki/THQ)
`79`  | [Accolade](https://en.wikipedia.org/wiki/Accolade_(company))[^infogrames]
`7A`  | [Triffix Entertainment](https://www.mobygames.com/company/4307/triffix-entertainment-inc)
`7C`  | [MicroProse](https://en.wikipedia.org/wiki/MicroProse)
`7F`  | [Kemco](https://en.wikipedia.org/wiki/Kemco)
`80`  | [Misawa Entertainment](https://www.mobygames.com/company/8225/misawa-entertainment-coltd)
`83`  | [LOZC G.](https://en.wikipedia.org/wiki/Category:LOZC_G._Amusements_games)
`86`  | [Tokuma Shoten](https://en.wikipedia.org/wiki/Tokuma_Shoten)
`8B`  | [Bullet-Proof Software](https://en.wikipedia.org/wiki/Blue_Planet_Software)[^blueplanet]
`8C`  | [Vic Tokai Corp.](https://en.wikipedia.org/wiki/Tokai_Communications)[^tokaicomm]
`8E`  | [Ape Inc.](https://en.wikipedia.org/wiki/Creatures_Inc.)[^creatures]
`8F`  | [I'Max](https://en.wikipedia.org/wiki/I%27MAX)[^imax]
`91`  | [Chunsoft Co.](https://en.wikipedia.org/wiki/Spike_Chunsoft)[^spike]
`92`  | [Video System](https://en.wikipedia.org/wiki/Category:Video_System_games)
`93`  | [Tsubaraya Productions](https://en.wikipedia.org/wiki/Tsuburaya_Productions)
`95`  | [Varie](https://en.wikipedia.org/wiki/Varie)
`96`  | [Yonezawa](https://en.wikipedia.org/wiki/Sega_Fave)[^segabuy]/S'Pal
`97`  | [Kemco](https://en.wikipedia.org/wiki/Kemco)
`99`  | Arc
`9A`  | [Nihon Bussan](https://en.wikipedia.org/wiki/Nihon_Bussan)
`9B`  | [Tecmo](https://en.wikipedia.org/wiki/Tecmo)
`9C`  | [Imagineer](https://en.wikipedia.org/wiki/Imagineer_(Japanese_company))
`9D`  | [Banpresto](https://en.wikipedia.org/wiki/Banpresto)
`9F`  | Nova
`A1`  | [Hori Electric](https://www.mobygames.com/company/8959/hori-electric-co-ltd/)
`A2`  | [Bandai](https://en.wikipedia.org/wiki/Bandai)
`A4`  | [Konami](https://en.wikipedia.org/wiki/Konami)
`A6`  | Kawada
`A7`  | [Takara](https://en.wikipedia.org/wiki/Takara)
`A9`  | [Technos Japan](https://en.wikipedia.org/wiki/Techn%C5%8Ds_Japan)
`AA`  | [Broderbund](https://en.wikipedia.org/wiki/Broderbund)
`AC`  | [Toei Animation](https://en.wikipedia.org/wiki/Toei_Animation)
`AD`  | [Toho](https://en.wikipedia.org/wiki/Toho)
`AF`  | [Namco](https://en.wikipedia.org/wiki/Namco)
`B0`  | [Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment)
`B1`  | [ASCII Corporation](https://en.wikipedia.org/wiki/ASCII_Corporation) or Nexsoft
`B2`  | [Bandai](https://en.wikipedia.org/wiki/Bandai)
`B4`  | [Square Enix](https://en.wikipedia.org/wiki/Square_Enix)
`B6`  | [HAL Laboratory](https://en.wikipedia.org/wiki/HAL_Laboratory)
`B7`  | [SNK](https://en.wikipedia.org/wiki/SNK)
`B9`  | [Pony Canyon](https://en.wikipedia.org/wiki/Pony_Canyon)
`BA`  | [Culture Brain](https://en.wikipedia.org/wiki/Culture_Brain)
`BB`  | [Sunsoft](https://en.wikipedia.org/wiki/Sunsoft)
`BD`  | [Sony Imagesoft](https://en.wikipedia.org/wiki/Sony_Imagesoft)
`BF`  | [Sammy Corporation](https://en.wikipedia.org/wiki/Sammy_Corporation)
`C0`  | [Taito](https://en.wikipedia.org/wiki/Taito)
`C2`  | [Kemco](https://en.wikipedia.org/wiki/Kemco)
`C3`  | [Square](https://en.wikipedia.org/wiki/Square_(video_game_company))
`C4`  | [Tokuma Shoten](https://en.wikipedia.org/wiki/Tokuma_Shoten)
`C5`  | [Data East](https://en.wikipedia.org/wiki/Data_East)
`C6`  | [Tonkin House](https://en.wikipedia.org/wiki/Tonkin_House)
`C8`  | [Koei](https://en.wikipedia.org/wiki/Koei)
`C9`  | UFL
`CA`  | [Ultra Games](https://en.wikipedia.org/wiki/Ultra_Games)
`CB`  | [VAP, Inc.](https://en.wikipedia.org/wiki/VAP,_Inc.)
`CC`  | [Use Corporation](https://en.wikipedia.org/wiki/Category:Use_Corporation_games)
`CD`  | [Meldac](https://en.wikipedia.org/wiki/Meldac)
`CE`  | [Pony Canyon](https://en.wikipedia.org/wiki/Pony_Canyon)
`CF`  | [Angel](https://www.mobygames.com/company/5083/angel)
`D0`  | [Taito](https://en.wikipedia.org/wiki/Taito)
`D1`  | [SOFEL (Software Engineering Lab)](https://en.wikipedia.org/wiki/SOFEL)
`D2`  | [Quest](https://en.wikipedia.org/wiki/Quest_Corporation)
`D3`  | [Sigma Enterprises](https://www.mobygames.com/company/5001/sigma-enterprises-inc)
`D4`  | [ASK Kodansha Co.](https://www.mobygames.com/company/5166/ask-co-ltd/)
`D6`  | [Naxat Soft](https://en.wikipedia.org/wiki/Kaga_Create)[^kaga]
`D7`  | [Copya System](https://en.wikipedia.org/wiki/Category:Copya_Systems_games)
`D9`  | [Banpresto](https://en.wikipedia.org/wiki/Banpresto)
`DA`  | [Tomy](https://en.wikipedia.org/wiki/Tomy)
`DB`  | [LJN](https://en.wikipedia.org/wiki/LJN)
`DD`  | [Nippon Computer Systems](https://www.ncsx.co.jp/)
`DE`  | [Human Ent.](https://en.wikipedia.org/wiki/Human_Entertainment)
`DF`  | [Altron](https://en.wikipedia.org/wiki/Category:Altron_games)
`E0`  | [Jaleco](https://en.wikipedia.org/wiki/Jaleco)
`E1`  | [Towa Chiki](https://en.wikipedia.org/wiki/Towa_Chiki)
`E2`  | [Yutaka](https://en.wikipedia.org/wiki/Yutaka_(video_game_company)) # Needs more info
`E3`  | [Varie](https://en.wikipedia.org/wiki/Varie)
`E5`  | [Epoch](https://en.wikipedia.org/wiki/Epoch_Co.)
`E7`  | [Athena](https://en.wikipedia.org/wiki/Athena_(game_developer))
`E8`  | [Asmik Ace Entertainment](https://en.wikipedia.org/wiki/Asmik_Ace)
`E9`  | [Natsume](https://en.wikipedia.org/wiki/Natsume_Inc.)
`EA`  | [King Records](https://en.wikipedia.org/wiki/King_Records_(Japan))
`EB`  | [Atlus](https://en.wikipedia.org/wiki/Atlus)
`EC`  | Epic/[Sony Records](https://en.wikipedia.org/wiki/Sony_Music)
`EE`  | [IGS](https://web.archive.org/web/20240825224157/https://igs-entertainment.co/)
`F0`  | [A Wave](https://www.mobygames.com/company/9123/a-wave-inc/)
`F3`  | [Extreme Entertainment](https://www.mobygames.com/company/4221/extreme-entertainment-group-inc)
`FF`  | [LJN](https://en.wikipedia.org/wiki/LJN)

[^ubisoft]: Later known as [Ubisoft](https://en.wikipedia.org/wiki/Ubisoft).

[^blueplanet]: Later succeeded by [Blue Planet Software](https://en.wikipedia.org/wiki/Blue_Planet_Software), then acquired by [The Tetris Company](https://en.wikipedia.org/wiki/The_Tetris_Company) in 2020.

[^virgin]: Later known as [Virgin Mastertronic Ltd., then Virgin Interactive Entertainment, then Avalon Interactive Group, Ltd.](https://en.wikipedia.org/wiki/Virgin_Interactive_Entertainment).

[^lucasfilm]: Later known as [LucasArts](https://en.wikipedia.org/wiki/Lucasfilm_Games) between 1990-2021.

[^atari]: Later known as [Atari SA](https://en.wikipedia.org/wiki/Atari_SA).

[^sculptured]: Later accquired by [Iguana Entertainment](https://en.wikipedia.org/wiki/Iguana_Entertainment) in 1995. Parent studio owned by [Acclaim Entertainment](https://en.wikipedia.org/wiki/Acclaim_Entertainment).

[^sci]: Later known as [SCi (Sales Curve Interactive), then SCi Entertainment Group plc, then Eidos](https://en.wikipedia.org/wiki/SCi_Games), then acquired by [Square Enix](https://en.wikipedia.org/wiki/Square_Enix) in 2009.

[^spike]: Later known as [Spike Chunsoft Co., Ltd.](https://en.wikipedia.org/wiki/Spike_Chunsoft).

[^kaga]: Later known as [Kaga Create](https://en.wikipedia.org/wiki/Kaga_Create).

[^tokaicomm]: Known as Vic Tokai Corporation until 2011 when the name changed to Tokai Communications.

[^infogrames]: Later Infogrames North America, Inc.

[^caesars]: Later named Caesars Entertainment, Inc.

[^creatures]: Now known as Creatures, Inc.

[^imax]: Not to be confused with the IMAX motion picture film format.

[^segabuy]: Merged into Sega as Sega-Yonezawa, later becoming Sega Toys, and finally Sega Fave.

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
