
# Hardware Registers

Address    | Name        | Description                                                       | Readable / Writable | Models
-----------|-------------|-------------------------------------------------------------------|---------------------|-------
$FF00      | [P1/JOYP]   | Joypad                                                            | Mixed               | All
$FF01      | [SB]        | Serial transfer data                                              | R/W                 | All
$FF02      | [SC]        | Serial transfer control                                           | R/W                 | Mixed
$FF04      | [DIV]       | Divider register                                                  | R/W                 | All
$FF05      | [TIMA]      | Timer counter                                                     | R/W                 | All
$FF06      | [TMA]       | Timer modulo                                                      | R/W                 | All
$FF07      | [TAC]       | Timer control                                                     | R/W                 | All
$FF0F      | [IF]        | Interrupt flag                                                    | R/W                 | All
$FF10      | [NR10]      | Sound channel 1 sweep                                             | R/W                 | All
$FF11      | [NR11]      | Sound channel 1 length timer & duty cycle                         | Mixed               | All
$FF12      | [NR12]      | Sound channel 1 volume & envelope                                 | R/W                 | All
$FF13      | [NR13]      | Sound channel 1 period low                                        | W                   | All
$FF14      | [NR14]      | Sound channel 1 period high & control                             | Mixed               | All
$FF16      | [NR21]      | Sound channel 2 length timer & duty cycle                         | Mixed               | All
$FF17      | [NR22]      | Sound channel 2 volume & envelope                                 | R/W                 | All
$FF18      | [NR23]      | Sound channel 2 period low                                        | W                   | All
$FF19      | [NR24]      | Sound channel 2 period high & control                             | Mixed               | All
$FF1A      | [NR30]      | Sound channel 3 DAC enable                                        | R/W                 | All
$FF1B      | [NR31]      | Sound channel 3 length timer                                      | W                   | All
$FF1C      | [NR32]      | Sound channel 3 output level                                      | R/W                 | All
$FF1D      | [NR33]      | Sound channel 3 period low                                        | W                   | All
$FF1E      | [NR34]      | Sound channel 3 period high & control                             | Mixed               | All
$FF20      | [NR41]      | Sound channel 4 length timer                                      | W                   | All
$FF21      | [NR42]      | Sound channel 4 volume & envelope                                 | R/W                 | All
$FF22      | [NR43]      | Sound channel 4 frequency & randomness                            | R/W                 | All
$FF23      | [NR44]      | Sound channel 4 control                                           | Mixed               | All
$FF24      | [NR50]      | Master volume & VIN panning                                       | R/W                 | All
$FF25      | [NR51]      | Sound panning                                                     | R/W                 | All
$FF26      | [NR52]      | Sound on/off                                                      | Mixed               | All
$FF30-FF3F | [Wave RAM]  | Storage for one of the sound channels' waveform                   | R/W                 | All
$FF40      | [LCDC]      | LCD control                                                       | R/W                 | All
$FF41      | [STAT]      | LCD status                                                        | Mixed               | All
$FF42      | [SCY]       | Viewport Y position                                               | R/W                 | All
$FF43      | [SCX]       | Viewport X position                                               | R/W                 | All
$FF44      | [LY]        | LCD Y coordinate                                                  | R                   | All
$FF45      | [LYC]       | LY compare                                                        | R/W                 | All
$FF46      | [DMA]       | OAM DMA source address & start                                    | R/W                 | All
$FF47      | [BGP]       | BG palette data                                                   | R/W                 | DMG
$FF48      | [OBP0]      | OBJ palette 0 data                                                | R/W                 | DMG
$FF49      | [OBP1]      | OBJ palette 1 data                                                | R/W                 | DMG
$FF4A      | [WY]        | Window Y position                                                 | R/W                 | All
$FF4B      | [WX]        | Window X position plus 7                                          | R/W                 | All
$FF4D      | [KEY1]      | Prepare speed switch                                              | Mixed               | CGB
$FF4F      | [VBK]       | VRAM bank                                                         | R/W                 | CGB
$FF51      | [HDMA1]     | VRAM DMA source high                                              | W                   | CGB
$FF52      | [HDMA2]     | VRAM DMA source low                                               | W                   | CGB
$FF53      | [HDMA3]     | VRAM DMA destination high                                         | W                   | CGB
$FF54      | [HDMA4]     | VRAM DMA destination low                                          | W                   | CGB
$FF55      | [HDMA5]     | VRAM DMA length/mode/start                                        | R/W                 | CGB
$FF56      | [RP]        | Infrared communications port                                      | Mixed               | CGB
$FF68      | [BCPS/BGPI] | Background color palette specification / Background palette index | R/W                 | CGB
$FF69      | [BCPD/BGPD] | Background color palette data / Background palette data           | R/W                 | CGB
$FF6A      | [OCPS/OBPI] | OBJ color palette specification / OBJ palette index               | R/W                 | CGB
$FF6B      | [OCPD/OBPD] | OBJ color palette data / OBJ palette data                         | R/W                 | CGB
$FF6C      | [OPRI]      | Object priority mode                                              | R/W                 | CGB
$FF70      | [SVBK]      | WRAM bank                                                         | R/W                 | CGB
$FF76      | [PCM12]     | Audio digital outputs 1 & 2                                       | R                   | CGB
$FF77      | [PCM34]     | Audio digital outputs 3 & 4                                       | R                   | CGB
$FFFF      | [IE]        | Interrupt enable                                                  | R/W                 | All

