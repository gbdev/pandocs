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
{{#include imgs/src/apu_detailed.svg:2:}}

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

### Pulse channel with sweep (CH1)

The first square channel has a frequency sweep unit, controlled by `NR10`. This internally has a "sweep timer", "enabled flag",
and a period/frequency "shadow register". The "enabled flag" controls if the sweep unit is active,
the "sweep timer" is clocked at 128 Hz by the [DIV-APU](#DIV-APU), and the "shadow register" holds the current output period.

During a [trigger event](#Triggering), several things occur:

- CH1 [period value](<#FF13 — NR13: Channel 1 period low \[write-only\]>) is copied to the "shadow register".
- The "sweep timer" is reset.
- The "enabled flag" is set if either the [sweep pace or individual step](<#FF10 — NR10: Channel 1 sweep>) are non-zero, cleared otherwise.
- If the individual step is non-zero, _frequency calculation_ and _overflow check_ are performed immediately.

_Frequency calculation_ consists of taking the value in the frequency "shadow register", shifting it right by the individual step,
optionally negating the value, depending on the [direction](<#FF10 — NR10: Channel 1 sweep>), and summing this with the frequency "shadow register" to produce a new frequency.
What is done with this new frequency depends on the context.

The _overflow check_ simply calculates the new frequency and if it is greater than 2047, or $7FF, CH1 is disabled.

When the "sweep timer" is [clocked](#DIV-APU) if the "enabled flag" is set and the sweep pace is not zero,
a new frequency is calculated and the overflow check is performed.
If the new frequency is 2047 or less and the individual step is not zero, this new frequency is written back to the
"shadow register" and CH1 frequency in `NR13` and `NR14`, then frequency calculation and overflow check
are run _again_ immediately using this new value, but this second new frequency is not written back.

CH1 frequency can be modified via `NR13` and `NR14` while sweep is active,
but the "shadow register" won't be affected so the next time the "sweep timer" updates the channel's frequency,
this modification will be lost. This can be avoided by triggering the channel.

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

{{#include imgs/src/ch4_lfsr.svg:2:}}

CH4 revolves around a [LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register), pictured above.
The LFSR has 16 bits: 15 bits for its current state and 1 bit to temporarily store the next bit to shift in.

When CH4 is ticked (at the frequency specified via [`NR43`](<#FF22 — NR43: Channel 4 frequency & randomness>)):
1. The result of <math><menclose notation="top"><msub><mi>LFSR</mi><mn>0</mn></msub> <mo>⊙</mo> <msub><mi>LFSR</mi><mn>1</mn></msub></menclose></math> (`1` if bit 0 and bit 1 are identical, `0` otherwise) is written to bit 15.
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

## Obscure Behavior

- The volume envelope and sweep timers treat a period of 0 as 8.
- Just after powering on, the first duty step of Ch1 and Ch2 after they are triggered for the first time is played
  as if it were 0. Also, the duty cycle clocking is disabled until the first trigger.
- When triggering Ch3, the first sample to play is the previous one still in the high nibble of the sample buffer, and the next sample is the second nibble from the wave table. This is because it doesn't load the first byte on trigger like it *should*.
  The first nibble from the wave table is thus not played until the waveform loops.
- When triggering Ch1 and Ch2, the low two bits of the frequency timer are NOT modified.
- Extra length clocking occurs when writing to NRx4 when the DIV-APU next step is one that doesn't clock the length timer. In this case, if the length timer was PREVIOUSLY disabled and now enabled and the length timer is not zero, it is decremented. If this decrement makes it zero and trigger is clear, the channel is disabled. On the CGB-02, the length timer only has to have been disabled before; the current length enable state doesn't matter. This breaks at least one game (Prehistorik Man), and was fixed on CGB-04 and CGB-05.
- If a channel is triggered when the DIV-APU next step is one that doesn't clock the length timer and the length timer is now enabled and length is being set to 64 (256 for wave channel) because it was previously zero, it is set to 63 instead (255 for wave channel).
- If a channel is triggered when the DIV-APU next step will clock the volume envelope, the envelope's timer is reloaded with one greater than it would have been.
- Using a noise channel clock shift of 14 or 15 results in the LFSR receiving no clocks.
- Clearing the sweep direction bit in NR10 after at least one sweep calculation has been made using the substraction mode since the last trigger causes the channel to be immediately disabled. This prevents you from having the sweep lower the frequency then raise the frequency without a trigger inbetween.
- Triggering the wave channel on the DMG while it reads a sample byte will alter the first four bytes of wave RAM. If the channel was reading one of the first four bytes, the only first byte will be rewritten with the byte being read. If the channel was reading one of the later 12 bytes, the first FOUR bytes of wave RAM will be rewritten with the four aligned bytes that the read was from (bytes 4-7, 8-11, or 12-15); for example if it were reading byte 9 when it was retriggered, the first four bytes would be rewritten with the contents of bytes 8-11. To avoid this corruption you should stop the wave by writing 0 then $80 to NR30 before triggering it again. The game Duck Tales encounters this issue part way through most songs.
- "Zombie" mode: the volume can be manually altered while a channel is playing by writing to NRx2. Behavior depends on the old and new values of NRx2, and whether the envlope has stopped automatic updates. The CGB-02 and CGB-04 are the most consistent:
  * If the old envelope period was zero and the envelope is still doing automatic updates, volume is incremented by 1, otherwise if the envelope was in decrease mode, volume is incremented by 2.
  * If the mode was changed (increase to decrease or decrease to increase), volume is set to 16-volume.
  * Only the low 4 bits of volume are kept after the above operations.

Other models behave differently, especially the DMG units which have crazy behavior in some cases. The only useful consistent behavior is using increase mode with a period of zero in order to increment the volume by 1. That is, write $V8 to NRx2 to set the initial volume to V before triggering the channel, then write $08 to NRx2 to increment the volume as the sound plays (repeat 15 times to decrement the volume by 1). This allows manual volume control on all units tested.

- When all four channel DACs are off, the master volume units are disconnected from the sound output and the output level becomes 0. When any channel DAC is on, a high-pass filter capacitor is connected which slowly removes any DC component from the signal. The following code applied at 4194304 Hz implements these two behaviors for one of the DMG output channels (unoptimized floating point for clarity):

```c
static double capacitor = 0.0;

double high_pass( double in, bool dacs_enabled )
{
     double out = 0.0;
     if ( dacs_enabled )
     {
         out = in - capacitor;

         // capacitor slowly charges to 'in' via their difference
         capacitor = in - out * 0.999958; // use 0.998943 for MGB&CGB
     }
     return out;
}
```

The charge factor can be calculated for any output sampling rate as 0.999958^(4194304/rate). So if you were applying high_pass() at 44100 Hz, you'd use a charge factor of 0.996.
