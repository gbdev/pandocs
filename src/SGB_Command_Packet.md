# Command Packet Transfers

Command packets (aka Register Files) are transferred from the Game Boy to
the SNES by using P14 and P15 output lines of the JOYPAD register
(FF00h).  These same lines are also used to select the two rows in the
Game Boy keyboard matrix (which still works).

## Transferring Bits

A command packet transfer must be initiated by setting both P14 and P15
to LOW, this will reset and start the SNES packet receiving program.
Data is then transferred (LSB first), setting P14=LOW will indicate a
"0" bit, and setting P15=LOW will indicate a "1" bit. For example:

```
    RESET  0   0   1   1   0   1   0
P14  --_---_---_-----------_-------_--...
P15  --_-----------_---_-------_------...
```

Data and reset pulses must be kept LOW for at least 5us. P14 and P15
must be kept both HIGH for at least 15us between any pulses. Obviously,
it'd be no good idea to access the JOYPAD register during the transfer,
for example, in case that your VBlank interrupt procedure reads-out
joypad states each frame, be sure to disable that interrupt during the
transfer (or disable only the joypad procedure by using a software
flag).

## Transferring Packets

Each packet is invoked by a RESET pulse, then 128 bits of data are
transferred (16 bytes, LSB of first byte first), and finally, a
"0" bit must be transferred as stop bit. The structure of normal
packets thus is:

1. 1 pulse: Start signal
2. 1 byte: Header byte (Command Code \* 8 + Length)
3. 15 bytes: Parameter Data
4. 1 bit: Stop Bit (0)

The above "Length" indicates the total number of packets (1-7,
including the first packet) which will be sent.  If more than 15
parameter bytes are used, then further packet(s) will follow, as such:

1. 1 pulse: Start signal
2. 16 bytes: Parameter Data
3. 1 bit: Stop Bit (0)

By using all 7 packets, up to 111 data bytes (15+16\*6) may be sent.
Unused bytes at the end of the last packet don't matter. A 60ms (4
frames) delay should be invoked between each packet transfer.
