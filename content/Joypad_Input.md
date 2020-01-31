### FF00 - P1/JOYP - Joypad (R/W)

The eight Game Boy buttons/direction keys are arranged in form of a 2x4
matrix. Select either button or direction keys by writing to this
register, then read-out bit 0-3.

```
Bit 7 - Not used
Bit 6 - Not used
Bit 5 - P15 Select Button Keys      (0=Select)
Bit 4 - P14 Select Direction Keys   (0=Select)
Bit 3 - P13 Input Down  or Start    (0=Pressed) (Read Only)
Bit 2 - P12 Input Up    or Select   (0=Pressed) (Read Only)
Bit 1 - P11 Input Left  or Button B (0=Pressed) (Read Only)
Bit 0 - P10 Input Right or Button A (0=Pressed) (Read Only)
```

::: tip NOTE
Most programs are repeatedly reading from this port several times
(the first reads used as short delay, allowing the inputs to stabilize,
and only the value from the last read actually used).
:::

### Usage in SGB software

Beside for normal joypad input, SGB games mis-use the joypad register to
output SGB command packets to the SNES, also, SGB programs may read out
gamepad states from up to four different joypads which can be connected
to the SNES. See SGB description for details.

### INT 60 - Joypad Interrupt

Joypad interrupt is requested when any of the above Input lines changes
from High to Low. Generally this should happen when a key becomes
pressed (provided that the button/direction key is enabled by above
Bit4/5), however, because of switch bounce, one or more High to Low
transitions are usually produced both when pressing or releasing a key.

### Using the Joypad Interrupt

It's more or less useless for programmers, even when selecting both
buttons and direction keys simultaneously it still cannot recognize all
keystrokes, because in that case a bit might be already held low by a
button key, and pressing the corresponding direction key would thus
cause no difference. The only meaningful purpose of the keystroke
interrupt would be to terminate STOP (low power) standby state. GBA SP,
because of the different buttons used, seems to not be affected by
switch bounce.

