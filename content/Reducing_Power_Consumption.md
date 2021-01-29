The following programming techniques can be used to reduce the power
consumption of the Game Boy hardware and extend the life of the
batteries.

# Using the HALT Instruction

The HALT instruction should be used whenever possible to reduce power
consumption & extend the life of the batteries. This command stops the
system clock, reducing the power consumption of both the CPU and ROM.

The CPU will remain stopped until an interrupt occurs at which point the
interrupt is serviced and then the instruction immediately following the
HALT is executed.

Depending on how much CPU time is required by a game, the HALT
instruction can extend battery life anywhere from 5 to 50% or possibly
more.

When waiting for a vblank event, this would be a BAD example:

```
 @@wait:
  ld   a,(0FF44h)      ;LY
  cp   a,144
  jr   nz,@@wait
```

A better example would be a procedure as shown below. In this case the
vblank interrupt must be enabled, and your vblank interrupt procedure
must set vblank_flag to a non-zero value.

```
  ld   hl,vblank_flag  ;hl=pointer to vblank_flag
  xor  a               ;a=0
 @@wait:               ;wait...
  halt                 ;suspend CPU - wait for ANY interrupt
  cp   a,(hl)          ;vblank flag still zero?
  jr   z,@@wait        ;wait more if zero
  ld   (hl),a          ;set vblank_flag back to zero
```
The vblank_flag is used to determine whether the HALT period has been
terminated by a vblank interrupt, or by another interrupt. In case your
program has all other interrupts disabled, then it would be okay to
replace the above procedure by a single HALT instruction.

Another possibility is, if your game uses no other interrupt than VBlank
(or uses no interrupt), to only enable VBlank interrupts and simply use
a halt instruction, which will only resume main code execution when a
VBlank occurs.

Remember when using HALT to wait between VBlanks, your interrupt
routines MUST enable interrupts (ie with ei during the execution, or
better, using the RETI instruction)

# Using the STOP Instruction

The STOP instruction is intended to switch the Game Boy into VERY low
power standby mode. For example, a program may use this feature when it
hasn't sensed keyboard input for a longer period (for example, when
somebody forgot to turn off the Game Boy).

No licenced rom makes use of STOP outside of CGB speed switching.
Special care need to be taken if you want to make use of the STOP
instruction.

On a DMG, disable the LCD before invoking STOP, leaving the LCD enabled
will draw a horizontal black line on the screen and damage the hardware.

On CGB, leaving the LCD enabled when invoking STOP will return in a
black screen. Except if the LCD is in Mode3, where it will keep drawing
the current screen.

STOP is terminated by one of the P10 to P13 lines going low. Which means
before entering STOP P14 and/or P15 should be asserted by writting $00,
$10 or $20 to the P1 register, depending on which buttons you want
to terminate the STOP on.

If STOP is invoked while a button is helt down, it might enter a bugged
STOP mode. Which functions like a HALT instead of STOP, so requires
interrupts to be enabled.

# Disabling the Sound Controller

If your program doesn't use sound at all (or during some periods) then
write 00h to register FF26 to save 16% or more on GB power consumption.
Sound can be turned back on by writing 80h to the same register, all
sound registers must be then re-initialized. When the Game Boy is turned
on, sound is enabled by default, and must be turned off manually when
not used.

# Not using CGB Double Speed Mode

Because CGB Double Speed mode consumes more power, it's recommended to
use normal speed when possible. There's limited ability to switch
between both speeds, for example, a game might use normal speed in the
title screen, and double speed in the game, or vice versa. However,
during speed switch, the display collapses for a short moment, so it's
not a good idea to alter speeds within active game or title screen
periods.

# Using the Skills

Most of the above power saving methods will produce best results when
using efficient and tight assembler code which requires as little CPU
power as possible. Using a high level language will require more CPU
power and these techniques will not have as big as an effect.

To optimize your code, it might be a good idea to look at [this
page](http://wikiti.brandonw.net/index.php?title=Z80_Optimization),
although it applies to the original Z80 CPU, so one must adapt the
optimizations to the GBZ80.

