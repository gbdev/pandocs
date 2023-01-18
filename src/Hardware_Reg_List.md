
# Hardware Registers

| Addr       | Name                                                       | Description                                                       | R/W |
|:-----------|------------------------------------------------------------|-------------------------------------------------------------------|-----|
| $FF00      | [P1/JOYP][linkP1/JOYP]                                     | Joypad                                                            | R/W |
| $FF01      | [SB][linkSB]                                               | Serial transfer data                                              | R/W |
| $FF02      | [SC][linkSC]                                               | Serial transfer control                                           | R/W |
| $FF04      | [DIV][linkDIV]                                             | Divider register                                                  | R/W |
| $FF05      | [TIMA][linkTIMA]                                           | Timer counter                                                     | R/W |
| $FF06      | [TMA][linkTMA]                                             | Timer modulo                                                      | R/W |
| $FF07      | [TAC][linkTAC]                                             | Timer control                                                     | R/W |
| $FF0F      | [IF][linkIF]                                               | Interrupt flag                                                    | R/W |
| $FF10      | [NR10][linkNR10]                                           | Channel 1 sweep                                                   | R/W |
| $FF11      | [NR11][linkNR11]                                           | Channel 1 length timer & duty cycle                               | R/W |
| $FF12      | [NR12][linkNR12]                                           | Channel 1 volume & envelope                                       | R/W |
| $FF13      | [NR13][linkNR13]                                           | Channel 1 wavelength low                                          | W   |
| $FF14      | [NR14][linkNR14]                                           | Channel 1 wavelength high & control                               | R/W |
| $FF16      | [NR21][linkNR21]                                           | Channel 2 length timer & duty cycle                               | R/W |
| $FF17      | [NR22][linkNR22]                                           | Channel 2 volume & envelope                                       | R/W |
| $FF18      | [NR23][linkNR23]                                           | Channel 2 wavelength low                                          | W   |
| $FF19      | [NR24][linkNR24]                                           | Channel 2 wavelength high & control                               | R/W |
| $FF1A      | [NR30][linkNR30]                                           | Channel 3 DAC enable                                              | R/W |
| $FF1B      | [NR31][linkNR31]                                           | Channel 3 length timer                                            | W   |
| $FF1C      | [NR32][linkNR32]                                           | Channel 3 output level                                            | R/W |
| $FF1D      | [NR33][linkNR33]                                           | Channel 3 wavelength low                                          | W   |
| $FF1E      | [NR34][linkNR34]                                           | Channel 3 wavelength high & control                               | R/W |
| $FF20      | [NR41][linkNR41]                                           | Channel 4 length timer                                            | W   |
| $FF21      | [NR42][linkNR42]                                           | Channel 4 volume & envelope                                       | R/W |
| $FF22      | [NR43][linkNR43]                                           | Channel 4 frequency & randomness                                  | R/W |
| $FF23      | [NR44][linkNR44]                                           | Channel 4 control                                                 | R/W |
| $FF24      | [NR50][linkNR50]                                           | Master volume & VIN panning                                       | R/W |
| $FF25      | [NR51][linkNR51]                                           | Sound panning                                                     | R/W |
| $FF26      | [NR52][linkNR52]                                           | Sound on/off                                                      | R/W |
| $FF30-FF3F | [Wave pattern ram][linkWave pattern ram]                   | Wave pattern RAM                                                  | R/W |
| $FF40      | [LCDC][linkLCDC]                                           | LCD control                                                       | R/W |
| $FF41      | [STAT][linkSTAT]                                           | LCD status                                                        | R/W |
| $FF42      | [SCY][linkSCY]                                             | Viewport Y position                                               | R/W |
| $FF43      | [SCX][linkSCX]                                             | Viewport X position                                               | R/W |
| $FF44      | [LY][linkLY]                                               | LCD Y coordinate                                                  | R   |
| $FF45      | [LYC][linkLYC]                                             | LY compare                                                        | R/W |
| $FF46      | [DMA][linkDMA]                                             | OAM DMA source address & start                                    | R/W |
| $FF47      | [BGP (Non-CGB Mode only)][linkBGP (Non-CGB Mode only)]     | BG palette data                                                   | R/W |
| $FF48      | [OBP0 (Non-CGB Mode only)][linkOBP0 (Non-CGB Mode only)]   | OBJ palette 0 data                                                | R/W |
| $FF49      | [OBP1 (Non-CGB Mode only)][linkOBP1 (Non-CGB Mode only)]   | OBJ palette 1 data                                                | R/W |
| $FF4A      | [WY][linkWY]                                               | Window Y position                                                 | R/W |
| $FF4B      | [WX][linkWX]                                               | Window X position plus 7                                          | R/W |
| $FF4D      | [KEY1 (CGB Mode only)][linkKEY1 (CGB Mode only)]           | Prepare speed switch                                              | R/W |
| $FF4F      | [VBK (CGB Mode only)][linkVBK (CGB Mode only)]             | VRAM bank                                                         | R/W |
| $FF51      | [HDMA1 (CGB Mode only)][linkHDMA1 (CGB Mode only)]         | VRAM DMA source high                                              | W   |
| $FF52      | [HDMA2 (CGB Mode only)][linkHDMA2 (CGB Mode only)]         | VRAM DMA source low                                               | W   |
| $FF53      | [HDMA3 (CGB Mode only)][linkHDMA3 (CGB Mode only)]         | VRAM DMA destination high                                         | W   |
| $FF54      | [HDMA4 (CGB Mode only)][linkHDMA4 (CGB Mode only)]         | VRAM DMA destination low                                          | W   |
| $FF55      | [HDMA5 (CGB Mode only)][linkHDMA5 (CGB Mode only)]         | VRAM DMA length/mode/start                                        | R/W |
| $FF56      | [RP (CGB Mode only)][linkRP (CGB Mode only)]               | Infrared communications port                                      | R/W |
| $FF68      | [BCPS/BGPI (CGB Mode only)][linkBCPS/BGPI (CGB Mode only)] | Background color palette specification / Background palette index | R/W |
| $FF69      | [BCPD/BGPD (CGB Mode only)][linkBCPD/BGPD (CGB Mode only)] | Background color palette data / Background palette data           | R/W |
| $FF6A      | [OCPS/OBPI (CGB Mode only)][linkOCPS/OBPI (CGB Mode only)] | OBJ color palette specification / OBJ palette index               | R/W |
| $FF6B      | [OCPD/OBPD (CGB Mode only)][linkOCPD/OBPD (CGB Mode only)] | OBJ color palette data / OBJ palette data                         | R/W |
| $FF6C      | [OPRI (CGB Mode only)][linkOPRI (CGB Mode only)]           | Object priority mode                                              | R/W |
| $FF70      | [SVBK (CGB Mode only)][linkSVBK (CGB Mode only)]           | WRAM bank                                                         | R/W |
| $FF76      | [PCM12 (CGB Mode only)][linkPCM12 (CGB Mode only)]         | Digital outputs 1 & 2                                             | R   |
| $FF77      | [PCM34 (CGB Mode only)][linkPCM34 (CGB Mode only)]         | Digital outputs 3 & 4                                             | R   |
| $FFFF      | [IE][linkIE]                                               | Interrupt enable                                                  | R/W |


