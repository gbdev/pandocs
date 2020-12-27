Communication between two Game Boy systems happens one byte at a time. One
Game Boy generates a clock signal internally and thus controls
when the exchange happens. The other one uses an external clock (that is,
the one from the other Game Boy) and has no control over when the
transfer happens. If it hasn't gotten around to loading up the next
data byte at the time the transfer begins, the last one will go out
again. Alternately, if it's ready to send the next byte but the last
one hasn't gone out yet, it has no choice but to wait.

### FF01 - SB - Serial transfer data (R/W)

Before a transfer, it holds the next byte that will go out.

During a transfer, it has a blend of the outgoing and incoming bytes.
Each cycle, the leftmost bit is shifted out (and over the wire) and the
incoming bit is shifted in from the other side:

```
o7 o6 o5 o4 o3 o2 o1 o0
o6 o5 o4 o3 o2 o1 o0 i7
o5 o4 o3 o2 o1 o0 i7 i6
o4 o3 o2 o1 o0 i7 i6 i5
o3 o2 o1 o0 i7 i6 i5 i4
o2 o1 o0 i7 i6 i5 i4 i3
o1 o0 i7 i6 i5 i4 i3 i2
o0 i7 i6 i5 i4 i3 i2 i1
i7 i6 i5 i4 i3 i2 i1 i0
```

### FF02 - SC - Serial Transfer Control (R/W)

```
Bit 7 - Transfer Start Flag (0=No transfer is in progress or requested, 1=Transfer in progress, or requested)
Bit 1 - Clock Speed (0=Normal, 1=Fast) ** CGB Mode Only **
Bit 0 - Shift Clock (0=External Clock, 1=Internal Clock)
```

The primary Game Boy will load up a data byte in SB and then set
SC to 0x81 (Transfer requested, use internal clock). It will be notified
that the transfer is complete in two ways: SC's Bit 7 will be cleared
(that is, SC will be set up 0x01), and also the Serial Interrupt handler
will be called (that is, the CPU will jump to 0x0058).

The other Game Boy will load up a data byte and can optionally set SC's
Bit 7 (that is, SC=0x80). Regardless of whether or not it has done this, if
and when the primary Game Boy wants to conduct a transfer, it will happen
(pulling whatever happens to be in SB at that time). The externally clocked
Game Boy will have its serial interrupt handler called at the end of the
transfer, and if it bothered to set SC's Bit 7, it will be cleared.

### Internal Clock

In Non-CGB Mode the Game Boy supplies an internal clock of 8192Hz only
(allowing to transfer about 1 KByte per second minus overhead for delays).
In CGB Mode four internal clock rates are available, depending on Bit 1
of the SC register, and on whether the CGB Double Speed Mode is used:

```
   8192Hz -  1KB/s - Bit 1 cleared, Normal
  16384Hz -  2KB/s - Bit 1 cleared, Double Speed Mode
 262144Hz - 32KB/s - Bit 1 set,     Normal
 524288Hz - 64KB/s - Bit 1 set,     Double Speed Mode
```

### External Clock

The external clock is typically supplied by another Game Boy, but might
be supplied by another computer (for example if connected to a PCs
parallel port), in that case the external clock may have any speed. Even
the old/monochrome Game Boy is reported to recognizes external clocks of
up to 500KHz. And there is no limitation into the other direction - even
when suppling an external clock speed of "1 bit per month", then the
Game Boy will still eagerly wait for the next bit(s) to be transferred.
It isn't required that the clock pulses are sent at an regular interval
either.

### Timeouts

When using external clock then the transfer will not complete until the
last bit is received. In case that the second Game Boy isn't supplying a
clock signal, if it gets turned off, or if there is no second Game Boy
connected at all) then transfer will never complete. For this reason the
transfer procedure should use a timeout counter, and abort the
communication if no response has been received during the timeout
interval.

### Delays and Synchronization

The primary Game Boy should always execute a small
delay after each transfer, in order to ensure that the opponent
Game Boy has enough time to prepare itself for the next transfer, that is, the
Game Boy with external clock must have set its transfer start bit before
the Game Boy with internal clock starts the transfer. Alternately, the
two Game Boy systems could switch between internal and external clock for each
transferred byte to ensure synchronization.

Transfer is initiated by setting the primary Game Boy setting its Transfer
Start Flag, regardless of the value of this flag on the other device.
This bit is automatically set to 0 (on both) at the end of Transfer.
Reading this bit can be used to determine if the transfer is still
active.

### INT 58 - Serial Interrupt

When the transfer has completed (that is, after sending/receiving 8 bits, if
any) then an interrupt is requested by setting Bit 3 of the IF Register
(FF0F). When that interrupt is enabled, then the Serial Interrupt vector
at 0058 is called.

**XXXXXX\...**

Transmitting and receiving serial data is done simultaneously. The
received data is automatically stored in SB.

The serial I/O port on the Game Boy is a very simple setup and is crude
compared to standard RS-232 (IBM-PC) or RS-485 (Macintosh) serial ports.
There are no start or stop bits.

During a transfer, a byte is shifted in at the same time that a byte is
shifted out. The rate of the shift is determined by whether the clock
source is internal or external. The most significant bit is shifted in
and out first.

When the internal clock is selected, it drives the clock pin on the game
link port and it stays high when not used. During a transfer it will go
low eight times to clock in/out each bit.

The state of the last bit shifted out determines the state of the output
line until another transfer takes place.

If a serial transfer with internal clock is performed and no external
Game Boy is present, a value of \$FF will be received in the transfer.

The following code initiates the process of shifting \$75 out the serial
port and a byte to be shifted into \$FF01:

```
   ld   a,$75
   ld  ($FF01),a
   ld   a,$81
   ld  ($FF02),a
```

The Game Boy does not support wake-on-LAN. Completion of an externally
clocked serial transfer does not exit STOP mode.

