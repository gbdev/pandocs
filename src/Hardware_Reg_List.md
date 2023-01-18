
# Hardware Registers

<table>
  <thead>
    <tr>
      <th>Addr</th>
      <th>Name</th>
      <th>Description</th>
      <th>R/W</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>$FF00</td>
      <td><a href="./Joypad_Input.html#ff00--p1joyp-joypad">P1/JOYP</a></td>
      <td>Joypad</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF01</td>
      <td><a href="./Serial_Data_Transfer_(Link_Cable).html#ff01--sb-serial-transfer-data">SB</a></td>
      <td>Serial transfer data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF02</td>
      <td><a href="./Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control">SC</a></td>
      <td>Serial transfer control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF04</td>
      <td><a href="./Timer_and_Divider_Registers.html#ff04--div-divider-register">DIV</a></td>
      <td>Divider register</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF05</td>
      <td><a href="./Timer_and_Divider_Registers.html#ff05--tima-timer-counter">TIMA</a></td>
      <td>Timer counter</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF06</td>
      <td><a href="./Timer_and_Divider_Registers.html#ff06--tma-timer-modulo">TMA</a></td>
      <td>Timer modulo</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF07</td>
      <td><a href="./Timer_and_Divider_Registers.html#ff07--tac-timer-control">TAC</a></td>
      <td>Timer control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF0F</td>
      <td><a href="./Interrupts.html#ff0f--if-interrupt-flag">IF</a></td>
      <td>Interrupt flag</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF10</td>
      <td><a href="./Audio_Registers.html#ff10--nr10-channel-1-sweep">NR10</a></td>
      <td>Channel 1 sweep</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF11</td>
      <td><a href="./Audio_Registers.html#ff11--nr11-channel-1-length-timer--duty-cycle">NR11</a></td>
      <td>Channel 1 length timer & duty cycle</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF12</td>
      <td><a href="./Audio_Registers.html#ff12--nr12-channel-1-volume--envelope">NR12</a></td>
      <td>Channel 1 volume & envelope</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF13</td>
      <td><a href="./Audio_Registers.html#ff13--nr13-channel-1-wavelength-low-write-only">NR13</a></td>
      <td>Channel 1 wavelength low</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF14</td>
      <td><a href="./Audio_Registers.html#ff14--nr14-channel-1-wavelength-high--control">NR14</a></td>
      <td>Channel 1 wavelength high & control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF16</td>
      <td><a href="./Audio_Registers.html#sound-channel-2--pulse">NR21</a></td>
      <td>Channel 2 length timer & duty cycle</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF17</td>
      <td><a href="./Audio_Registers.html#sound-channel-2--pulse">NR22</a></td>
      <td>Channel 2 volume & envelope</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF18</td>
      <td><a href="./Audio_Registers.html#sound-channel-2--pulse">NR23</a></td>
      <td>Channel 2 wavelength low</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF19</td>
      <td><a href="./Audio_Registers.html#sound-channel-2--pulse">NR24</a></td>
      <td>Channel 2 wavelength high & control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF1A</td>
      <td><a href="./Audio_Registers.html#ff1a--nr30-channel-3-dac-enable">NR30</a></td>
      <td>Channel 3 DAC enable</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF1B</td>
      <td><a href="./Audio_Registers.html#ff1b--nr31-channel-3-length-timer-write-only">NR31</a></td>
      <td>Channel 3 length timer</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF1C</td>
      <td><a href="./Audio_Registers.html#ff1c--nr32-channel-3-output-level">NR32</a></td>
      <td>Channel 3 output level</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF1D</td>
      <td><a href="./Audio_Registers.html#ff1d--nr33-channel-3-wavelength-low-write-only">NR33</a></td>
      <td>Channel 3 wavelength low</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF1E</td>
      <td><a href="./Audio_Registers.html#ff1e--nr34-channel-3-wavelength-high--control">NR34</a></td>
      <td>Channel 3 wavelength high & control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF20</td>
      <td><a href="./Audio_Registers.html#ff20--nr41-channel-4-length-timer-write-only">NR41</a></td>
      <td>Channel 4 length timer</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF21</td>
      <td><a href="./Audio_Registers.html#ff21--nr42-channel-4-volume--envelope">NR42</a></td>
      <td>Channel 4 volume & envelope</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF22</td>
      <td><a href="./Audio_Registers.html#ff22--nr43-channel-4-frequency--randomness">NR43</a></td>
      <td>Channel 4 frequency & randomness</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF23</td>
      <td><a href="./Audio_Registers.html#ff23--nr44-channel-4-control">NR44</a></td>
      <td>Channel 4 control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF24</td>
      <td><a href="./Audio_Registers.html#ff24--nr50-master-volume--vin-panning">NR50</a></td>
      <td>Master volume & VIN panning</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF25</td>
      <td><a href="./Audio_Registers.html#ff25--nr51-sound-panning">NR51</a></td>
      <td>Sound panning</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF26</td>
      <td><a href="./Audio_Registers.html#ff26--nr52-sound-onoff">NR52</a></td>
      <td>Sound on/off</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF30-FF3F</td>
      <td><a href="./Audio_Registers.html#ff30ff3f--wave-pattern-ram"></a></td>
      <td>Wave pattern RAM</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF40</td>
      <td><a href="./LCDC.html#ff40--lcdc-lcd-control">LCDC</a></td>
      <td>LCD control</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF41</td>
      <td><a href="./STAT.html#ff41--stat-lcd-status">STAT</a></td>
      <td>LCD status</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF42</td>
      <td><a href="./Scrolling.html#ff42ff43--scy-scx-viewport-y-position-x-position">SCY</a></td>
      <td>Viewport Y position</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF43</td>
      <td><a href="./Scrolling.html#ff42ff43--scy-scx-viewport-y-position-x-position">SCX</a></td>
      <td>Viewport X position</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF44</td>
      <td><a href="./Scrolling.html#ff44--ly-lcd-y-coordinate-read-only">LY</a></td>
      <td>LCD Y coordinate</td>
      <td>R</td>
    </tr>
    <tr>
      <td>$FF45</td>
      <td><a href="./Scrolling.html#ff45--lyc-ly-compare">LYC</a></td>
      <td>LY compare</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF46</td>
      <td><a href="./OAM_DMA_Transfer.html#ff46--dma-oam-dma-source-address--start">DMA</a></td>
      <td>OAM DMA source address & start</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF47</td>
      <td><a href="./Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data">BGP (Non-CGB Mode only)</a></td>
      <td>BG palette data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF48</td>
      <td><a href="./Palettes.html#ff48ff49--obp0-obp1-non-cgb-mode-only-obj-palette-0-1-data">OBP0 (Non-CGB Mode only)</a></td>
      <td>OBJ palette 0 data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF49</td>
      <td><a href="./Palettes.html#ff48ff49--obp0-obp1-non-cgb-mode-only-obj-palette-0-1-data">OBP1 (Non-CGB Mode only)</a></td>
      <td>OBJ palette 1 data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF4A</td>
      <td><a href="./Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7">WY</a></td>
      <td>Window Y position</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF4B</td>
      <td><a href="./Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7">WX</a></td>
      <td>Window X position plus 7</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF4D</td>
      <td><a href="./CGB_Registers.html#ff4d--key1-cgb-mode-only-prepare-speed-switch">KEY1 (CGB Mode only)</a></td>
      <td>Prepare speed switch</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF4F</td>
      <td><a href="./CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank">VBK (CGB Mode only)</a></td>
      <td>VRAM bank</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF51</td>
      <td><a href="./CGB_Registers.html#ff51ff52--hdma1-hdma2-cgb-mode-only-vram-dma-source-high-low-write-only">HDMA1 (CGB Mode only)</a></td>
      <td>VRAM DMA source high</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF52</td>
      <td><a href="./CGB_Registers.html#ff51ff52--hdma1-hdma2-cgb-mode-only-vram-dma-source-high-low-write-only">HDMA2 (CGB Mode only)</a></td>
      <td>VRAM DMA source low</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF53</td>
      <td><a href="./CGB_Registers.html#ff53ff54--hdma3-hdma4-cgb-mode-only-vram-dma-destination-high-low-write-only">HDMA3 (CGB Mode only)</a></td>
      <td>VRAM DMA destination high</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF54</td>
      <td><a href="./CGB_Registers.html#ff53ff54--hdma3-hdma4-cgb-mode-only-vram-dma-destination-high-low-write-only">HDMA4 (CGB Mode only)</a></td>
      <td>VRAM DMA destination low</td>
      <td>W</td>
    </tr>
    <tr>
      <td>$FF55</td>
      <td><a href="./CGB_Registers.html#ff55--hdma5-cgb-mode-only-vram-dma-lengthmodestart">HDMA5 (CGB Mode only)</a></td>
      <td>VRAM DMA length/mode/start</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF56</td>
      <td><a href="./CGB_Registers.html#ff56--rp-cgb-mode-only-infrared-communications-port">RP (CGB Mode only)</a></td>
      <td>Infrared communications port</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF68</td>
      <td><a href="./Palettes.html#ff68--bcpsbgpi-cgb-mode-only-background-color-palette-specification--background-palette-index">BCPS/BGPI (CGB Mode only)</a></td>
      <td>Background color palette specification / Background palette index</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF69</td>
      <td><a href="./Palettes.html#ff69--bcpdbgpd-cgb-mode-only-background-color-palette-data--background-palette-data">BCPD/BGPD (CGB Mode only)</a></td>
      <td>Background color palette data / Background palette data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF6A</td>
      <td><a href="./Palettes.html#ff6aff6b--ocpsobpi-ocpdobpd-cgb-mode-only-obj-color-palette-specification--obj-palette-index-obj-color-palette-data--obj-palette-data">OCPS/OBPI (CGB Mode only)</a></td>
      <td>OBJ color palette specification / OBJ palette index</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF6B</td>
      <td><a href="./Palettes.html#ff6aff6b--ocpsobpi-ocpdobpd-cgb-mode-only-obj-color-palette-specification--obj-palette-index-obj-color-palette-data--obj-palette-data">OCPD/OBPD (CGB Mode only)</a></td>
      <td>OBJ color palette data / OBJ palette data</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF6C</td>
      <td><a href="./CGB_Registers.html#ff6c--opri-cgb-mode-only-object-priority-mode">OPRI (CGB Mode only)</a></td>
      <td>Object priority mode</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF70</td>
      <td><a href="./CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank">SVBK (CGB Mode only)</a></td>
      <td>WRAM bank</td>
      <td>R/W</td>
    </tr>
    <tr>
      <td>$FF76</td>
      <td><a href="./Audio_details.html#ff76--pcm12-cgb-mode-only-digital-outputs-1--2-read-only">PCM12 (CGB Mode only)</a></td>
      <td>Digital outputs 1 & 2</td>
      <td>R</td>
    </tr>
    <tr>
      <td>$FF77</td>
      <td><a href="./Audio_details.html#ff77--pcm34-cgb-mode-only-digital-outputs-3--4-read-only">PCM34 (CGB Mode only)</a></td>
      <td>Digital outputs 3 & 4</td>
      <td>R</td>
    </tr>
    <tr>
      <td>$FFFF</td>
      <td><a href="./Interrupts.html#ffff--ie-interrupt-enable">IE</a></td>
      <td>Interrupt enable</td>
      <td>R/W</td>
    </tr>
  </tbody>
</table>