[linkSB]: ./Serial_Data_Transfer_(Link_Cable).html#ff01--sb-serial-transfer-data
[linkSC]: ./Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control
[linkDIV]: ./Timer_and_Divider_Registers.html#ff04--div-divider-register
[linkTIMA]: ./Timer_and_Divider_Registers.html#ff05--tima-timer-counter
[linkTMA]: ./Timer_and_Divider_Registers.html#ff06--tma-timer-modulo
[linkTAC]: ./Timer_and_Divider_Registers.html#ff07--tac-timer-control
[linkIF]: ./Interrupts.html#ff0f--if-interrupt-flag
[linkNR10]: ./Audio_Registers.html#ff10--nr10-channel-1-sweep
[linkNR11]: ./Audio_Registers.html#ff11--nr11-channel-1-length-timer--duty-cycle
[linkNR12]: ./Audio_Registers.html#ff12--nr12-channel-1-volume--envelope
[linkNR13]: ./Audio_Registers.html#ff13--nr13-channel-1-wavelength-low-write-only
[linkNR14]: ./Audio_Registers.html#ff14--nr14-channel-1-wavelength-high--control
[linkNR21]: ./Audio_Registers.html#sound-channel-2--pulse
[linkNR22]: ./Audio_Registers.html#sound-channel-2--pulse
[linkNR23]: ./Audio_Registers.html#sound-channel-2--pulse
[linkNR24]: ./Audio_Registers.html#sound-channel-2--pulse
[linkNR30]: ./Audio_Registers.html#ff1a--nr30-channel-3-dac-enable
[linkNR31]: ./Audio_Registers.html#ff1b--nr31-channel-3-length-timer-write-only
[linkNR32]: ./Audio_Registers.html#ff1c--nr32-channel-3-output-level
[linkNR33]: ./Audio_Registers.html#ff1d--nr33-channel-3-wavelength-low-write-only
[linkNR34]: ./Audio_Registers.html#ff1e--nr34-channel-3-wavelength-high--control
[linkNR41]: ./Audio_Registers.html#ff20--nr41-channel-4-length-timer-write-only
[linkNR42]: ./Audio_Registers.html#ff21--nr42-channel-4-volume--envelope
[linkNR43]: ./Audio_Registers.html#ff22--nr43-channel-4-frequency--randomness
[linkNR44]: ./Audio_Registers.html#ff23--nr44-channel-4-control
[linkNR50]: ./Audio_Registers.html#ff24--nr50-master-volume--vin-panning
[linkNR51]: ./Audio_Registers.html#ff25--nr51-sound-panning
[linkNR52]: ./Audio_Registers.html#ff26--nr52-sound-onoff
[linkWave pattern RAM]: ./Audio_Registers.html#ff30ff3f--wave-pattern-ram
[linkLCDC]: ./LCDC.html#ff40--lcdc-lcd-control
[linkSTAT]: ./STAT.html#ff41--stat-lcd-status
[linkSCY]: ./Scrolling.html#ff42ff43--scy-scx-viewport-y-position-x-position
[linkSCX]: ./Scrolling.html#ff42ff43--scy-scx-viewport-y-position-x-position
[linkLY]: ./Scrolling.html#ff44--ly-lcd-y-coordinate-read-only
[linkLYC]: ./Scrolling.html#ff45--lyc-ly-compare
[linkDMA]: ./OAM_DMA_Transfer.html#ff46--dma-oam-dma-source-address--start
[linkBGP (Non-CGB Mode only)]: ./Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
[linkOBP0 (Non-CGB Mode only)]: ./Palettes.html#ff48ff49--obp0-obp1-non-cgb-mode-only-obj-palette-0-1-data
[linkOBP1 (Non-CGB Mode only)]: ./Palettes.html#ff48ff49--obp0-obp1-non-cgb-mode-only-obj-palette-0-1-data
[linkWY]: ./Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
[linkWX]: ./Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7
[linkKEY1 (CGB Mode only)]: ./CGB_Registers.html#ff4d--key1-cgb-mode-only-prepare-speed-switch
[linkVBK (CGB Mode only)]: ./CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank
[linkHDMA1 (CGB Mode only)]: ./CGB_Registers.html#ff51ff52--hdma1-hdma2-cgb-mode-only-vram-dma-source-high-low-write-only
[linkHDMA2 (CGB Mode only)]: ./CGB_Registers.html#ff51ff52--hdma1-hdma2-cgb-mode-only-vram-dma-source-high-low-write-only
[linkHDMA3 (CGB Mode only)]: ./CGB_Registers.html#ff53ff54--hdma3-hdma4-cgb-mode-only-vram-dma-destination-high-low-write-only
[linkHDMA4 (CGB Mode only)]: ./CGB_Registers.html#ff53ff54--hdma3-hdma4-cgb-mode-only-vram-dma-destination-high-low-write-only
[linkHDMA5 (CGB Mode only)]: ./CGB_Registers.html#ff55--hdma5-cgb-mode-only-vram-dma-lengthmodestart
[linkRP (CGB Mode only)]: ./CGB_Registers.html#ff56--rp-cgb-mode-only-infrared-communications-port
[linkBCPS/BGPI (CGB Mode only)]: ./Palettes.html#ff68--bcpsbgpi-cgb-mode-only-background-color-palette-specification--background-palette-index
[linkBCPD/BGPD (CGB Mode only)]: ./Palettes.html#ff69--bcpdbgpd-cgb-mode-only-background-color-palette-data--background-palette-data
[linkOCPS/OBPI (CGB Mode only)]: ./Palettes.html#ff6aff6b--ocpsobpi-ocpdobpd-cgb-mode-only-obj-color-palette-specification--obj-palette-index-obj-color-palette-data--obj-palette-data
[linkOCPD/OBPD (CGB Mode only)]: ./Palettes.html#ff6aff6b--ocpsobpi-ocpdobpd-cgb-mode-only-obj-color-palette-specification--obj-palette-index-obj-color-palette-data--obj-palette-data
[linkOPRI (CGB Mode only)]: ./CGB_Registers.html#ff6c--opri-cgb-mode-only-object-priority-mode
[linkSVBK (CGB Mode only)]: ./CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank
[linkPCM12 (CGB Mode only)]: ./Audio_details.html#ff76--pcm12-cgb-mode-only-digital-outputs-1--2-read-only
[linkPCM34 (CGB Mode only)]: ./Audio_details.html#ff77--pcm34-cgb-mode-only-digital-outputs-3--4-read-only
[linkIE]: ./Interrupts.html#ffff--ie-interrupt-enable
