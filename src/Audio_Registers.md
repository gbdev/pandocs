# Audio Registers

Audio registers are named following a `NRxy` scheme, where `x` is the channel number (or `5` for "global" registers), and `y` is the register's ID within the channel.
Since many registers share common properties, a notation is often used where e.g. `NRx2` is used to designate `NR12`, `NR22`, `NR32`, and `NR42` at the same time, for simplicity.

As a rule of thumb, for any `x` in `1`, `2`, `3`, `4`:
- `NRx0` is some channel-specific feature (if present),
- `NRx1` controls the length timer,
- `NRx2` controls the volume and envelope,
- `NRx3` controls the wavelength (maybe only partially),
- `NRx4` has the channel's trigger and length timer enable bits;

...but there are some exceptions.

One of the pitfalls of the `NRxy` naming convention is that the register's purpose is not immediately clear from its name, so some alternative names have been proposed, [such as `AUDENA` for `NR52`](https://github.com/gbdev/hardware.inc/blob/05f5a9b6c7172abe1d7488080c1c050284c09226/hardware.inc#L415).

## Global control registers

### FF26 — NR52: Sound on/off

This register controls whether the APU is powered on at all (akin to [`LCDC` bit 7](<#LCDC.7 — LCD enable>)), and allows checking whether channels are active[^nr52_dac].
Turning the APU off drains less power (around 16%), but clears all APU registers and makes them read-only until turned back on, except `NR52`[^dmg_apu_off].

```
 Bit 7 - All sound on/off  (0: turn the APU off) (Read/Write)
 Bit 3 - Channel 4 ON flag (Read Only)
 Bit 2 - Channel 3 ON flag (Read Only)
 Bit 1 - Channel 2 ON flag (Read Only)
 Bit 0 - Channel 1 ON flag (Read Only)
```

Bits 0-3 of this register are read-only status bits.
Writing to those does NOT enable or disable the channels, despite many emulators behaving as if.

A channel is turned on by triggering it (i.e. setting bit 7 of `NRx4`)[^dac_off].
A channel is turned off when any of the following occurs:
- The channel's length timer, if enabled in `NRx4`, expires
- For CH1: when the wavelength sweep overflows[^freq_sweep_underflow]
- [The channel's DAC](#DACs) is turned off

The envelope reaching a volume of 0 does NOT turn the channel off!

[^nr52_dac]:
Actually, the low nibble of NR52 only reports whether the channels' *generation* circuits are enabled, not if [the DACs](#DACs) are.

[^dmg_apu_off]:
...and the length timers (in `NRx1`) on monochrome models.

[^dac_off]:
If [the DAC](#DACs) is off, then the write to NRx4 will be ineffective and won't turn the channel on.

[^freq_sweep_underflow]:
The wavelength sweep cannot normally underflow, so a "decreasing" sweep (`NR10` bit 3 set) won't turn the channel off.

### FF25 — NR51: Sound panning

Each channel can be panned hard left, center, hard right, or ignored entirely.

```
Bit 7 - Mix channel 4 into left output
Bit 6 - Mix channel 3 into left output
Bit 5 - Mix channel 2 into left output
Bit 4 - Mix channel 1 into left output
Bit 3 - Mix channel 4 into right output
Bit 2 - Mix channel 3 into right output
Bit 1 - Mix channel 2 into right output
Bit 0 - Mix channel 1 into right output
```

### FF24 — NR50: Master volume & VIN panning

The volume bits specify the master volume, i.e. how much each output should be scaled.
A value of 0 is treated as a volume of 1 (very quiet), and a value of 7 is treated as a volume of 8 (no volume reduction).
Importantly, the amplifier **never mutes** a non-silent input.

The VIN mixing bits work exactly like bits in [`NR51`](<#FF25 — NR51: Sound panning>).

```
Bit 7   - Mix VIN into left output  (1=Enable)
Bit 6-4 - Left output volume        (0-7)
Bit 3   - Mix VIN into right output (1=Enable)
Bit 2-0 - Right output volume       (0-7)
```

## Sound Channel 1 — Pulse with wavelength sweep

### FF10 — NR10: Channel 1 sweep

This register controls CH1's wavelength sweep functionality.

```
Bit 6-4 - Sweep pace
Bit 3   - Sweep increase/decrease
           0: Addition    (wavelength increases)
           1: Subtraction (wavelength decreases)
Bit 2-0 - Sweep slope control (n: 0-7)
```

The <var>sweep pace</var> dictates how often the wavelength gets changed, in units of 128 Hz ticks[^div_apu] (7.8 ms).
The pace is only reloaded after the following sweep iteration, or when (re)triggering the channel.
However, if bits 4–6 are all set to 0, then iterations are instantly disabled, and the pace will be reloaded immediately if it's set to something else.

On each sweep iteration, the wavelength in [`NR13`](<#FF13 — NR13: Channel 1 wavelength low \[write-only\]>) and [`NR14`](<#FF14 — NR14: Channel 1 wavelength high & control>) is modified and written back.
That is, unless <var>n</var> (the slope) is 0, in which case iterations do nothing (in this case, subtraction mode should be set, see below).

On each tick, the new wavelength <math><msub><mi>L</mi><mrow><mi>t</mi><mo>+</mo><mn>1</mn></mrow></msub></math> is computed from the current one <math><msub><mi>L</mi><mi>t</mi></msub></math> as follows:

<math display="block">
  <msub><mi>L</mi><mrow><mi>t</mi><mo>+</mo><mn>1</mn></mrow></msub> <mo>=</mo> <msub><mi>L</mi><mi>t</mi></msub> <mo>±</mo> <mfrac><msub><mi>L</mi><mi>t</mi></msub><msup><mn>2</mn><mi>n</mi></msup></mfrac>
</math>

In addition mode, if the wavelength would overflow (i.e. <math><msub><mi>L</mi><mrow><mi>t</mi><mo>+</mo><mn>1</mn></mrow></msub></math> is strictly more than $7FF), the channel is turned off instead.
**This occurs even if sweep iterations are disabled** by <var>n</var> = 0.

Note that if the wavelength ever becomes 0, the wavelength sweep will never be able to change it.
For the same reason, the wavelength sweep cannot underflow the wavelength (which would turn the channel off).

[^div_apu]:
[As long as `DIV` is not written to](#DIV-APU).

### FF11 — NR11: Channel 1 length timer & duty cycle

This register controls both the channel's [length timer](<#Length timer>) and [duty cycle](https://en.wikipedia.org/wiki/Duty_cycle) (the ratio of the time spent low vs. high).
However, the selected duty cycle also alters the phase, although the effect is hardly noticeable.

```
Bit 7-6 - Wave duty            (Read/Write)
Bit 5-0 - Initial length timer (Write Only)
```

<style>
.waveform {
  display: block;
  width: 16rem;
  background-image: repeating-linear-gradient(to right, var(--bg), var(--bg) 1rem, var(--table-alternate-bg) 1rem, var(--table-alternate-bg) 2rem);
  border: 1px solid var(--table-header-bg);
}
.waveform polyline {
  stroke-width: 1px;
  stroke: var(--inline-code-color);
  fill: none;
}
</style>

Wave duty (binary) | Duty cycle | Waveform
------------------:|:----------:|--------------------------
 00                |   12.5 %   | <svg viewBox="0 -1 80 12" preserveAspectRatio="xMidYMid meet" class="waveform"><polyline points="0,0 35,0 35,10 40,10 40,0 75,0 75,10 80,10 80,0"/></svg>
 01                |   25 %     | <svg viewBox="0 -1 80 12" preserveAspectRatio="xMidYMid meet" class="waveform"><polyline points="0,10 5,10 5,0 35,0 35,10 45,10 45,0 75,0 75,10 80,10"/></svg>
 10                |   50 %     | <svg viewBox="0 -1 80 12" preserveAspectRatio="xMidYMid meet" class="waveform"><polyline points="0,10 5,10 5,0 25,0 25,10 45,10 45,0 65,0 65,10 80,10"/></svg>
 11                |   75 %     | <svg viewBox="0 -1 80 12" preserveAspectRatio="xMidYMid meet" class="waveform"><polyline points="0,0 5,0 5,10 35,10 35,0 45,0 45,10 75,10 75,0 80,0"/></svg>

It's worth noting that there is no audible difference between the 25 % and 75 % duty cycle settings.

The higher the [length timer](<#Length timer>) field, the shorter the time before the channel is cut.

### FF12 — NR12: Channel 1 volume & envelope

This register controls the digital amplitude of the "high" part of the pulse, and the sweep applied to that setting.

```
Bit 7-4 - Initial volume of envelope (0-F) (0=No Sound)
Bit 3   - Envelope direction (0=Decrease, 1=Increase)
Bit 2-0 - Sweep pace (0=No Sweep)
```

Setting bits 3-7 of this register all to 0 turns the DAC off (and thus, the channel as well), which [may cause an audio pop](<#Mixer>).

The envelope ticks at 64 Hz, and the channel's envelope will be increased / decreased (depending on bit 3) every <var>Sweep pace</var> of those ticks.

Writes to this register while the channel is on require retriggering it afterwards.

### FF13 — NR13: Channel 1 wavelength low \[write-only\]

This register stores the low 8 bits of the channel's 11-bit "[wavelength](<#Frequency>)".
The upper 3 bits are stored in the low 3 bits of `NR14`.

The actual signal frequency is <math><mfrac><mn>131072</mn><mrow><mn>2048</mn><mo>-</mo><mi>wavelen</mi></mrow></mfrac></math> Hz: the higher the value, the higher the frequency.
This is the whole wave's frequency; the rate at which the channel steps through the 8 "steps" in its wave form is 8× that, i.e. <math><mfrac><mn>1048576</mn><mrow><mn>2048</mn><mo>-</mo><mi>wavelen</mi></mrow></mfrac></math> Hz = <math><mfrac><mn>1</mn><mrow><mn>2048</mn><mo>-</mo><mn>wavelen</mn></mrow></mfrac></math> MiHz.

### FF14 — NR14: Channel 1 wavelength high & control

```
Bit 7   - Trigger (1=Restart channel)  (Write Only)
Bit 6   - Sound Length enable          (Read/Write)
          (1=Stop output when length in NR11 expires)
Bit 2-0 - "Wavelength"'s higher 3 bits (Write Only)
```

Writing a value here with bit 7 set [triggers](<#Triggering>) the channel.

Bit 6 takes effect immediately upon writing to this register.

## Sound Channel 2 — Pulse

This sound channel works exactly like channel 1, except that it lacks a wavelength sweep (and thus an equivalent to [`NR10`](<#FF10 — NR10: Channel 1 sweep>)).
Please refer to the corresponding CH1 register:
- `NR21` (\$FF16) → [`NR11`](<#FF11 — NR11: Channel 1 length timer & duty cycle>)
- `NR22` (\$FF17) → [`NR12`](<#FF12 — NR12: Channel 1 volume & envelope>)
- `NR23` (\$FF18) → [`NR13`](<#FF13 — NR13: Channel 1 wavelength low \[write-only\]>)
- `NR24` (\$FF19) → [`NR14`](<#FF14 — NR14: Channel 1 wavelength high & control>)

## Sound Channel 3 — Wave output

While other channels only offer limited control over the waveform they generate, this channel allows outputting any wave.
It's thus sometimes called a "voluntary wave" channel.

While the "length" of the wave is fixed at 32 "samples", 4-bit each, the speed at which it is read can be customized.
It's possible to "shorten" the wave by either feeding it a repeating pattern, or doubling each sample and doubling the read rate.
It's also possible to artificially "increase" the wave's length by loading a new wave as soon as the whole buffer has been read; this is sometimes used for full-on sample playback.

### FF1A — NR30: Channel 3 DAC enable

This register controls CH3's DAC.
Like other channels, turning the DAC off immediately turns the channel off as well.

```
Bit 7 - Sound Channel 3 DAC  (0=Off, 1=On)
```

The DAC is often turned off just before writing to [wave RAM](<#FF30–FF3F — Wave pattern RAM>) to avoid issues with accessing it; see further below for more info.

Turning the DAC off [may cause an audio pop](<#Mixer>).

### FF1B — NR31: Channel 3 length timer \[write-only\]

This register controls the channel's [length timer](<#Length timer>).

```
Bit 7-0 - length timer
```

The higher the [length timer](<#Length timer>), the shorter the time before the channel is cut.

### FF1C — NR32: Channel 3 output level

This channel lacks the envelope functionality that the other three channels have.

```
Bits 6-5 - Output level selection
```

Bits 6-5 (binary) | Output level
-----------------:|--------------
  %00             | Mute (No sound)
  %01             | 100% volume (use samples read from Wave RAM as-is)
  %10             |  50% volume (shift samples read from Wave RAM right once)
  %11             |  25% volume (shift samples read from Wave RAM right twice)

### FF1D — NR33: Channel 3 wavelength low \[write-only\]

This register stores the low 8 bits of the channel's 11-bit "[wavelength](<#Frequency>)".
The upper 3 bits are stored in the low 3 bits of `NR34`.

The actual signal frequency is <math><mfrac><mn>65536</mn><mrow><mn>2048</mn><mo>-</mo><mi>wavelen</mi></mrow></mfrac></math> Hz: the higher the value, the higher the frequency.
This is the whole wave's frequency; the rate at which the channel steps through the 8 "indices" in its wave form is 32 times that, i.e. <math><mfrac><mn>2097152</mn><mrow><mn>2048</mn><mo>-</mo><mi>wavelen</mi></mrow></mfrac></math>) Hz = <math><mfrac><mn>2</mn><mrow><mn>2048</mn><mo>-</mo><mi>wavelen</mi></mrow></mfrac></math> MiHz.

::: warning DELAY

Wavelength changes (written to `NR33` or `NR34`) only take effect after the following time wave RAM is read.
([Source](https://github.com/LIJI32/SameSuite/blob/master/apu/channel_3/channel_3_freq_change_delay.asm))

:::

### FF1E — NR34: Channel 3 wavelength high & control

```
Bit 7   - Trigger (1=Restart Sound)    (Write Only)
Bit 6   - Sound Length enable          (Read/Write)
          (1=Stop output when length in NR31 expires)
Bit 2-0 - "Wavelength"'s higher 3 bits (Write Only)
```

Writing a value here with bit 7 set [triggers](<#Triggering>) the channel.

::: warning RETRIGGERING CAUTION

On monochrome consoles only, retriggering CH3 while it's about to read a byte from wave RAM causes wave RAM to be corrupted in a generally unpredictable manner.

:::

Bit 6 takes effect immediately upon writing to this register.

::: warning PLAYBACK DELAY

Triggering the wave channel does not immediately start playing wave RAM; instead, the *last* sample ever read (which is reset to 0 when the APU is off) is output until the channel next reads a sample.

:::

### FF30–FF3F — Wave pattern RAM

Wave RAM is 16 bytes long; each byte holds two "samples", each 4 bits.

As CH3 plays, it reads wave RAM left to right, upper nibble first.
That is, $FF30's upper nibble, $FF30's lower nibble, $FF31's upper nibble, and so on.

::: warning ACCESS ORDER

When CH3 is started, the first sample read is the one at index *1*, i.e. the lower nibble of the first byte, NOT the upper nibble.
([Source](https://github.com/LIJI32/SameSuite/blob/master/apu/channel_3/channel_3_first_sample.asm))

:::

Accessing wave RAM while CH3 is **active** (i.e. playing) causes accesses to misbehave:

- On AGB, reads return $FF, and writes are ignored. ([Source](https://github.com/LIJI32/SameSuite/blob/master/apu/channel_3/channel_3_wave_ram_locked_write.asm))
- On monochrome consoles, wave RAM can only be accessed on the same cycle that CH3 does.
  Otherwise, reads return $FF, and writes are ignored.
- On other consoles, the byte accessed will be the one CH3 is currently reading[^wave_access]; that is, if CH3 is currently reading one of the first two samples, the CPU will really access $FF30, regardless of the address being used. ([Source](https://github.com/LIJI32/SameSuite/blob/master/apu/channel_3/channel_3_wave_ram_locked_write.asm))

Wave RAM *can* be accessed normally even if the DAC is on, as long as the channel is not active. ([Source](https://github.com/LIJI32/SameSuite/blob/master/apu/channel_3/channel_3_wave_ram_dac_on_rw.asm))
This is especially relevant on GBA, since ["DACs" are always enabled there](<#Game Boy Advance audio>).

[^wave_access]:
The way it works is that wave RAM is a 16-byte memory buffer, and while it's playing, CH3 has priority over the CPU when choosing which of those 16 bytes is accessed.
So, from the CPU's point of view, wave RAM reads out the same byte, regardless of the address.

## Sound Channel 4 — Noise

This channel is used to output white noise[^not_white], which is done by randomly switching the amplitude between two levels fairly fast.

The frequency can be adjusted in order to make the noise appear "harder" (lower frequency) or "softer" (higher frequency).

The random function that switches the output level can also be manipulated.
Certain settings can cause the wave to be more regular, sounding closer to a pulse than noise.

[^not_white]:
By default, the noise will sound close to white; but it can be manipulated to sound differently.

### FF20 — NR41: Channel 4 length timer \[write-only\]

This register controls the channel's [length timer](<#Length timer>).

```
Bit 5-0 - length timer
```

The higher the [length timer](<#Length timer>), the shorter the time before the channel is cut.

### FF21 — NR42: Channel 4 volume & envelope

This register controls the digital amplitude produced when the LFSR outputs a 1, and the sweep applied to that setting.

```
Bit 7-4 - Initial volume of envelope (0-F) (0=No Sound)
Bit 3   - Envelope direction (0=Decrease, 1=Increase)
Bit 2-0 - Sweep pace (0=No Sweep)
```

Setting bits 3-7 of this register all to 0 turns the DAC off (and thus, the channel as well), which [may cause an audio pop](<#Mixer>).

The envelope ticks at 64 Hz, and the channel's envelope will be increased / decreased (depending on bit 3) every <var>Sweep pace</var> of those ticks.

### FF22 — NR43: Channel 4 frequency & randomness

This register allows controlling the way the amplitude is randomly switched.

```
Bit 7-4 - Clock shift (s)
Bit 3   - LFSR width (0=15 bits, 1=7 bits)
Bit 2-0 - Clock divider (r)
```

The frequency at which the LFSR is clocked is <math><mfrac><mn>262144</mn><mrow><mi>r</mi><mo>×</mo><msup><mn>2</mn><mi>s</mi></msup></mrow></mfrac></math> Hz.
(<var>r</var> = 0 is treated as <var>r</var> = 0.5 instead.)
If the bit shifted out is a 0, the channel emits a 0; otherwise, it emits the volume selected in `NR42`.

If the LFSR is set to 7-bit mode, the output will become more regular, and some frequencies will sound more like Pulse than Noise.
Note that switching from 15- to 7-bit mode when the LFSR [is in a certain state](<#Noise channel (CH4)>) can "lock it up", which essentially silences CH4; this can be avoided by retriggering CH4, which resets the LFSR.

### FF23 — NR44: Channel 4 control

```
Bit 7   - Trigger (1=Restart channel)  (Write Only)
Bit 6   - Sound Length enable          (Read/Write)
          (1=Stop output when length in NR41 expires)
```

Writing a value here with bit 7 set [triggers](<#Triggering>) the channel.

Bit 6 takes effect immediately upon writing to this register.
