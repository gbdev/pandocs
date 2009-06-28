The following can be used to recude the power consumption of the
gameboy, and to extend the life of the batteries.

PWR Using the HALT Instruction
------------------------------

It is recommended that the HALT instruction be used whenever possible to
reduce power consumption & extend the life of the batteries. This
command stops the system clock reducing the power consumption of both
the CPU and ROM.

The CPU will remain suspended until an interrupt occurs at which point
the interrupt is serviced and then the instruction immediately following
the HALT is executed.

Depending on how much CPU time is required by a game, the HALT
instruction can extend battery life anywhere from 5 to 50% or possibly
more.

When waiting for a vblank event, this would be a BAD example:

` @@wait:`\
`  ld   a,(0FF44h)      ;LY`\
`  cp   a,144`\
`  jr   nz,@@wait`

A better example would be a procedure as shown below. In this case the
vblank interrupt must be enabled, and your vblank interrupt procedure
must set vblank\_flag to a non-zero value.

`  ld   hl,vblank_flag  ;hl=pointer to vblank_flag`\
`  xor  a               ;a=0`\
` @@wait:               ;wait...`\
`  halt                 ;suspend CPU - wait for ANY interrupt`\
`  cp   a,(hl)          ;vblank flag still zero?`\
`  jr   z,@@wait        ;wait more if zero`\
`  ld   (hl),a          ;set vblank_flag back to zero`

The vblank\_flag is used to determine whether the HALT period has been
terminated by a vblank interrupt, or by another interrupt. In case that
your program has all other interrupts disabled, then it would be proof
to replace the above procedure by a single HALT instruction.

PWR Using the STOP Instruction
------------------------------

The STOP instruction is intended to switch the gameboy into VERY low
power standby mode. For example, a program may use this feature when it
hasn\'t sensed keyboard input for a longer period (assuming that
somebody forgot to turn off the gameboy).

Before invoking STOP, it might be required to disable Sound and Video
manually (as well as IR-link port in CGB). Much like HALT, the STOP
state is terminated by interrupt events - in this case this would be
commonly a joypad interrupt. The joypad register might be required to be
prepared for STOP either.

PWR Disabeling the Sound Controller
-----------------------------------

If your programs doesn\'t use sound at all (or during some periods) then
write 00h to register FF26 to save 16% or more on GB power consumption.
Sound can be turned back on by writing 80h to the same register, all
sound registers must be then re-initialized. When the gameboy becomes
turned on, sound is enabled by default, and must be turned off manually
when not used.

PWR Not using CGB Double Speed Mode
-----------------------------------

Because CGB Double Speed mode consumes more power, it\'d be recommended
to use normal speed when possible. There\'s limited ability to switch
between both speeds, for example, a game might use normal speed in the
title screen, and double speed in the game, or vice versa. However,
during speed switch the display collapses for a short moment, so that
it\'d be no good idea to alter speeds within active game or title screen
periods.

PWR Using the Skills
--------------------

Most of the above power saving methods will produce best results when
using efficient and tight assembler code which requires as less CPU
power as possible. Thus, experienced old-school programmers will
(hopefully) produce lower power consumption, as than HLL-programming
teenagers, for example.

