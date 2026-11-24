
# Hardware Registers

Address    | Name               | Description                                                       | Readable / Writable | Models
-----------|--------------------|-------------------------------------------------------------------|---------------------|-------
$FF00      | [`JOYP`]           | Joypad                                                            | Mixed               | All
$FF01      | [`SB`]             | Serial transfer data                                              | R/W                 | All
$FF02      | [`SC`]             | Serial transfer control                                           | R/W                 | Mixed
$FF04      | [`DIV`]            | Divider register                                                  | R/W                 | All
$FF05      | [`TIMA`]           | Timer counter                                                     | R/W                 | All
$FF06      | [`TMA`]            | Timer modulo                                                      | R/W                 | All
$FF07      | [`TAC`]            | Timer control                                                     | R/W                 | All
$FF0F      | [`IF`]             | Interrupt flag                                                    | R/W                 | All
$FF10      | [`AUD1SWEEP`]      | Sound channel 1 sweep                                             | R/W                 | All
$FF11      | [`AUD1LEN`]        | Sound channel 1 length timer & duty cycle                         | Mixed               | All
$FF12      | [`AUD1ENV`]        | Sound channel 1 volume & envelope                                 | R/W                 | All
$FF13      | [`AUD1LOW`]        | Sound channel 1 period low                                        | W                   | All
$FF14      | [`AUD1HIGH`]       | Sound channel 1 period high & control                             | Mixed               | All
$FF16      | [`AUD2LEN`]        | Sound channel 2 length timer & duty cycle                         | Mixed               | All
$FF17      | [`AUD2ENV`]        | Sound channel 2 volume & envelope                                 | R/W                 | All
$FF18      | [`AUD2LOW`]        | Sound channel 2 period low                                        | W                   | All
$FF19      | [`AUD2HIGH`]       | Sound channel 2 period high & control                             | Mixed               | All
$FF1A      | [`AUD3ENA`]        | Sound channel 3 DAC enable                                        | R/W                 | All
$FF1B      | [`AUD3LEN`]        | Sound channel 3 length timer                                      | W                   | All
$FF1C      | [`AUD3LEVEL`]      | Sound channel 3 output level                                      | R/W                 | All
$FF1D      | [`AUD3LOW`]        | Sound channel 3 period low                                        | W                   | All
$FF1E      | [`AUD3HIGH`]       | Sound channel 3 period high & control                             | Mixed               | All
$FF20      | [`AUD4LEN`]        | Sound channel 4 length timer                                      | W                   | All
$FF21      | [`AUD4ENV`]        | Sound channel 4 volume & envelope                                 | R/W                 | All
$FF22      | [`AUD4POLY`]       | Sound channel 4 frequency & randomness                            | R/W                 | All
$FF23      | [`AUD4GO`]         | Sound channel 4 control                                           | Mixed               | All
$FF24      | [`AUDVOL`]         | Master volume & VIN panning                                       | R/W                 | All
$FF25      | [`AUDTERM`]        | Sound panning                                                     | R/W                 | All
$FF26      | [`AUDENA`]         | Sound on/off                                                      | Mixed               | All
$FF30-FF3F | [Wave RAM]         | Storage for one of the sound channels' waveform                   | R/W                 | All
$FF40      | [`LCDC`]           | LCD control                                                       | R/W                 | All
$FF41      | [`STAT`]           | LCD status                                                        | Mixed               | All
$FF42      | [`SCY`]            | Viewport Y position                                               | R/W                 | All
$FF43      | [`SCX`]            | Viewport X position                                               | R/W                 | All
$FF44      | [`LY`]             | LCD Y coordinate                                                  | R                   | All
$FF45      | [`LYC`]            | LY compare                                                        | R/W                 | All
$FF46      | [`DMA`]            | OAM DMA source address & start                                    | R/W                 | All
$FF47      | [`BGP`]            | BG palette data                                                   | R/W                 | DMG
$FF48      | [`OBP0`]           | OBJ palette 0 data                                                | R/W                 | DMG
$FF49      | [`OBP1`]           | OBJ palette 1 data                                                | R/W                 | DMG
$FF4A      | [`WY`]             | Window Y position                                                 | R/W                 | All
$FF4B      | [`WX`]             | Window X position plus 7                                          | R/W                 | All
$FF4C      | [`SYS`]            | CPU mode select                                                   | Mixed               | CGB
$FF4D      | [`SPD`]            | Prepare speed switch                                              | Mixed               | CGB
$FF4F      | [`VBK`]            | VRAM bank                                                         | R/W                 | CGB
$FF50      | [`BANK`]           | Boot ROM mapping control                                          | W                   | All
$FF51      | [`VDMA_SRC_HIGH`]  | VRAM DMA source high                                              | W                   | CGB
$FF52      | [`VDMA_SRC_LOW`]   | VRAM DMA source low                                               | W                   | CGB
$FF53      | [`VDMA_DEST_HIGH`] | VRAM DMA destination high                                         | W                   | CGB
$FF54      | [`VDMA_DEST_LOW`]  | VRAM DMA destination low                                          | W                   | CGB
$FF55      | [`VDMA_LEN`]       | VRAM DMA length/mode/start                                        | R/W                 | CGB
$FF56      | [`RP`]             | Infrared communications port                                      | Mixed               | CGB
$FF68      | [`BGPI`]      | Background color palette specification / Background palette index | R/W                 | CGB
$FF69      | [`BGPD`]      | Background color palette data / Background palette data           | R/W                 | CGB
$FF6A      | [`OBPI`]      | OBJ color palette specification / OBJ palette index               | R/W                 | CGB
$FF6B      | [`OBPD`]      | OBJ color palette data / OBJ palette data                         | R/W                 | CGB
$FF6C      | [`OPRI`]           | Object priority mode                                              | R/W                 | CGB
$FF70      | [`WBK`]            | WRAM bank                                                         | R/W                 | CGB
$FF76      | [`PCM12`]          | Audio digital outputs 1 & 2                                       | R                   | CGB
$FF77      | [`PCM34`]          | Audio digital outputs 3 & 4                                       | R                   | CGB
$FFFF      | [`IE`]             | Interrupt enable                                                  | R/W                 | All

