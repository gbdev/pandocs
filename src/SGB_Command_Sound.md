# Sound Functions

## SGB Command $08 — SOUND

Used to start/stop internal sound effect, start/stop sound using
internal tone data.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1     Sound Effect A (Port 1) Decrescendo 8-bit Sound Code
 2     Sound Effect B (Port 2) Sustain     8-bit Sound Code
 3     Sound Effect Attributes
         Bit 0-1 - Sound Effect A Pitch  (0..3=Low..High)
         Bit 2-3 - Sound Effect A Volume (0..2=High..Low, 3=Mute on)
         Bit 4-5 - Sound Effect B Pitch  (0..3=Low..High)
         Bit 6-7 - Sound Effect B Volume (0..2=High..Low, 3=Not used)
 4     Music Score Code (must be zero if not used)
 5-F   Not used (zero)
```

See Sound Effect Tables below for a list of available pre-defined
effects.

Notes:

1.  Mute is only active when both bits D2 and D3 are 1.
2.  When the volume is set for either Sound Effect A or Sound Effect B,
    mute is turned off.
3.  When Mute on/off has been executed, the sound fades out/fades in.
4.  Mute on/off operates on the (BGM) which is reproduced by Sound
    Effect A, Sound Effect B, and the Super NES APU. A "mute off" flag
    does not exist by itself. When mute flag is set, volume and pitch of
    Sound Effect A (port 1) and Sound Effect B (port 2) must be set.

## SGB Command $09 — SOU_TRN

Used to transfer sound code or data to SNES Audio Processing Unit memory
(APU-RAM).

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1-F   Not used (zero)
```

The sound code/data is sent by [VRAM transfer](<#VRAM Transfers>) as a contiguous list of "packets".

All 16-bit values are little-endian.

Data transfer packet format:
```
 0-1    Size of data below (N); if zero, this is instead a jump packet
 2-3    Destination address in S-APU RAM (typically $2B00, see below)
 4-N+3  Data to be transferred
```

Jump packet format:
```
 0-1  Must be $0000
 2-3  S-APU jump address, use $0400 to safely restart the built-in SGB BIOS' N-SPC sound engine
```

Possible destinations in APU-RAM are:

Memory range | Description
-------------|-------------
 $0400-2AFF  | APU-RAM Program Area (9.75KBytes)
 $2B00-4AFF  | APU-RAM Sound Score Area (8Kbytes)
 $4DB0-EEFF  | APU-RAM Sampling Data Area (40.25 Kbytes)

This function may be used to take control of the SNES sound chip, and/or
to access the SNES MIDI engine. In either case it requires deeper
knowledge of SNES sound programming.

## SGB Sound Effect A/B Tables

Below lists the digital sound effects that are pre-defined in the
SGB BIOS, and which can be used with the SGB "SOUND" Command.
Effect A and B may be simultaneously used.
Sound Effect A uses channels 6 and 7, Sound Effect B uses channels
0, 1, 4 and 5. Effects that use less channels will use only the upper channels
(eg. 4 and 5 for a B Effect with only two channels).

## Sound Effect A Flag Table

Code | Description             | Recommended pitch | Nb of channels used
-----|-------------------------|-------------------|--------------------
 00  | Dummy flag, re-trigger  |  -                |  2
 01  | Nintendo                |  3                |  1
 02  | Game Over               |  3                |  2
 03  | Drop                    |  3                |  1
 04  | OK ... A                |  3                |  2
 05  | OK ... B                |  3                |  2
 06  | Select...A              |  3                |  2
 07  | Select...B              |  3                |  1
 08  | Select...C              |  2                |  2
 09  | Mistake...Buzzer        |  2                |  1
 0A  | Catch Item              |  2                |  2
 0B  | Gate squeaks 1 time     |  2                |  2
 0C  | Explosion...small       |  1                |  2
 0D  | Explosion...medium      |  1                |  2
 0E  | Explosion...large       |  1                |  2
 0F  | Attacked...A            |  3                |  1
 10  | Attacked...B            |  3                |  2
 11  | Hit (punch)...A         |  0                |  2
 12  | Hit (punch)...B         |  0                |  2
 13  | Breath in air           |  3                |  2
 14  | Rocket Projectile...A   |  3                |  2
 15  | Rocket Projectile...B   |  3                |  2
 16  | Escaping Bubble         |  2                |  1
 17  | Jump                    |  3                |  1
 18  | Fast Jump               |  3                |  1
 19  | Jet (rocket) takeoff    |  0                |  1
 1A  | Jet (rocket) landing    |  0                |  1
 1B  | Cup breaking            |  2                |  2
 1C  | Glass breaking          |  1                |  2
 1D  | Level UP                |  2                |  2
 1E  | Insert air              |  1                |  1
 1F  | Sword swing             |  1                |  1
 20  | Water falling           |  2                |  1
 21  | Fire                    |  1                |  1
 22  | Wall collapsing         |  1                |  2
 23  | Cancel                  |  1                |  2
 24  | Walking                 |  1                |  2
 25  | Blocking strike         |  1                |  2
 26  | Picture floats on & off |  3                |  2
 27  | Fade in                 |  0                |  2
 28  | Fade out                |  0                |  2
 29  | Window being opened     |  1                |  2
 2A  | Window being closed     |  0                |  2
 2B  | Big Laser               |  3                |  2
 2C  | Stone gate closes/opens |  0                |  2
 2D  | Teleportation           |  3                |  1
 2E  | Lightning               |  0                |  2
 2F  | Earthquake              |  0                |  2
 30  | Small Laser             |  2                |  2
 80  | Effect A, stop/silent   |  -                |  2

Sound effect A is used for formanto sounds (percussion sounds).

## Sound Effect B Flag Table

Code | Description              | Recommended pitch | Nb of channels used
-----|--------------------------|-------------------|--------------------
 00  | Dummy flag, re-trigger   |  -                |  4
 01  | Applause...small group   |  2                |  1
 02  | Applause...medium group  |  2                |  2
 03  | Applause...large group   |  2                |  4
 04  | Wind                     |  1                |  2
 05  | Rain                     |  1                |  1
 06  | Storm                    |  1                |  3
 07  | Storm with wind/thunder  |  2                |  4
 08  | Lightning                |  0                |  2
 09  | Earthquake               |  0                |  2
 0A  | Avalanche                |  0                |  2
 0B  | Wave                     |  0                |  1
 0C  | River                    |  3                |  2
 0D  | Waterfall                |  2                |  2
 0E  | Small character running  |  3                |  1
 0F  | Horse running            |  3                |  1
 10  | Warning sound            |  1                |  1
 11  | Approaching car          |  0                |  1
 12  | Jet flying               |  1                |  1
 13  | UFO flying               |  2                |  1
 14  | Electromagnetic waves    |  0                |  1
 15  | Score UP                 |  3                |  1
 16  | Fire                     |  2                |  1
 17  | Camera shutter, formanto |  3                |  4
 18  | Write, formanto          |  0                |  1
 19  | Show up title, formanto  |  0                |  1
 80  | Effect B, stop/silent    |  -                |  4

Sound effect B is mainly used for looping sounds (sustained sounds).
