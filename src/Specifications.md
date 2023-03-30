# Specifications

<style>
td {
    text-align: center;
}
td:first-child {
    text-align: left;
}
</style>

<table>
  <thead>
    <tr>
      <th></th><th>Game Boy (DMG)</th><th>Game Boy Pocket (MGB)</th><th>Super Game Boy (SGB)</th><th>Game Boy Color (CGB)</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>CPU</td><td colspan="4">8-bit 8080-like Sharp CPU (speculated to be a SM83 core)</td>
    </tr>
    <tr>
      <td>CPU freq</td><td colspan="2">4.194304&nbsp;MHz<sup class="footnote-reference"><a href="#dmg_clk">1</a></sup></td><td>Depends on revision<sup class="footnote-reference"><a href="#sgb_clk">2</a></sup></td><td>Up to 8.388608&nbsp;MHz</td>
    </tr>
    <tr>
        <td>Work RAM</td><td colspan="3">8&nbsp;KiB</td><td>32&nbsp;KiB<sup class="footnote-reference"><a href="#compat">3</a></sup> (4&nbsp;+&nbsp;7&nbsp;×&nbsp;4&nbsp;KiB)</td>
    </tr>
    <tr>
        <td>Video RAM</td><td colspan="3">8&nbsp;KiB</td><td>16&nbsp;KiB<sup class="footnote-reference"><a href="#compat">3</a></sup> (2&nbsp;×&nbsp;8&nbsp;KiB)</td>
    </tr>
    <tr>
        <td>Screen</td><td>LCD 4.7&nbsp;×&nbsp;4.3&nbsp;cm</td><td>LCD 4.8&nbsp;×&nbsp;4.4&nbsp;cm</td><td>CRT TV</td><td>TFT 4.4&nbsp;×&nbsp;4&nbsp;cm</td>
    </tr>
    <tr>
        <td>Resolution</td><td colspan="2">160&nbsp;×&nbsp;144</td><td>160&nbsp;×&nbsp;144 within 256&nbsp;×&nbsp;224 border</td><td>160&nbsp;×&nbsp;144</td>
    </tr>
    <tr>
        <td>OBJ ("sprites")</td><td colspan="4">8&nbsp;×&nbsp;8 or 8&nbsp;×&nbsp;16 ; max 40 per screen, 10 per line</td>
    </tr>
    <tr>
        <td>Palettes</td><td colspan="2">BG: 1&nbsp;×&nbsp;4, OBJ: 2&nbsp;×&nbsp;3</td><td>BG/OBJ: 1&nbsp;+&nbsp;4&nbsp;×&nbsp;3, border: 4&nbsp;×&nbsp;15</td><td>BG: 8&nbsp;×&nbsp;4, OBJ: 8&nbsp;×&nbsp;3<sup class="footnote-reference"><a href="#compat">3</a></sup></td>
    </tr>
    <tr>
        <td>Colors</td><td>4 shades of green</td><td>4 shades of gray</td><td colspan="2">32768 colors (15-bit RGB)</td>
    </tr>
    <tr>
        <td>Horizontal sync</td><td colspan="2">9.198&nbsp;KHz</td><td>Complicated<sup class="footnote-reference"><a href="#sgb_vid">4</a></sup></td><td>9.198&nbsp;KHz</td>
    </tr>
    <tr>
        <td>Vertical sync</td><td colspan="2">59.73&nbsp;Hz</td><td>Complicated<sup class="footnote-reference"><a href="#sgb_vid">4</a></sup></td><td>59.73&nbsp;Hz</td>
    </tr>
    <tr>
        <td>Sound</td><td colspan="2">4 channels with stereo output</td><td>4 GB channels + SNES audio</td><td>4 channels with stereo output</td>
    </tr>
    <tr>
        <td>Power</td><td>DC 6V, 0.7&nbsp;W</td><td>DC 3V, 0.7&nbsp;W</td><td>Powered by SNES</td><td>DC 3V, 0.6&nbsp;W</td>
    </tr>
  </tbody>
</table>

[^dmg_clk]:
Real DMG units tend to run about 50-70 PPM slow. Accuracy of other models is unknown. [See this page](https://github.com/jkotlinski/gbchrono) for more details.

[^sgb_clk]:
SGB1 cartridges derive the GB CPU clock from the SNES' clock, [yielding a clock speed a bit higher](<#SGB System Clock>), which differs slightly between NTSC and PAL systems.
SGB2 instead uses a clock internal to the cartridge, and so has the same speed as the handhelds.

[^compat]:
The same value as on DMG is used in compatibility mode.

[^sgb_vid]:
The SGB runs two consoles: a Game Boy within the SGB cartridge, and the SNES itself.
The GB LCD output is captured and displayed by the SNES, but the two consoles' frame rates don't quite sync up, leading to duplicated and/or dropped frames.
The GB side of the vertical sync depends on the CPU clock[^sgb_clk], with the same ratio as the handhelds.
