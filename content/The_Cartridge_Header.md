An internal information area is located at 0100-014F in each cartridge.
It contains the following values:

### 0100-0103 - Entry Point

After displaying the Nintendo Logo, the built-in boot procedure jumps to
this address (100h), which should then jump to the actual main program
in the cartridge. Usually this 4 byte area contains a NOP instruction,
followed by a JP 0150h instruction. But not always.

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
incorrect. A CGB verifies only the first 18h bytes of the bitmap, but
others (for example a pocket gameboy) verify all 30h bytes.

### 0134-0143 - Title

Title of the game in UPPER CASE ASCII. If it is less than 16 characters
then the remaining bytes are filled with 00's. When inventing the CGB,
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
 80h - Game supports CGB functions, but works on old gameboys also.
 C0h - Game works on CGB only (physically the same as 80h).
```
Values with Bit 7 set, and either Bit 2 or 3 set, will switch the
gameboy into a special non-CGB-mode with uninitialized palettes. Purpose
unknown, eventually this has been supposed to be used to colorize
monochrome games that include fixed palette data at a special location
in ROM.

### 0144-0145 - New Licensee Code

Specifies a two character ASCII licensee code, indicating the company or
publisher of the game. These two bytes are used in newer games only
(games that have been released after the SGB has been invented). Older
games are using the header entry at `014B` instead.

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

- `00h` : No SGB functions (Normal Gameboy or CGB only game)
- `03h` : Game supports SGB functions

The SGB disables its SGB functions if this byte is set to another value than `03h`.

### 0147 - Cartridge Type

Specifies which Memory Bank Controller (if any) is used in the
cartridge, and if further external hardware exists in the cartridge.

|Code| Type|
|-----|-----------|
|`00h`|  ROM ONLY|
|`01h`|  MBC1|
|`02h`|  MBC1+RAM |
|`03h`|  MBC1+RAM+BATTERY|
|`05h`|  MBC2|
|`06h`|  MBC2+BATTERY|
|`08h`|  ROM+RAM|
|`09h`|  ROM+RAM+BATTERY |
|`0Bh`|  MMM01|
|`0Ch`|  MMM01+RAM|
|`0Dh`|  MMM01+RAM+BATTERY|
|`0Fh`|  MBC3+TIMER+BATTERY|
|`10h`|  MBC3+TIMER+RAM+BATTERY|
|`11h`|  MBC3|
|`12h`|  MBC3+RAM|
|`13h`|  MBC3+RAM+BATTERY|
|`19h`|  MBC5|
|`1Ah`|  MBC5+RAM|
|`1Bh`|  MBC5+RAM+BATTERY|
|`1Ch`|  MBC5+RUMBLE|
|`1Dh`|  MBC5+RUMBLE+RAM|
|`1Eh`|  MBC5+RUMBLE+RAM+BATTERY|
|`20h`|  MBC6|
|`22h`|  MBC7+SENSOR+RUMBLE+RAM+BATTERY|
|`FCh`|  POCKET CAMERA|
|`FDh`|  BANDAI TAMA5|
|`FEh`|  HuC3|
|`FFh`|  HuC1+RAM+BATTERY|


### 0148 - ROM Size

Specifies the ROM Size of the cartridge. Typically calculated as "32KB
shl N".

|code | Size      | Banks |
|-----|-----------|--------------|
|`00h`|  32 KByte |2 banks <br> (No ROM banking)|
|`01h`|  64 KByte |4 banks|
|`02h`| 128 KByte |8 banks|
|`03h`| 256 KByte |16 banks|
|`04h`| 512 KByte |32 banks|
|`05h`|   1 MByte |64 banks|
|`06h`|   2 MByte |128 banks|
|`07h`|   4 MByte |256 banks|
|`08h`|   8 MByte |512 banks|
|`52h`| 1.1 MByte |72 banks|
|`53h`| 1.2 MByte |80 banks|
|`54h`| 1.5 MByte |96 banks|


### 0149 - RAM Size

Specifies the size of the external RAM in the cartridge (if any).

```
 00h - None
 01h - 2 KBytes
 02h - 8 KBytes
 03h - 32 KBytes (4 banks of 8KBytes each)
 04h - 128 KBytes (16 banks of 8KBytes each)
 05h - 64 KBytes (8 banks of 8KBytes each)
```

When using a MBC2 chip 00h must be specified in this entry, even though
the MBC2 includes a built-in RAM of 512 x 4 bits.

### 014A - Destination Code

Specifies if this version of the game is supposed to be sold in Japan,
or anywhere else. Only two values are defined.

```
00h - Japanese
01h - Non-Japanese
```

### 014B - Old Licensee Code

Specifies the games company/publisher code in range 00-FFh. A value of
33h signalizes that the New License Code in header bytes 0144-0145 is
used instead. (Super Game Boy functions won't work if \<\> \$33.) A list
of licensee codes can be found
[here](https://raw.githubusercontent.com/gb-archive/salvage/master/txt-files/gbrom.txt).

### 014C - Mask ROM Version number

Specifies the version number of the game. That is usually 00h.

### 014D - Header Checksum

Contains an 8 bit checksum across the cartridge header bytes 0134-014C.
The checksum is calculated as follows:

```
x=0:FOR i=0134h TO 014Ch:x=x-MEM[i]-1:NEXT
```

The lower 8 bits of the result must be the same than the value in this
entry. The GAME WON'T WORK if this checksum is incorrect.

### 014E-014F - Global Checksum

Contains a 16 bit checksum (upper byte first) across the whole cartridge
ROM. Produced by adding all bytes of the cartridge (except for the two
checksum bytes). The Game Boy doesn't verify this checksum.

