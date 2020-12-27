An internal information area is located at $0100-014F in each cartridge.
It contains the following values:

### 0100-0103 - Entry Point

After displaying the Nintendo Logo, the built-in boot procedure jumps to
this address ($0100), which should then jump to the actual main program
in the cartridge. Usually this 4 byte area contains a `nop` instruction,
followed by a `jp $0150` instruction. But not always.

### 0104-0133 - Nintendo Logo

These bytes define the bitmap of the Nintendo logo that is displayed
when the Game Boy gets turned on. The hexdump of this bitmap is:
```
 CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
 00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
 BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
```
The Game Boy's boot procedure verifies the content of this bitmap
(after it has displayed it), and LOCKS ITSELF UP if these bytes are
incorrect. A CGB verifies only the first half ($18 bytes of) the bitmap, but
others (for example a pocket gameboy) verify all $30 bytes.

### 0134-0143 - Title

Title of the game in UPPER CASE ASCII. If it is less than 16 characters
then the remaining bytes are filled with $00 bytes. When inventing the CGB,
Nintendo has reduced the length of this area to 15 characters, and some
months later they had the fantastic idea to reduce it to 11 characters
only. The new meaning of the ex-title bytes is described below.

### 013F-0142 - Manufacturer Code

In older cartridges this area has been part of the Title (see above), in
newer cartridges this area contains an 4 character uppercase
manufacturer code. Purpose and Deeper Meaning unknown.

### 0143 - CGB Flag

In older cartridges this byte has been part of the Title (see above). In
CGB cartridges the upper bit is used to enable CGB functions. This is
required, otherwise the CGB switches itself into Non-CGB-Mode. Typical
values are:
```
 80h - Game supports CGB functions, but also works on old Game Boys.
 C0h - Game works on CGB only (physically the same as $80).
```
Values with Bit 7 set, and either Bit 2 or 3 set, will switch the
gameboy into a special non-CGB-mode called "PGB mode".

TODO: research and document PGB modes...

### 0144-0145 - New Licensee Code

Specifies a two-character ASCII licensee code, indicating the company or
publisher of the game. These two bytes are used in newer games only
(games that have been released after the SGB has been invented). Older
games are using the header entry at `$014B` instead.

Sample licensee codes :

|Code|Publisher|
|----|---------|
|`00`|   none |
|`01`|   Nintendo R&D1  |
|`08`|   Capcom |
|`13`|   Electronic Arts  |
|`18`|   Hudson Soft  |
|`19`|   b-ai |
|`20`|   kss  |
|`22`|   pow  |
|`24`|   PCM Complete |
|`25`|   san-x  |
|`28`|   Kemco Japan  |
|`29`|   seta |
|`30`|   Viacom |
|`31`|   Nintendo |
|`32`|   Bandai |
|`33`|   Ocean/Acclaim  |
|`34`|   Konami |
|`35`|   Hector |
|`37`|   Taito  |
|`38`|   Hudson |
|`39`|   Banpresto  |
|`41`|   Ubi Soft |
|`42`|   Atlus  |
|`44`|   Malibu |
|`46`|   angel  |
|`47`|   Bullet-Proof |
|`49`|   irem |
|`50`|   Absolute |
|`51`|   Acclaim  |
|`52`|   Activision |
|`53`|   American sammy |
|`54`|   Konami |
|`55`|   Hi tech entertainment  |
|`56`|   LJN  |
|`57`|   Matchbox |
|`58`|   Mattel |
|`59`|   Milton Bradley |
|`60`|   Titus  |
|`61`|   Virgin |
|`64`|   LucasArts  |
|`67`|   Ocean  |
|`69`|   Electronic Arts  |
|`70`|   Infogrames |
|`71`|   Interplay  |
|`72`|   Broderbund |
|`73`|   sculptured |
|`75`|   sci  |
|`78`|   THQ  |
|`79`|   Accolade |
|`80`|   misawa |
|`83`|   lozc |
|`86`|   Tokuma Shoten Intermedia |
|`87`|   Tsukuda Original |
|`91`|   Chunsoft |
|`92`|   Video system |
|`93`|   Ocean/Acclaim  |
|`95`|   Varie  |
|`96`|   Yonezawa/s'pal |
|`97`|   Kaneko |
|`99`|   Pack in soft |
|`A4`|   Konami (Yu-Gi-Oh!) |

