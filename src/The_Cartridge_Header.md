# The Cartridge Header

Each cartridge contains a header, located at the address range `$0100`-`$014F`.
The cartridge header contains the following information:

## 0100-0103 – Entry Point

After displaying the Nintendo logo, the built-in boot procedure jumps to
this address (`$0100`), which should then jump to the actual main program
in the cartridge. Usually (but not always) this 4-byte area contains a `nop`
instruction followed by a `jp $0150` instruction.

## 0104-0133 – Nintendo Logo

These bytes contain a bitmap of the Nintendo logo that is displayed
when the Game Boy is powered on. The hex dump of this bitmap is:

```
CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
```

The Game Boy's boot procedure **first displays** the logo and **then verifies**
its contents. If the contents do not match the values above, the boot
ROM **locks itself up**.

A CGB verifies only the first half (`$18` bytes) of the bitmap, but other models
(e.g. the Game Boy Pocket) verify all `$30` bytes, as does the
Game Boy Advance.

## 0134-0143 – Title

These bytes contain the title of the game in UPPER CASE ASCII.
If the title is less than 16 characters long, the remaining bytes all have the
value `$00`.

While developing the CGB, Nintendo reduced the length of this area
to 15 characters, and some months later they had the fantastic idea to reduce it
further to just 11 characters. The new meanings of the bytes `$013F`-`$0143` are
described below.

## 013F-0142 – Manufacturer Code

In older cartridges these bytes were part of the Title (see above).
In newer cartridges they contain a 4-character manufacturer code
(uppercase). The exact purpose of the code is unknown.

## 0143 – CGB Flag

In older cartridges this byte was part of the Title (see above). In
CGB cartridges the most significant bit is used to enable CGB functions. This is
required; otherwise the CGB switches itself into Non-CGB-Mode.

Typical values are:

Code  | CGB support
------|------------------------------------------------------------------
`$80` | The game supports CGB functions (but also works on old Game Boys)
`$C0` | The game works on CGB only (physically the same as `$80`)

Values with bit 7 set, and either bit 2 or 3 set, will switch the
Game Boy into a special non-CGB-mode called "PGB mode".

TODO: research and document PGB modes...

## 0144-0145 – New Licensee Code

These bytes specify a two-character ASCII licensee code indicating the
developer/publisher of the game. The New Licensee Code is present in newer
cartridges only (those released after the SGB was invented).
Older cartridges use the Old Licensee Code at `$014B` instead.

Sample licensee codes:

Code | Publisher
-----|-----------
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
`A4` | Konami (Yu-Gi-Oh!)

## 0146 – SGB Flag

This byte specifies whether the game supports SGB functions.

Common values are:

Code  | SGB support
------|-----------------------------------------------------
`$00` | No SGB functions (regular Game Boy or CGB-only game)
`$03` | The game supports SGB functions

The SGB disables its SGB functions if this byte is set to a value other than `$03`.

## 0147 – Cartridge Type

This byte indicates what kind of Memory Bank Controller (and other hardware)
the cartridge contains.

Code  | Type
------|------
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
MBC3 with a 64 KiB RAM refers to MBC30, used only in
_Pocket Monsters: Crystal Version_ (the Japanese version of
_Pokémon Crystal Version_).

## 0148 – ROM Size

This byte specifies the size of the ROM on the cartridge. In most cases the ROM size
is given by `32 KiB x (2 ^ <value>)`:

Code  | ROM size  | Number of ROM banks
------|-----------|-----------------
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
As the other ROM sizes are all powers of 2, these are likely inaccurate. The source of these
values is unknown.

## 0149 – RAM Size

This byte specifies the size of the RAM on the cartridge (if any).

Code  | RAM size | Comment
------|----------|---------
`$00` |   0      | No RAM [^mbc2]
`$01` |   –      | Unused [^2kib_sram]
`$02` |   8 KiB  |  1 bank
`$03` |  32 KiB  |  4 banks of 8 KiB each
`$04` | 128 KiB  | 16 banks of 8 KiB each
`$05` |  64 KiB  |  8 banks of 8 KiB each

[^mbc2]:
When using a MBC2 chip, `$00` must be specified as the RAM Size, even though
MBC2 cartridges contain an on-chip RAM of 512 x 4 bits.

[^2kib_sram]:
Listed in various unofficial docs as 2 KiB. However, a 2 KiB RAM chip was never used in a cartridge.
The source of this value is unknown.

Various "PD" ROMs ("Public Domain" homebrew ROMs, generally tagged with `(PD)`
in the filename) are known to use the `$01` RAM Size tag, but this is believed
to have been a mistake with early homebrew tools and the PD ROMs often don't use
cartridge RAM at all.

## 014A – Destination Code

This byte specifies whether this version of the game is intended to be sold in
Japan or elsewhere.

Only two values are defined:

Code  | Destination
------|------------
`$00` | Japan
`$01` | Other

## 014B – Old Licensee Code

This byte is used in older (pre-SGB) cartridges to specify the developer/publisher
of the game. The value `$33` indicates that the New Licensee Code (in header bytes
`$0144`-`$0145`) is used instead. (Super Game Boy functions will only work if the
value is `$33`.)

A list of Old Licensee Codes can be found
[here](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt).

## 014C – Mask ROM Version number

This byte specifies the version number of the game. The value is usually `$00`.

## 014D – Header Checksum

This byte contains an 8-bit checksum computed from the cartridge header bytes
`$0134`-`$014C`. The boot ROM computes the checksum as follows:

```c
checksum := 0
address  := 0x0134
while address <= 0x014C:
	checksum := checksum - [address] - 1
	address  := address + 1
```

The boot ROM verifies this checksum. If the byte at `$014D` does not match the
lower 8 bits of `checksum`, the boot ROM will lock up, and the program on the
cartridge **won't run**.

## 014E-014F – ROM Checksum

These bytes contain a 16-bit (big-endian) checksum simply computed as the sum of
all the bytes of the cartridge ROM (except these two checksum bytes).

The boot ROM does not verify this checksum.