[P1/JOYP]: <#FF00 — P1/JOYP: Joypad>
[SB]: <#FF01 — SB: Serial transfer data>
[SC]: <#FF02 — SC: Serial transfer control>
[DIV]: <#FF04 — DIV: Divider register>
[TIMA]: <#FF05 — TIMA: Timer counter>
[TMA]: <#FF06 — TMA: Timer modulo>
[TAC]: <#FF07 — TAC: Timer control>
[IF]: <#FF0F — IF: Interrupt flag>
[NR10]: <#FF10 — NR10: Channel 1 sweep>
[NR11]: <#FF11 — NR11: Channel 1 length timer & duty cycle>
[NR12]: <#FF12 — NR12: Channel 1 volume & envelope>
[NR13]: <#FF13 — NR13: Channel 1 period low \[write-only\]>
[NR14]: <#FF14 — NR14: Channel 1 period high & control>
[NR21]: <#Sound Channel 2 — Pulse>
[NR22]: <#Sound Channel 2 — Pulse>
[NR23]: <#Sound Channel 2 — Pulse>
[NR24]: <#Sound Channel 2 — Pulse>
[NR30]: <#FF1A — NR30: Channel 3 DAC enable>
[NR31]: <#FF1B — NR31: Channel 3 length timer \[write-only\]>
[NR32]: <#FF1C — NR32: Channel 3 output level>
[NR33]: <#FF1D — NR33: Channel 3 period low \[write-only\]>
[NR34]: <#FF1E — NR34: Channel 3 period high & control>
[NR41]: <#FF20 — NR41: Channel 4 length timer \[write-only\]>
[NR42]: <#FF21 — NR42: Channel 4 volume & envelope>
[NR43]: <#FF22 — NR43: Channel 4 frequency & randomness>
[NR44]: <#FF23 — NR44: Channel 4 control>
[NR50]: <#FF24 — NR50: Master volume & VIN panning>
[NR51]: <#FF25 — NR51: Sound panning>
[NR52]: <#FF26 — NR52: Audio master control>
[Wave RAM]: <#FF30–FF3F — Wave pattern RAM>
[LCDC]: <#FF40 — LCDC: LCD control>
[STAT]: <#FF41 — STAT: LCD status>
[SCY]: <#FF42–FF43 — SCY, SCX: Background viewport Y position, X position>
[SCX]: <#FF42–FF43 — SCY, SCX: Background viewport Y position, X position>
[LY]: <#FF44 — LY: LCD Y coordinate \[read-only\]>
[LYC]: <#FF45 — LYC: LY compare>
[DMA]: <#FF46 — DMA: OAM DMA source address & start>
[BGP]: <#FF47 — BGP (Non-CGB Mode only): BG palette data>
[OBP0]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[OBP1]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[WY]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[WX]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[KEY1]: <#FF4D — KEY1 (CGB Mode only): Prepare speed switch>
[VBK]: <#FF4F — VBK (CGB Mode only): VRAM bank>
[HDMA1]: <#FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]>
[HDMA2]: <#FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]>
[HDMA3]: <#FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]>
[HDMA4]: <#FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]>
[HDMA5]: <#FF55 — HDMA5 (CGB Mode only): VRAM DMA length/mode/start>
[RP]: <#FF56 — RP (CGB Mode only): Infrared communications port>
[BCPS/BGPI]: <#FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index>
[BCPD/BGPD]: <#FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data>
[OCPS/OBPI]: <#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>
[OCPD/OBPD]: <#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>
[OPRI]: <#FF6C — OPRI (CGB Mode only): Object priority mode>
[SVBK]: <#FF70 — SVBK (CGB Mode only): WRAM bank>
[PCM12]: <#FF76 — PCM12 (CGB Mode only): Digital outputs 1 & 2 \[read-only\]>
[PCM34]: <#FF77 — PCM34 (CGB Mode only): Digital outputs 3 & 4 \[read-only\]>
[IE]: <#FFFF — IE: Interrupt enable>
