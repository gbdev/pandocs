# GBC Enhancements

The GBC was engineered to provide enhancements over the monochrome consoles; the flagship colour functionality is [described elsewhere](<#LCD Color Palettes (CGB only)>), but there are other enhancements.
Some existing registers gain new functionality (e.g. [`SC`]'s speed booster), some existing registers change meaning (e.g. [`LCDC`'s bit 0]), and new registers are added at some previously-unused addresses: these chapters document the latter.

## Unlocking enhancements / compatibility mode

Games not made with the GBC in mind will malfunction with its enhancements; for this reason, the GBC has a backwards compatibility mode which it enables unless the game [advertises compatibility](<#0143 — CGB flag>).
We refer to the DMG compatibility mode as **Non-CGB Mode**, and the enhanced mode as **CGB Mode**.

Note that a GBC in compatibility mode still differs from monochrome consoles in several ways, which is known to [break at least two licensed games](<#Spurious STAT interrupts>).

Further, monochrome consoles do *not* perform any verification, and thus will still boot games that aren't meant to function without GBC enhancements.
All such official games detect that they are not running on a GBC (see below), and [display a message](https://www.vgmuseum.com/features/warn/) sometimes with special artwork, but some homebrew has gotten more creative and provided extra functionality, such as a music player.

(See also [trivia about the colorisation process].)

## Detecting GBC

The official way to detect a GBC is to examine the value of the CPU's accumulator (`a`) [left by the boot ROM]: $11 indicates GBC (or GBA in GBC compatibility mode).

This technique has two downsides: it only tells which console is running the game, but not whether enhancements are actually available; and it is only available at boot-up and requires storing its result in memory somewhere.
It is possible to instead detect whether the enhancements are enabled, since most registers' extra bits are not writable in compatibility mode.

### Detecting GBA

The GBA also has its own GBC backwards-compatibility mode (which *also* includes the GBC's DMG compatibility mode, don't get confused!), which is officially distinguishable by checking the value of `b` likewise: bit 0 is clear on GBC, and set on GBA.

The GBA's backwards compatibility mode has a few unique quirks, so it may be useful to detect it to compensate; the most common difference is using different colours due to [LCD differences], but some official games had special GBA-exclusive features (example: [*Shantae*'s Tinkerbat form](https://shantae.fandom.com/wiki/Forms#Tinkerbat)) as promotional bonuses.

## Documented registers

### VRAM Banks

The CGB has twice the VRAM of the DMG, but it is banked and either bank
has a different purpose.

#### FF4F — VBK (CGB Mode only): VRAM bank

This register can be written to change VRAM banks. Only bit 0
matters, all other bits are ignored.

#### VRAM bank 1

VRAM bank 1 is split like VRAM bank 0 ; 8000-97FF also stores tiles
(just like in bank 0), which can be accessed the same way as (and at the
same time as) bank 0 tiles. 9800-9FFF contains the attributes for the
corresponding Tile Maps.

Reading from this register will return the number of the currently
loaded VRAM bank in bit 0, and all other bits will be set to 1.

### FF4D — KEY1/SPD (CGB Mode only): Prepare speed switch

{{#bits 8 >
	"KEY1" 7:"Current speed" 0:"Switch armed"
}}

- **Current speed** (*Read-only*): `0` = Normal-speed mode, `1` = Double-speed mode
- **Switch armed** (*Read/Write*): `0` = No, `1` = Armed

This register is used to prepare the Game Boy to switch between CGB
Double Speed Mode and Normal Speed Mode. The actual speed switch is
performed by executing a `stop` instruction after Bit 0 has been set. After
that, Bit 0 will be cleared automatically, and the Game Boy will operate
at the "other" speed. The recommended speed switching procedure in
pseudocode would be:

```
IF KEY1_BIT7 != DESIRED_SPEED THEN
   IE = $00       ; (FFFF) = $00
   JOYP = $30     ; (FF00) = $30
   KEY1 = $01     ; (FF4D) = $01
   STOP
ENDIF
```

The CGB is operating in Normal Speed Mode when it is first turned on. Note
that using the Double Speed Mode increases the power consumption; therefore, it
would be recommended to use Normal Speed whenever possible.

In Double Speed Mode the following will operate twice as fast as normal:

- The CPU (2.10 MHz, so 1 M-cycle = approx. 0.5 µs)
- Timer and Divider Registers
- Serial Port (Link Cable)
- DMA Transfer to OAM

And the following will keep operating as usual:

- LCD Video Controller
- HDMA Transfer to VRAM
- All Sound Timings and Frequencies

The CPU stops for 2050 M-cycles (= 8200 T-cycles) after the `stop` instruction is
executed. During this time, the CPU is in a strange state. `DIV` does not tick, so
*some* audio events are not processed. Additionally, VRAM/OAM/... locking is "frozen", yielding
different results depending on the [PPU mode](<#PPU modes>) it's started in:

- HBlank / VBlank (Mode 0 / Mode 1): The PPU cannot access any video memory, and produces black pixels
- OAM scan (Mode 2): The PPU can access VRAM just fine, but not OAM, leading to rendering background, but not objects (sprites)
- Rendering (Mode 3): The PPU can access everything correctly, and so rendering is not affected

TODO: confirm whether interrupts can occur (just the joypad one?) during the pause, and consequences if so

### FF56 — RP (CGB Mode only): Infrared communications port

This register allows to input and output data through the CGBs built-in
Infrared Port. When reading data, bit 6 and 7 must be set (and obviously
Bit 0 must be cleared — if you don't want to receive your own Game Boy's
IR signal). After sending or receiving data you should reset the
register to $00 to reduce battery power consumption again.

{{#bits 8 >
	"RP" 7-6:"Read enable" 1:"Receiving" 0:"Emitting"
}}

- **Read enable** (*Read/Write*): `0` = Disable (bit 1 reads `1`), `3` = Enable
- **Receiving** (*Read-only*): `0` = Receiving IR signal, `1` = Normal
- **Emitting** (*Read/Write*): `0` = LED off, `1` = LED on

Note that the receiver will adapt itself to the normal level of IR
pollution in the air, so if you would send a LED ON signal for a longer
period, then the receiver would treat that as normal (=OFF) after a
while. For example, a Philips TV Remote Control sends a series of 32 LED
ON/OFF pulses (length 10us ON, 17.5us OFF each) instead of a permanent
880us LED ON signal. Even though being generally CGB compatible, the GBA
does not include an infra-red port.

### FF4C — KEY0/SYS (CGB Mode only): CPU mode select

This GBC-only register (which is not officially documented) is written only by the CGB boot ROM,
as it gets locked after the bootrom finish execution (by a write to the [BANK register](<#Monochrome models (DMG0, DMG, MGB)>)).

Once it is locked, the behavior of the system can't be changed without a reset (this behavior can be observed using [this test ROM](https://github.com/alloncm/MagenTests?tab=readme-ov-file#key0-cpu-mode-register-lock-after-boot)).

As a result of the above most of the behavior is not directly testable without hardware manipulation.
Even though we can't test its behavior directly we can inspect the disassembly of the CGB bootrom and infer the following: 

{{#bits 8 >
	"KEY0" 2:"DMG compatibility mode"
}}

- **DMG compatibility mode**: `0` = Disabled (full CGB mode, for regular CGB cartridges), `1` = Enabled (for DMG only cartridges)

#### PGB mode

:::tip Research needed

It has been speculated that setting bit 3 is related to a special mode called "PGB" for controlling the LCD externally.

This mode is not well researched nor documented yet, you are welcome to help [here!](https://github.com/gbdev/pandocs/issues/581)

:::

### FF6C — OPRI (CGB Mode only): Object priority mode

This register serves as a flag for which object priority mode to use. While
the DMG prioritizes objects by x-coordinate, the CGB prioritizes them by
location in OAM. This flag is set by the CGB bios after checking the game's CGB compatibility.

OPRI has an effect if a [PGB](<#PGB mode>) value (`0xX8`, `0xXC`) is written to [KEY0](<#FF4C — KEY0/SYS (CGB Mode only): CPU mode select>) but STOP hasn't been executed yet, and the write takes effect instantly.

:::warning TO BE VERIFIED

It does not have an effect, at least not an instant effect, if written to during CGB or DMG mode after the boot ROM has been unmapped.
It is not known if triggering a PSM NMI, which remaps the boot ROM, has an effect on this register's behavior.

:::

{{#bits 8 >
	"OPRI" 0:"Priority mode"
}}

- **Priority mode** (*Read/Write*): `0` = CGB-style priority, `1` = DMG-style priority

### FF70 — SVBK/WBK (CGB Mode only): WRAM bank

In CGB Mode, 32 KiB of internal RAM are available.
This memory is divided into 8 banks of 4 KiB each.
Bank 0 is always available in memory at C000–CFFF, banks 1–7 can be selected into the address space at D000–DFFF.

{{#bits 8 >
	"SVBK" 2-0:"WRAM bank"
}}

- **WRAM bank** (*Read/Write*): Writing a value will map the corresponding bank to [D000–DFFF](<Memory_Map.md#Memory Map>), except 0, which maps bank 1 instead.

## Undocumented registers

These are undocumented CGB Registers. The purpose of these registers is
unknown (if any). It isn't recommended to use them in your software,
but you could, for example, use them to check if you are running on an
emulator or on DMG hardware.

### FF72–FF73 — Bits 0–7 (CGB Mode only)

Both of these registers are fully read/write.
Their initial value is $00.

### FF74 — Bits 0–7 (CGB Mode only)

In CGB mode, this register is fully readable and writable.
Its initial value is $00.

Otherwise, this register is read-only, and locked at value $FF.

### FF75 — Bits 4–6 (CGB Mode only)

Only bits 4, 5 and 6 of this register are read/write enabled.
Their initial value is 0.

[`SC`]: <#FF02 — SC: Serial transfer control>
[`LCDC`'s bit 0]: <#LCDC.0 — BG and Window enable/priority>
[trivia about the colorisation process]: <#GBC Approval Process>
[left by the boot ROM]: <#CPU registers>
[LCD differences]: <#RGB Translation by GBAs>
