# Audio Details

tl;dr:
> The PPU is a bunch of state machines, and the APU is a bunch of counters.

<!-- Styles in <svg> tags apply to the whole page, and we have two on this one. Hoist this to be less confusing. -->
<style>
  text {
    fill: var(--fg);
    dominant-baseline: middle;
    font-size: .6em;
  }
  #dsg { font-size: 1em; }
  .centered { text-anchor: middle; }
  .right    { text-anchor: end; }
  .tabnum { font-feature-settings: "tnum"; }
  svg > rect, svg > use {
    stroke: var(--fg);
    fill: var(--fg);
  }
  svg > path, svg > circle {
    stroke: var(--fg);
    fill: none;
  }
  .dashed { stroke-dasharray: 2; }
  .opa50 { opacity: .5; }
  .fill50 { fill-opacity: .5; }
  .digital   { stroke: #df0000; fill: #df0000; }
  .analog    { stroke: #005f98; fill: #005f98; }
  .misc_data { stroke: #a4269c; fill: #a4269c; }
  .dac_ena   { stroke: #ffa700; fill: #ffa700; }
  .ch_active { stroke: #17cc00; fill: #17cc00; }
  rect.legend { width: 5px; height: 5px; }
  text.legend { font-size: .7em; }
  .arrow { stroke-width: 2px; opacity: .35; }
  .unfilled {
    fill: none;
  }
  .no-stroke {
    stroke: none;
  }
</style>

<figure>
{{#include imgs/apu_detailed.svg:2:}}

<figcaption>Source: Lior "LIJI32" Halphon</figcaption>
</figure>

Each of the four "conceptual" channels is composed of a "generation" circuit (designated "channel" in the above diagram), and a [DAC](https://en.wikipedia.org/wiki/Digital-to-analog_converter).
The digital value produced by the generator, which ranges between $0 and $F (0 and 15), is linearly translated by the DAC into an analog[^digital_analog] value between -1 and 1 (the unit is arbitrary).

The four analog channel outputs are then fed into the mixer[^vin], which selectively adds them (depending on [`NR51`]) into two analog outputs (Left and Right).
Thus, the analog range of those outputs is 4× that of each channel, -4 to 4.

Then, both of these two get their respective volume scaled, once from [`NR50`](<#FF24 — NR50: Master volume & VIN panning>), and once from the volume knob (if the console has one).
Note that the former step never mutes a non-silent input, but the latter can.

Each of the two analog outputs then goes through a [high-pass filter](https://en.wikipedia.org/wiki/High-pass_filter) (HPF).
For short, a HPF constantly tries to "pull" the signal towards analog 0 (neutral); the reason for that is explained further below.

[^digital_analog]:
To be clear: digital values are discrete and clear-cut; conversely, the analog domain is continuous.
The former is what computers use, the latter is what the real world is made of.

[^vin]:
Actually, VIN acts as a 5<sup>th</sup> channel fed into the mixer, whose control bits are in NR50 instead of NR51.
This was omitted from the diagram for simplicity.

## PCM registers

These two registers, only present in the Game Boy Color and later models, allow reading the output of the generation circuits directly; this is very useful for testing "internal" APU behavior.
These registers are not documented in any known Nintendo manual.

### FF76 — PCM12 (CGB Mode only): Digital outputs 1 & 2 \[read-only\]

The low nibble is a copy of sound channel 1's digital output, the high nibble a copy of sound channel 2's.

### FF77 — PCM34 (CGB Mode only): Digital outputs 3 & 4 \[read-only\]

Same, but with channels 3 and 4.

## Finer technical explanation

### DIV-APU

A "DIV-APU" counter is increased every time `DIV`'s bit 4 (5 in [double-speed mode](<#FF4D — KEY1 (CGB Mode only): Prepare speed switch>)) goes from 1 to 0, therefore at a frequency of 512 Hz (regardless of whether double-speed is active).
Thus, the counter can be made to increase faster by writing to `DIV` while its relevant bit is set (which clears `DIV`, and triggers the falling edge).

The following events occur every <var>N</var> DIV-APU ticks:

Event          | Rate | Frequency[^div_apu_freq]
---------------|------|-------------------------
Envelope sweep | 8    | 64 Hz
Sound length   | 2    | 256 Hz
CH1 freq sweep | 4    | 128 Hz

[^div_apu_freq]:
Indicated values are under normal operation; the frequencies will obviously differ if writing to `DIV` to increase the counter faster.

### Mixer

A high-pass filter (HPF) removes constant biases over time.
The HPFs therefore remove the DC offset created by inactive channels with an enabled DAC, and off-center waveforms.

:::tip Avoiding audio pops

Enabling or disabling a DAC ([see below](#DACs)), adding or removing it using NR51, or changing the volume in NR50, will cause an audio pop.
(All of these actions cause a change in DC offset, which is smoothed out by the HPFs over time, but still creates a pop.)

To avoid this, a sound driver should avoid turning the DACs off; this can be done by writing $08 to `NRx2` (silences the channel but keeps the DAC on) then $80 to `NRx4` to retrigger the channel and reload `NRx2`.

:::

The HPF is more aggressive on GBA than on GBC, which itself is more aggressive than on DMG.
(The more "aggressive" a HPF, the faster it pulls the signal towards "analog 0"; this tends to also distort waveforms.)

### DACs

Channel <var>x</var>'s DAC is enabled if and only if `[NRx2] & $F8 != 0`; the exception is CH3, whose DAC is directly controlled by bit 7 of NR30 instead.
Note that the envelope functionality changes the volume, but not the value stored in NRx2, and thus doesn't disable the DACs.

If a DAC is enabled, the digital range $0 to $F is linearly translated to the analog range -1 to 1, in arbitrary units.
Importantly, the slope is negative: "digital 0" maps to "analog 1", not "analog -1".

If a DAC is disabled, it fades to an analog value of 0, which corresponds to "digital 7.5".
The nature of this fade is not entirely deterministic and varies between models.

NR52's low 4 bits report whether the channels are turned on, not their DACs.

### Channels

A channel is activated by a write to NRx4's MSB, unless its DAC is off, which forces it to be disabled as well.
The opposite is not true, however: a disabled channel outputs 0, which an enabled DAC will dutifully convert into "analog 1".

A channel can be deactivated in one of the following ways:
- Turning off its DAC
- Its [length timer](<#Length timer>) expiring
- (CH1 only) [Frequency sweep](<#FF10 — NR10: Channel 1 sweep>) overflowing the frequency

### Pulse channels (CH1, CH2)

Each pulse channel has an internal "duty step" counter, which is used to index into [the selected waveform](<#FF11 — NR11: Channel 1 length timer & duty cycle>) (each background stripe corresponds to one "duty step")[^pulse_lut].
The "duty step" increments at the channel's sample rate, which is 8 times [the channel's frequency](<#FF13 — NR13: Channel 1 period low \[write-only\]>)).

The "duty step" counter cannot be reset, except by turning the APU off, which sets both back to 0.
Retriggering a pulse channel causes its "duty step timer" to reset, thus retriggering a pulse channel often enough will cause its "duty step" to never advance.

When first starting up a pulse channel, it will *always* output a (digital) zero.

[^pulse_lut]:
Actually, there is not LUT, but the manipulations done to the counter's bits are equivalent.

### Wave channel (CH3)

CH3 has an internal "sample index" counter.
The "sample index" increments at the channel's sample rate, which is 32 times [the channel's frequency](<#FF1D — NR33: Channel 3 period low \[write-only\]>)
Each time it increments, the corresponding "sample" (nibble) is read from wave RAM.
(This means that sample #0 is skipped when first starting up CH3.)

CH3 does not emit samples directly, but stores every sample read into a buffer, and emits that continuously; (re)triggering the channel does *not* clear nor refresh this buffer, so the last sample ever read will be emitted again.
This buffer *is* cleared when turning the APU on, so CH3 will emit a "digital 0" when first powered on.

CH3 output level control does not, in fact, alter the output level.
It shifts the **digital** value CH3 is outputting, not the analog value.
This only matters when changing the setting mid-playback: the digital values being shifted bias them towards 0, which biases the analog output towards "1"; the HPF will smooth this over time, but not instantly.

### Noise channel (CH4)

{{#include imgs/ch4_lfsr.svg:2:}}

CH4 revolves around a [LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register), pictured above.
The LFSR has 16 bits: 15 bits for its current state and 1 bit to temporarily store the next bit to shift in.

When CH4 is ticked (at the frequency specified via [`NR43`](<#FF22 — NR43: Channel 4 frequency & randomness>)):
1. The result of <math><menclose notation="top"><msub><mi>LFSR</mi><mn>0</mn></msub> <mo>⊕</mo> <msub><mi>LFSR</mi><mn>1</mn></msub></menclose></math> (`1` if bit 0 and bit 1 are identical, `0` otherwise) is written to bit 15.
2. If "short mode" was selected in [`NR43`](<#FF22 — NR43: Channel 4 frequency & randomness>), then bit 15 is copied to bit 7 as well.
3. Finally, the entire LFSR is shifted right, and bit 0 selects between 0 and [the chosen volume](<#FF21 — NR42: Channel 4 volume & envelope>).

The LFSR is set to 0 when (re)triggering the channel.

:::tip Lock-up

If the "active" portion of the LFSR only contains "1" bits, only "1" bits will be generated; this prevents CH4 from ever changing values (until retriggered), essentially silencing it.

This does not happen under regular operation, but can be achieved by switching from 15-bit to 7-bit mode when the LFSR's bottom 7 bits are all "1"s (which occurs relatively early after triggering the channel, for example).

:::

## Game Boy Advance audio

The APU was reworked pretty heavily for the GBA, which introduces some slightly different behavior:
- Instead of mixing being done by analog circuitry, it's instead done digitally; then, sound is converted to an analog signal and an offset is added (see `SOUNDBIAS` in [GBATEK](http://problemkaputt.de/gbatek.htm#gbasoundcontrolregisters) for more details).
- This also means that the GBA APU has no DACs.
  Instead, they are emulated digitally such that a disabled "DAC" behaves like an enabled DAC receiving 0 as its input.
- Additionally, CH3's DAC has its output inverted.
  In particular, this causes the channel to emit a loud spike when disabled; therefore, it's a good idea to "disconnect" the channel using NR51 before accessing wave RAM.

None of the additional features (more wave RAM, digital FIFOs, etc.) are available to CGB programs.

[`NR51`]: <#FF25 — NR51: Sound panning>