### 0146 - SGB Flag

Specifies whether the game supports SGB functions, common values are:

- `$00` : No SGB functions (Normal Gameboy or CGB only game)
- `$03` : Game supports SGB functions

The SGB disables its SGB functions if this byte is set to a value other than `$03`.

### 0147 - Cartridge Type

Specifies which Memory Bank Controller (if any) is used in the
cartridge, and if further external hardware exists in the cartridge.

|Code| Type|
|-----|-----------|
|`$00`|  ROM ONLY|
|`$01`|  MBC1|
|`$02`|  MBC1+RAM |
|`$03`|  MBC1+RAM+BATTERY|
|`$05`|  MBC2|
|`$06`|  MBC2+BATTERY|
|`$08`|  ROM+RAM|
|`$09`|  ROM+RAM+BATTERY |
|`$0B`|  MMM01|
|`$0C`|  MMM01+RAM|
|`$0D`|  MMM01+RAM+BATTERY|
|`$0F`|  MBC3+TIMER+BATTERY|
|`$10`|  MBC3+TIMER+RAM+BATTERY|
|`$11`|  MBC3|
|`$12`|  MBC3+RAM|
|`$13`|  MBC3+RAM+BATTERY|
|`$19`|  MBC5|
|`$1A`|  MBC5+RAM|
|`$1B`|  MBC5+RAM+BATTERY|
|`$1C`|  MBC5+RUMBLE|
|`$1D`|  MBC5+RUMBLE+RAM|
|`$1E`|  MBC5+RUMBLE+RAM+BATTERY|
|`$20`|  MBC6|
|`$22`|  MBC7+SENSOR+RUMBLE+RAM+BATTERY|
|`$FC`|  POCKET CAMERA|
|`$FD`|  BANDAI TAMA5|
|`$FE`|  HuC3|
|`$FF`|  HuC1+RAM+BATTERY|


### 0148 - ROM Size

Specifies the ROM Size of the cartridge. Typically calculated as "N such that 32 KiB
<< N".

|code | Size      | Banks |
|-----|-----------|--------------|
|`$00`|  32 KByte |2 banks <br> (No ROM banking)|
|`$01`|  64 KByte |4 banks|
|`$02`| 128 KByte |8 banks|
|`$03`| 256 KByte |16 banks|
|`$04`| 512 KByte |32 banks|
|`$05`|   1 MByte |64 banks|
|`$06`|   2 MByte |128 banks|
|`$07`|   4 MByte |256 banks|
|`$08`|   8 MByte |512 banks|
|`$52`| 1.1 MByte |72 banks|
|`$53`| 1.2 MByte |80 banks|
|`$54`| 1.5 MByte |96 banks|


### 0149 - RAM Size

Specifies the size of the external RAM in the cartridge (if any).

```
 $00 - None
 $01 - 2 KBytes
 $02 - 8 KBytes
 $03 - 32 KBytes (4 banks of 8KBytes each)
 $04 - 128 KBytes (16 banks of 8KBytes each)
 $05 - 64 KBytes (8 banks of 8KBytes each)
```

When using a MBC2 chip $00 must be specified in this entry, even though
the MBC2 includes a built-in RAM of 512 x 4 bits.

### 014A - Destination Code

Specifies if this version of the game is supposed to be sold in Japan,
or anywhere else. Only two values are defined.

```
$00 - Japanese
$01 - Non-Japanese
```

### 014B - Old Licensee Code

Specifies the games company/publisher code in range $00-FF. A value of
$33 signals that the New Licensee Code (in header bytes \$0144-0145) is
used instead. (Super Game Boy functions won't work if \<\> \$33.) A list
of licensee codes can be found
[here](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt).

### 014C - Mask ROM Version number

Specifies the version number of the game. That is usually $00.

### 014D - Header Checksum

Contains an 8 bit checksum across the cartridge header bytes $0134-014C.
The boot ROM computes `x` as follows:

```
x = 0
i = $0134
while i <= $014C
	x = x - [i] - 1
```

If the byte at $014D does not match the lower 8 bits of `x`, the boot ROM will lock up,
and the cartridge program **won't run**.

### 014E-014F - Global Checksum

Contains a 16 bit checksum (upper byte first) across the whole cartridge
ROM. Produced by adding all bytes of the cartridge (except for the two
checksum bytes). The Game Boy doesn't verify this checksum.
