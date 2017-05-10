There is a flaw in the GameBoy hardware that causes trash to be written
to OAM RAM if the following commands are used while their 16-bit content
is in the range of \$FE00 to \$FEFF:

` inc rr        dec rr          ;rr = bc,de, or hl`\
` ldi a,(hl)    ldd a,(hl)`\
` ldi (hl),a    ldd (hl),a`

Only sprites 1 & 2 (\$FE00 & \$FE04) are not affected by these
instructions.

Game Boy Color and Advance are not affected by this bug.