[`JOYP`]: <#FF00 — JOYP: Joypad>
[`SB`]: <#FF01 — SB: Serial transfer data>
[`SC`]: <#FF02 — SC: Serial transfer control>
[`DIV`]: <#FF04 — DIV: Divider register>
[`TIMA`]: <#FF05 — TIMA: Timer counter>
[`TMA`]: <#FF06 — TMA: Timer modulo>
[`TAC`]: <#FF07 — TAC: Timer control>
[`IF`]: <#FF0F — IF: Interrupt flag>
[`AUD1SWEEP`]: <#FF10 — AUD1SWEEP: Channel 1 sweep>
[`AUD1LEN`]: <#FF11 — AUD1LEN: Channel 1 length timer & duty cycle>
[`AUD1ENV`]: <#FF12 — AUD1ENV: Channel 1 volume & envelope>
[`AUD1LOW`]: <#FF13 — AUD1LOW: Channel 1 period low \[write-only\]>
[`AUD1HIGH`]: <#FF14 — AUD1HIGH: Channel 1 period high & control>
[`AUD2LEN`]: <#Sound Channel 2 — Pulse>
[`AUD2ENV`]: <#Sound Channel 2 — Pulse>
[`AUD2LOW`]: <#Sound Channel 2 — Pulse>
[`AUD2HIGH`]: <#Sound Channel 2 — Pulse>
[`AUD3ENA`]: <#FF1A — AUD3ENA: Channel 3 DAC enable>
[`AUD3LEN`]: <#FF1B — AUD3LEN: Channel 3 length timer \[write-only\]>
[`AUD3LEVEL`]: <#FF1C — AUD3LEVEL: Channel 3 output level>
[`AUD3LOW`]: <#FF1D — AUD3LOW: Channel 3 period low \[write-only\]>
[`AUD3HIGH`]: <#FF1E — AUD3HIGH: Channel 3 period high & control>
[`AUD4LEN`]: <#FF20 — AUD4LEN: Channel 4 length timer \[write-only\]>
[`AUD4ENV`]: <#FF21 — AUD4ENV: Channel 4 volume & envelope>
[`AUD4POLY`]: <#FF22 — AUD4POLY: Channel 4 frequency & randomness>
[`AUD4GO`]: <#FF23 — AUD4GO: Channel 4 control>
[`AUDVOL`]: <#FF24 — AUDVOL: Master volume & VIN panning>
[`AUDTERM`]: <#FF25 — AUDTERM: Sound panning>
[`AUDENA`]: <#FF26 — AUDENA: Audio master control>
[`Wave RAM`]: <#FF30–FF3F — Wave pattern RAM>
[`LCDC`]: <#FF40 — LCDC: LCD control>
[`STAT`]: <#FF41 — STAT: LCD status>
[`SCY`]: <#FF42–FF43 — SCY, SCX: Background viewport Y position, X position>
[`SCX`]: <#FF42–FF43 — SCY, SCX: Background viewport Y position, X position>
[`LY`]: <#FF44 — LY: LCD Y coordinate \[read-only\]>
[`LYC`]: <#FF45 — LYC: LY compare>
[`DMA`]: <#FF46 — DMA: OAM DMA source address & start>
[`BGP`]: <#FF47 — BGP (Non-CGB Mode only): BG palette data>
[`OBP0`]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[`OBP1`]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[`WY`]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[`WX`]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[`SYS`]: <#FF4C — SYS (CGB Mode only): CPU mode select>
[`SPD`]: <#FF4D — SPD (CGB Mode only): Prepare speed switch>
[`VBK`]: <#FF4F — VBK (CGB Mode only): VRAM bank>
[`BANK`]: <#Power-Up Sequence>
[`VDMA_SRC_HIGH`]: <#FF51–FF52 — VDMA_SRC_HIGH, VDMA_SRC_LOW (CGB Mode only): VRAM DMA source address \[write-only\]>
[`VDMA_SRC_LOW`]: <#FF51–FF52 — VDMA_SRC_HIGH, VDMA_SRC_LOW (CGB Mode only): VRAM DMA source address \[write-only\]>
[`VDMA_DEST_HIGH`]: <#FF53–FF54 — VDMA_DEST_HIGH, VDMA_DEST_LOW (CGB Mode only): VRAM DMA destination address \[write-only\]>
[`VDMA_DEST_LOW`]: <#FF53–FF54 — VDMA_DEST_HIGH, VDMA_DEST_LOW (CGB Mode only): VRAM DMA destination address \[write-only\]>
[`VDMA_LEN`]: <#FF55 — VDMA_LEN (CGB Mode only): VRAM DMA length/mode/start>
[`RP`]: <#FF56 — RP (CGB Mode only): Infrared communications port>
[`BGPI`]: <#FF68 — BGPI (CGB Mode only): Background palette index>
[`BGPD`]: <#FF69 — BGPD (CGB Mode only): Background palette data>
[`OBPI`]: <#FF6A–FF6B — OBPI, OBPD (CGB Mode only): OBJ palette index, OBJ palette data>
[`OBPD`]: <#FF6A–FF6B — OBPI, OBPD (CGB Mode only): OBJ palette index, OBJ palette data>
[`OPRI`]: <#FF6C — OPRI (CGB Mode only): Object priority mode>
[`WBK`]: <#FF70 — WBK (CGB Mode only): WRAM bank>
[`PCM12`]: <#FF76 — PCM12 (CGB Mode only): Digital outputs 1 & 2 \[read-only\]>
[`PCM34`]: <#FF77 — PCM34 (CGB Mode only): Digital outputs 3 & 4 \[read-only\]>
[`IE`]: <#FFFF — IE: Interrupt enable>
