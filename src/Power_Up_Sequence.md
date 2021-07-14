# Power-Up Sequence

When the Game Boy is powered up, the CPU starts executing instuctions at address $0000—**not** $0100!
A program called the *boot ROM*, burned inside the CPU, is mapped "over" the cartridge ROM at first, which is responsible for the boot-up animation played before control is handed over to the cartridge's ROM.

There exist 8 different known official boot ROMs:

Name | Size (bytes) | Notes
-----|--------------|------------------------------------------------------------
DMG0 | 256          | Blinks on failed checks, no ®
DMG  | 256          | |
MGB  | 256          | One-byte difference to DMG
SGB  | 256          | Only forwards logo to SGB BIOS, performs no checks
SGB2 | 256          | Same difference to SGB than between MGB and DMG
CGB0 | 256 + 1792   | Does not init [wave RAM](<#FF30-FF3F - Wave Pattern RAM>)
CGB  | 256 + 1792   | Split in two parts, with the cartridge header in the middle
AGB  | 256 + 1792   | Fixes "logo TOCTTOU"

A disassembly of all of them [is available online](https://github.com/ISSOtm/gb-bootroms).

## Monochrome models (DMG0, DMG, MGB)

The monochrome boot ROMs read [the logo from the header](<#0104-0133 - Nintendo Logo>), unpack it into VRAM, and then start slowly scrolling it down.
Since reads from an absent cartridge usually return $FF, this explains why powering the console on without a cartridge scrolls a black box.
Additionally, fauly or dirty connections can cause the data read to be corrupted, resulting in a jumbled-up logo.

*Once the logo has finished scrolling*, the boot ROM plays the famous "ba-ding!" sound, and reads the logo **again**, this time comparing it to a copy it stores.
Then, it also computes the header checksum, and compares it to [the checksum stored in the header](<#014D - Header Checksum>).
If either of these checks fail, the boot ROM **locks up**, and control is never passed to the cartridge ROM.

Finally, the boot ROM writes to the `BANK` register at $FF50, which unmaps the boot ROM.
The `ldh [$FF50], a` instruction being located at $00FE (and being two bytes long), [the first instruction executed from the cartridge ROM is at $0100](<#0100-0103 - Entry Point>).

Since the A register is used to write to $FF50, its value is passed to the cartridge ROM; the only difference between the DMG and MGB boot ROMs is that the former writes $01, and the latter uses $FF.

### DMG0

The DMG0 is a rare "early bird" variant of the DMG boot ROM present in few early DMGs.
The behavior of the boot ROM is globally the same, but significant portions of the code have been rearranged.

Interestingly, the DMG0 boot ROM performs both the logo and checksum checks before displaying anything.
If either verification fails, the screen is made to blink while the boot ROM locks up, alternating between solid white and solid black.

The DMG0 boot ROM also lacks the ® symbol next to the Nintendo logo.

Like the DMG, the DMG0 boot ROM writes $01 to $FF50.

## Super Game Boy (SGB, SGB2)

These boot ROMs are fairly unique in that they do *not* perform header checks.
Instead, they set up the Nintendo logo in VRAM from the header just like the monochrome boot ROMs, but then they send the entire header to the SGB BIOS via [the standard packet-transferring procedure](<#Command Packet Transfers>), using packet header bytes $F1, $F3, $F5, $F7, $F9, and $FB, in that order.
(These packet IDs are otherwise invalid and never used in regular SGB operation, though it seems that not all SGB BIOS revisions filter them out.)

The boot ROM then unmaps itself and hands off execution to the cartridge ROM without performing any checks.
The SGB BIOS, the program running on the SNES, actually verifies the Nintendo logo and header checksum itself.
If either verification fails, the BIOS itself locks up, repeatedly resetting the SGB CPU within the cartridge.

As the DMG and MGB boot ROMs, the SGB and SGB2 boot ROMs write $01 and $FF respectively to $FF50, and this is also the only difference between these two boot ROMs.

The way the packet-sending routine works makes transferring a set bit *one cycle* faster than transferring a reset bit.
This means that the time taken by the SGB boot ROMs *depends on the cartridge's header*!
The complexity of this is compounded by the fact that the boot ROM waits for 4 VBlanks after transferring each packet, mostly but not entirely grouping the timings.

## Color models (CGB0, CGB, AGB)

The color boot ROMs are much more complicated, notably because of the compatibility behavior.

First, the boot ROM is larger, as indicated above: 2048 bytes total.
It still has to be mapped starting at $0000, since this is where the CPU starts, but it must also access the cartridge header at $0100-014F.
Thus, the boot ROM is actually split in two parts, a $0000-00FF one, and a $0200-08FF one.

First, the boot ROMs unpack the Nintendo logo to VRAM like the monochrome models, likely for compatibility, and copies the logo to a buffer in HRAM at the same time.
(It is speculated that HRAM was used due to it being embedded within the CPU, unlike WRAM, so that it couldn't be tampered with.)

Then, the logo is read *again*, but this time, it is decompressed without upscaling, yielding the much smaller logo placed below the big "GAME BOY" one.
The boot ROM then sets up compatibility palettes, as described further below, and plays the logo animation with the "ba-ding!" sound.

During the logo animation, and if bit 7 of [the CGB compatibility byte](<#0143 - CGB Flag>) is reset (indicating a monochrome-only game), the user is allowed to pick a palette to override the one chosen for compatibility.
Each new choice prevents the animation from ending for 30 frames, potentially delaying the checks and fade-out.

Then, like the monochrome boot ROMs, the header logo is checked *from the buffer in HRAM*, and the header checksum is verified.
For some reason, however, only the first half of the logo is checked, despite the full logo being present in the HRAM buffer.

Finally, the boot ROM fades all BG palettes to white, and sets up compatibility.
If [the CGB compatibility byte](<#0143 - CGB Flag>) indicates CGB compatibility, the byte is written directly to `KEY0` ($FF4C), potentially enabling PGB mode; otherwise, $04 is written to `KEY0` (enabling DMG compatibility mode in the CPU), $01 is written to [`OPRI`](<#FF6C - OPRI - CGB Mode Only - Object Priority Mode>) (enabling [DMG OBJ priority](<#Sprite Priorities and Conflicts>)), and the compatibility palettes are written.
Additionally, the DMG logo tilemap is written if the compatibility requests it.

Like all other boot ROMs, the last thing the color boot ROMs do is hand off execution at the same time as they unmap themselves, though they write $11 instead of $01 or $FF.

### CGB0

Like the DMG0 boot ROM, some early CGBs contain a different boot ROM.
Unlike DMG0 and DMG, the differences between the CGB0 and CGB boot ROM, are very minor, with no change in the layout of the ROM.

The most notable change is that the CGB0 boot ROM does *not* init [wave RAM](<#FF30-FF3F - Wave Pattern RAM>).
This is known to cause, for example, a different title screen music in the game *R-Type*.

The CGB0 boot ROM also writes copies of other variables to some locations in WRAM that are not otherwise run from anywhere.
It is speculated that this may be debug remnants.

### Compatibility palettes

It is fairly well-known that the Game Boy Color automatically colorizes monochrome-only games.
The boot ROM is actually mostly responsible for this.

When in DMG compatibility mode, the [CGB palettes](<#LCD Color Palettes (CGB only)>) are still being used: the background uses BG palette 0 (likely because the entire [attribute map](<#BG Map Attributes (CGB Mode only)>) is set to all zeros), and objects use OBJ palette 0 or 1 depending on bit 4 of [their attribute](<#Byte3 - Attributes/Flags:>).
[`BGP`, `OBP0`, and `OBP1`](<#LCD Monochrome Palettes>) actually index into the CGB palettes instead of the DMG's shades of grey.

The boot ROM picks a compatibility palette using an ID computed using the following algorithm:
1. Check if the [old licensee code](<#014B - Old Licensee Code>) is $33.
   * If yes, the [new licensee code](<#0144-0145 - New Licensee Code>) must be used. Check that it equals the ASCII string `"01"`.
   * If not, check that it equals $01.

   <p>In effect, this checks that the licensee in the header is Nintendo.</p>

   * If this check fails, palettes ID $00 is used.
   * Otherwise, the algorithm proceeds.
1. Compute the sum of all 16 [game title](<#0134-0143 - Title>) bytes, storing this as the "title checksum".
1. Find the title checksum [in a table](https://github.com/ISSOtm/gb-bootroms/blob/443d7f057ae06e8d1d76fa8083650cf0be2cd0ae/src/cgb.asm#L1221-L1230), and record its index within the table.

   An almost-complete list of titles corresponding to the different checksums can be found in [Liji's free CGB boot ROM reimplementation](https://github.com/LIJI32/SameBoy/blob/1d7692cff5552e296be5e1ab075c4f187f57132c/BootROMs/cgb_boot.asm#L230-L328).
   * If not found, palettes ID $00 is used.
   * If the index is 64 or below, the index is used as-is as the palettes ID, and the algorithm ends.
   * Otherwise, it must be further corrected based on the title's fourth letter; proceed to the step below.
1. The fourth letter is searched for in [another table](https://github.com/ISSOtm/gb-bootroms/blob/443d7f057ae06e8d1d76fa8083650cf0be2cd0ae/src/cgb.asm#L1232-L1240).
   * If the letter can't be found, palettes ID $00 is used.
   * If the letter is found, the index obtained in the previous step is increased by 14 times the row index to get the palettes ID.
     (So, if the letter was found in the first row, the index is unchanged; if it's found in the second row, it's increased by 14, and so on.)

The resulting palettes ID is used to pick 3 palettes out of a table via a fairly complex mechanism.
The user can override this choice using certain button combinations during the logo animation; some of these manual choices are identical to auto-colorizations, [but others are unique](https://tcrf.net/Notes:Game_Boy_Color_Bootstrap_ROM#Manual_Select_Palette_Configurations).

::: tip

A table of checksums (and tie-breaker fourth letters when applicable) and the corresponding palettes can be found [on TCRF](https://tcrf.net/Notes:Game_Boy_Color_Bootstrap_ROM#Assigned_Palette_Configurations).

:::

If the ID is either $43 or $58, then the Nintendo logo's tilemap is written to VRAM.
This is intended for games that perform some kind of animation with the Nintendo logo; it suddenly appears in the middle of the screen, though, so it may look better for homebrew not to use this mechanism.

### Scrapped palette switching menu

Remnants of a functionality designed to allow switching the CGB palettes while the game is running exist in the CGB CPU.

## Stadium 2

Pokémon Stadium 2's "GB Tower" emulator contains a very peculiar boot ROM.
It can be found at offset $015995F0 in the US release, and is $3F0 bytes long.
Its purpose is unknown.

This boot ROM does roughly the same setup as a regular CGB boot ROM, but writes to $FF50 very early, and said write is followed by a lock-up loop.
Further, the boot ROM contains a valid header, which is mostly blank save for the logo, compatibility flag (which indicates dual compatibility), and header checksum.

## Logo check

While it may make sense for the boot ROM to at least partially verify the ROM's integrity via the header check, why does it check the logo's integrity separately?

### Legal implications

::: warning Caution

The following is advisory, but **is not legal advice**.
If necessary (e.g. commercial releases with logos on the boxes), consult a lawyer.

:::

The logo check was meant to deter piracy using trademark law.
Unlike nowadays, the Game Boy's technology was not sufficient to require Nintendo's approval to make a game run on it, and Nintendo decided against hardware protection like the NES' [lockout chip](https://wiki.nesdev.com/w/index.php/CIC_lockout_chip) likely for cost and/or power consumption reasons.

Instead, the boot ROM's logo check forces each ROM intended to run on the system to contain an (encoded) copy of the Nintendo logo, which is displayed on startup.
Nintendo's strategy was to threaten pirate developers with suing for trademark infringement.

Fortunately, [*Sega v. Accolade*](https://en.wikipedia.org/wiki/Sega_v._Accolade) ruled (in the US) that use of a trademarked logo is okay if it is *necessary* for running programs on the console, so there is no danger for homebrew developers.

That said, if you want to explicitly mark the lack of licensing from Nintendo, you can add some text to the logo screen once the boot ROM hands off control, for example like this:

![Mockup screenshot of an endorsement disambiguation screen](imgs/not_licensed.png)

### Bypass

The Nintendo logo check has been [circumvented many times](http://fuji.drillspirits.net/?post=87), be it to avoid legal action from Nintendo or for the swag, and there are basically two ways of doing so.

One is to exploit a [TOCTTOU](https://en.wikipedia.org/wiki/TOCTTOU) vulnerability in the way the console reads the logo (doing so once to draw it, and the other time to check it), which has however been patched on the AGB.
This requires custom hardware in the cartridge, however, and is made difficult by the timing and order of the reads varying greatly between boot ROMs.
Some implementations use a custom mapper, others use a capacitor holding some of the address lines to redirect reads to a separate region of ROM containing the modified logo.

The other way is Game Boy Color (and Advance) exclusive: for some reason, the boot ROM copies the full logo into HRAM, but only compares the first half.
Thus, a logo whose top half is correct but not the bottom half will get a pass from the CGB boot ROM.
Strangely, despite correcting the TOCTTOU vulnerability, the CGB-AGB boot ROM does *not* fix this mistake.

## Console state after boot ROM hand-off

Regardless of the console you intend for your game to run on, it is prudent to rely on as little of the following as possible, barring what is mentioned elsewhere in this documentation to detect which system you are running on.
This ensures maximum compatibility, both across consoles and cartridges (especially flashcarts, which typically run their own menu code before your game), increases reliability, and is generally considered good practice.

The following information may be incomplete, as well, so rely on this at your own risk.

### Common remarks

The console's WRAM and HRAM are random on power-up.
[Different models tend to exhibit different patterns](https://twitter.com/CasualPkPlayer/status/1409752977812852736?s=19), but they are random nonetheless, even depending on factors such as the ambient teperature.
Besides, turning the system off and on again has proven reliable enough [to carry over RAM from one game to another](https://www.youtube.com/watch?v=xayxmTLljr8), so it's not a good idea to rely on it at all.

Emulation of uninitialized RAM is spotty: some emulators fill RAM with a constant on startup (typically $00 or $FF), some emulators fully randomize RAM, and others attempt to reproduce the patterns observed on hardware.
It is a good idea to enable your favorite emulator's "break on uninitialized RAM read" exception (and if it doesn't have one, to consider using an emulator that does).

While technically not related to power-on, it is worth noting that external RAM in the cartridge, when present, usually contains random garbage data when first powered on.
It is strongly advised for the game to put a large enough known sequence of bytes at a fixed location in SRAM, and check its presence before accessing any saved data.

### CPU registers

Register | DMG   | MGB   | SGB   | SGB2  | CGB   | AGB
---------|-------|-------|-------|-------|-------|------
**A**    | $01   | $FF   | $01   | $FF   | $11   | $11
**F**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**B**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**C**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**D**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**E**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**H**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**L**    | $xx   | $xx   | $xx   | $xx   | $xx   | $xx
**PC**   | $0100 | $0100 | $0100 | $0100 | $0100 | $0100
**SP**   | $FFFE | $FFFE | $FFFE | $FFFE | $FFFE | $FFFE

## Hardware registers

TODO

```
  AF=$01B0
  BC=$0013
  DE=$00D8
  HL=$014D
  Stack Pointer=$FFFE
  [$FF05] = $00   ; TIMA
  [$FF06] = $00   ; TMA
  [$FF07] = $00   ; TAC
  [$FF10] = $80   ; NR10
  [$FF11] = $BF   ; NR11
  [$FF12] = $F3   ; NR12
  [$FF14] = $BF   ; NR14
  [$FF16] = $3F   ; NR21
  [$FF17] = $00   ; NR22
  [$FF19] = $BF   ; NR24
  [$FF1A] = $7F   ; NR30
  [$FF1B] = $FF   ; NR31
  [$FF1C] = $9F   ; NR32
  [$FF1E] = $BF   ; NR34
  [$FF20] = $FF   ; NR41
  [$FF21] = $00   ; NR42
  [$FF22] = $00   ; NR43
  [$FF23] = $BF   ; NR44
  [$FF24] = $77   ; NR50
  [$FF25] = $F3   ; NR51
  [$FF26] = $F1-GB, $F0-SGB ; NR52
  [$FF40] = $91   ; LCDC
  [$FF42] = $00   ; SCY
  [$FF43] = $00   ; SCX
  [$FF45] = $00   ; LYC
  [$FF47] = $FC   ; BGP
  [$FF48] = $FF   ; OBP0
  [$FF49] = $FF   ; OBP1
  [$FF4A] = $00   ; WY
  [$FF4B] = $00   ; WX
  [$FFFF] = $00   ; IE
```
