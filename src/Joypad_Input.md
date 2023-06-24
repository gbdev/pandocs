# Joypad Input

## FF00 — P1/JOYP: Joypad

The eight Game Boy action/direction buttons are arranged as a 2×4
matrix. Select either action or direction buttons by writing to this
register, then read out the bits 0-3.

```
Bit 7 - Not used
Bit 6 - Not used
Bit 5 - P15 Select Action buttons    (0=Select)
Bit 4 - P14 Select Direction buttons (0=Select)
Bit 3 - P13 Input: Down  or Start    (0=Pressed) (Read Only)
Bit 2 - P12 Input: Up    or Select   (0=Pressed) (Read Only)
Bit 1 - P11 Input: Left  or B        (0=Pressed) (Read Only)
Bit 0 - P10 Input: Right or A        (0=Pressed) (Read Only)
```

::: tip NOTE

Most programs read from this port several times in a row
(the first reads are used as a short delay, allowing the inputs to stabilize,
and only the value from the last read is actually used).

:::

## Usage in SGB software

Beside for normal joypad input, SGB games misuse the joypad register to
output SGB command packets to the SNES, also, SGB programs may read out
gamepad states from up to four different joypads which can be connected
to the SNES. See SGB description for details.
