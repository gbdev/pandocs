# VRAM Transfers

## Overview

Beside for the packet transfer method, larger data blocks of 4KBytes can
be transferred by using the video signal. These transfers are invoked by
first sending one of the commands with the ending \_TRN (by using normal
packet transfer), the 4K data block is then read-out by the SNES from
Game Boy display memory during the next frame.

## Transfer Data

Normally, transfer data should be stored at 8000-8FFF in Game Boy VRAM,
even though the SNES receives the data in from display scanlines, it
will automatically re-produce the same ordering of bits and bytes, as
being originally stored at 8000-8FFF in Game Boy memory.

## Preparing the Display

The above method works only when recursing the following things: BG Map
must display unsigned characters $00-$FF on the screen; $00..$13 in
first line, $14..$27 in next line, etc. The Game Boy display must be
enabled, the display may not be scrolled, objects (sprites) should not overlap
the background tiles, the BGP palette register must be set to $E4.

## Transfer Time

Note that the transfer data should be prepared in VRAM **before** sending
the transfer command packet. The actual transfer starts at the beginning
of the next frame after the command has been sent, and the transfer ends
at the end of the 5th frame after the command has been sent (not
counting the frame in which the command has been sent). The displayed
data must not be modified during the transfer, as the SGB reads it in
multiple chunks.

## Avoiding Screen Garbage

The display will contain "garbage" during the transfer, this
dirt-effect can be avoided by freezing the screen (in the state which
has been displayed before the transfer) by using the MASK_EN command.
Of course, this works only when actually executing the game on an SGB
(and not on handheld Game Boy systems), it'd be thus required to detect
the presence of SGB hardware before blindly sending VRAM data.
