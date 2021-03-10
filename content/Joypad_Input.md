### FF00 - P1/JOYP - Joypad (R/W)

The eight Game Boy action/direction buttons are arranged as a 2x4
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

### Usage in SGB software

Beside for normal joypad input, SGB games mis-use the joypad register to
output SGB command packets to the SNES, also, SGB programs may read out
gamepad states from up to four different joypads which can be connected
to the SNES. See SGB description for details.

### INT 60 - Joypad Interrupt

The Joypad interrupt is requested when any of the bits 0-3 change
from High to Low. This happens when a button is
pressed (provided that the action/direction buttons are enabled by
bit 5/4, respectively), however, due to switch bounce, one or more High to Low
transitions are usually produced when pressing a button.

### Using the Joypad Interrupt

It's practically useless for programmers. Even when selecting both
action and direction buttons simultaneously, it still cannot recognize all
presses, because in that case a bit might be already held Low by an
action button, and pressing the corresponding direction button would thus
make no difference. The only meaningful purpose of the Joypad
interrupt would be to terminate the STOP (low power) standby state. GBA SP,
because of the different buttons used, seems to not be affected by
switch bounce.

