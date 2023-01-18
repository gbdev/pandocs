
# Hardware Registers

Address    | Name                        | Description                                                       | Readable / Writable
:----------|-----------------------------|-------------------------------------------------------------------|----
$FF00      | [P1/JOYP]                   | Joypad                                                            | R/W
$FF01      | [SB]                        | Serial transfer data                                              | R/W
$FF02      | [SC]                        | Serial transfer control                                           | R/W
$FF04      | [DIV]                       | Divider register                                                  | R/W
$FF05      | [TIMA]                      | Timer counter                                                     | R/W
$FF06      | [TMA]                       | Timer modulo                                                      | R/W
$FF07      | [TAC]                       | Timer control                                                     | R/W
$FF0F      | [IF]                        | Interrupt flag                                                    | R/W
$FF10      | [NR10]                      | Channel 1 sweep                                                   | R/W
$FF11      | [NR11]                      | Channel 1 length timer & duty cycle                               | R/W
$FF12      | [NR12]                      | Channel 1 volume & envelope                                       | R/W
$FF13      | [NR13]                      | Channel 1 wavelength low                                          | W  
$FF14      | [NR14]                      | Channel 1 wavelength high & control                               | R/W
$FF16      | [NR21]                      | Channel 2 length timer & duty cycle                               | R/W
$FF17      | [NR22]                      | Channel 2 volume & envelope                                       | R/W
$FF18      | [NR23]                      | Channel 2 wavelength low                                          | W  
$FF19      | [NR24]                      | Channel 2 wavelength high & control                               | R/W
$FF1A      | [NR30]                      | Channel 3 DAC enable                                              | R/W
$FF1B      | [NR31]                      | Channel 3 length timer                                            | W  
$FF1C      | [NR32]                      | Channel 3 output level                                            | R/W
$FF1D      | [NR33]                      | Channel 3 wavelength low                                          | W  
$FF1E      | [NR34]                      | Channel 3 wavelength high & control                               | R/W
$FF20      | [NR41]                      | Channel 4 length timer                                            | W  
$FF21      | [NR42]                      | Channel 4 volume & envelope                                       | R/W
$FF22      | [NR43]                      | Channel 4 frequency & randomness                                  | R/W
$FF23      | [NR44]                      | Channel 4 control                                                 | R/W
$FF24      | [NR50]                      | Master volume & VIN panning                                       | R/W
$FF25      | [NR51]                      | Sound panning                                                     | R/W
$FF26      | [NR52]                      | Sound on/off                                                      | R/W
$FF30-FF3F | [Wave pattern ram]          | Wave pattern RAM                                                  | R/W
$FF40      | [LCDC]                      | LCD control                                                       | R/W
$FF41      | [STAT]                      | LCD status                                                        | R/W
$FF42      | [SCY]                       | Viewport Y position                                               | R/W
$FF43      | [SCX]                       | Viewport X position                                               | R/W
$FF44      | [LY]                        | LCD Y coordinate                                                  | R  
$FF45      | [LYC]                       | LY compare                                                        | R/W
$FF46      | [DMA]                       | OAM DMA source address & start                                    | R/W
$FF47      | [BGP (Non-CGB Mode only)]   | BG palette data                                                   | R/W
$FF48      | [OBP0 (Non-CGB Mode only)]  | OBJ palette 0 data                                                | R/W
$FF49      | [OBP1 (Non-CGB Mode only)]  | OBJ palette 1 data                                                | R/W
$FF4A      | [WY]                        | Window Y position                                                 | R/W
$FF4B      | [WX]                        | Window X position plus 7                                          | R/W
$FF4D      | [KEY1 (CGB Mode only)]      | Prepare speed switch                                              | R/W
$FF4F      | [VBK (CGB Mode only)]       | VRAM bank                                                         | R/W
$FF51      | [HDMA1 (CGB Mode only)]     | VRAM DMA source high                                              | W  
$FF52      | [HDMA2 (CGB Mode only)]     | VRAM DMA source low                                               | W  
$FF53      | [HDMA3 (CGB Mode only)]     | VRAM DMA destination high                                         | W  
$FF54      | [HDMA4 (CGB Mode only)]     | VRAM DMA destination low                                          | W  
$FF55      | [HDMA5 (CGB Mode only)]     | VRAM DMA length/mode/start                                        | R/W
$FF56      | [RP (CGB Mode only)]        | Infrared communications port                                      | R/W
$FF68      | [BCPS/BGPI (CGB Mode only)] | Background color palette specification / Background palette index | R/W
$FF69      | [BCPD/BGPD (CGB Mode only)] | Background color palette data / Background palette data           | R/W
$FF6A      | [OCPS/OBPI (CGB Mode only)] | OBJ color palette specification / OBJ palette index               | R/W
$FF6B      | [OCPD/OBPD (CGB Mode only)] | OBJ color palette data / OBJ palette data                         | R/W
$FF6C      | [OPRI (CGB Mode only)]      | Object priority mode                                              | R/W
$FF70      | [SVBK (CGB Mode only)]      | WRAM bank                                                         | R/W
$FF76      | [PCM12 (CGB Mode only)]     | Digital outputs 1 & 2                                             | R  
$FF77      | [PCM34 (CGB Mode only)]     | Digital outputs 3 & 4                                             | R  
$FFFF      | [IE]                        | Interrupt enable                                                  | R/W

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
[NR13]: <#FF13 — NR13: Channel 1 wavelength low \[write-only\]>
[NR14]: <#FF14 — NR14: Channel 1 wavelength high & control>
[NR21]: <#Sound Channel 2 — Pulse>
[NR22]: <#Sound Channel 2 — Pulse>
[NR23]: <#Sound Channel 2 — Pulse>
[NR24]: <#Sound Channel 2 — Pulse>
[NR30]: <#FF1A — NR30: Channel 3 DAC enable>
[NR31]: <#FF1B — NR31: Channel 3 length timer \[write-only\]>
[NR32]: <#FF1C — NR32: Channel 3 output level>
[NR33]: <#FF1D — NR33: Channel 3 wavelength low \[write-only\]>
[NR34]: <#FF1E — NR34: Channel 3 wavelength high & control>
[NR41]: <#FF20 — NR41: Channel 4 length timer \[write-only\]>
[NR42]: <#FF21 — NR42: Channel 4 volume & envelope>
[NR43]: <#FF22 — NR43: Channel 4 frequency & randomness>
[NR44]: <#FF23 — NR44: Channel 4 control>
[NR50]: <#FF24 — NR50: Master volume & VIN panning>
[NR51]: <#FF25 — NR51: Sound panning>
[NR52]: <#FF26 — NR52: Sound on/off>
[Wave pattern RAM]: <#FF30–FF3F — Wave pattern RAM>
[LCDC]: <#FF40 — LCDC: LCD control>
[STAT]: <#FF41 — STAT: LCD status>
[SCY]: <#FF42–FF43 — SCY, SCX: Viewport Y position, X position>
[SCX]: <#FF42–FF43 — SCY, SCX: Viewport Y position, X position>
[LY]: <#FF44 — LY: LCD Y coordinate \[read-only\]>
[LYC]: <#FF45 — LYC: LY compare>
[DMA]: <#FF46 — DMA: OAM DMA source address & start>
[BGP (Non-CGB Mode only)]: <#FF47 — BGP (Non-CGB Mode only): BG palette data>
[OBP0 (Non-CGB Mode only)]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[OBP1 (Non-CGB Mode only)]: <#FF48–FF49 — OBP0, OBP1 (Non-CGB Mode only): OBJ palette 0, 1 data>
[WY]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[WX]: <#FF4A–FF4B — WY, WX: Window Y position, X position plus 7>
[KEY1 (CGB Mode only)]: <#FF4D — KEY1 (CGB Mode only): Prepare speed switch>
[VBK (CGB Mode only)]: <#FF4F — VBK (CGB Mode only): VRAM bank>
[HDMA1 (CGB Mode only)]: <#FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]>
[HDMA2 (CGB Mode only)]: <#FF51–FF52 — HDMA1, HDMA2 (CGB Mode only): VRAM DMA source (high, low) \[write-only\]>
[HDMA3 (CGB Mode only)]: <#FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]>
[HDMA4 (CGB Mode only)]: <#FF53–FF54 — HDMA3, HDMA4 (CGB Mode only): VRAM DMA destination (high, low) \[write-only\]>
[HDMA5 (CGB Mode only)]: <#FF55 — HDMA5 (CGB Mode only): VRAM DMA length/mode/start>
[RP (CGB Mode only)]: <#FF56 — RP (CGB Mode only): Infrared communications port>
[BCPS/BGPI (CGB Mode only)]: <#FF68 — BCPS/BGPI (CGB Mode only): Background color palette specification / Background palette index>
[BCPD/BGPD (CGB Mode only)]: <#FF69 — BCPD/BGPD (CGB Mode only): Background color palette data / Background palette data>
[OCPS/OBPI (CGB Mode only)]: <#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>
[OCPD/OBPD (CGB Mode only)]: <#FF6A–FF6B — OCPS/OBPI, OCPD/OBPD (CGB Mode only): OBJ color palette specification / OBJ palette index, OBJ color palette data / OBJ palette data>
[OPRI (CGB Mode only)]: <#FF6C — OPRI (CGB Mode only): Object priority mode>
[SVBK (CGB Mode only)]: <#FF70 — SVBK (CGB Mode only): WRAM bank>
[PCM12 (CGB Mode only)]: <#FF76 — PCM12 (CGB Mode only): Digital outputs 1 & 2 \[read-only\]>
[PCM34 (CGB Mode only)]: <#FF77 — PCM34 (CGB Mode only): Digital outputs 3 & 4 \[read-only\]>
[IE]: <#FFFF — IE: Interrupt enable>
