# Command Packet Transfers

Command packets (aka Register Files) are transferred from the Game Boy to
the SNES by using P14 and P15 output lines of [the JOYPAD register](<#FF00 — P1/JOYP: Joypad>) (FF00).  These same lines are also used to select the two rows in the
Game Boy keyboard matrix (which still works).

## Transferring Bits

A command packet transfer must be initiated by setting both P14 and P15
to LOW; this will reset and start the ICD2 packet receiving circuit.
Data is then transferred (LSB first), setting P14=LOW will indicate a
"0" bit, and setting P15=LOW will indicate a "1" bit. For example:

```
    RESET  0   0   1   1   0   1   0
P14  --_---_---_-----------_-------_--...
P15  --_-----------_---_-------_------...
       ↑         ↑         ↑         ↑
Time   0         50       100       150
```

[The boot ROM](<#Super Game Boy (SGB, SGB2)>) and licensed software keep data and reset pulses LOW for at least 5 μs and leave P14 and P15 HIGH for at least 15 μs after each pulse.
Though the hardware is capable of receiving pulses and spaces as short as 2 μs (as tested using [sgb-speedtest]),
following the common practice of 5-cycle pulses and 15-cycle spaces may improve reliability in some corner case that the community has not yet discovered.

Obviously, it'd be no good idea to access [the joypad register](<#FF00 — P1/JOYP: Joypad>) during the transfer,
for example, in case that your VBlank interrupt procedure reads-out
joypad states each frame, so be sure to disable that interrupt during the
transfer (or disable only the joypad procedure by using a software
flag).

[sgb-speedtest]: https://github.com/zlago/sgb-speedtest/

## Transferring Packets

Each packet is invoked by a RESET pulse, then 128 bits of data are
transferred (16 bytes, LSB of first byte first), and finally, a
"0" bit must be transferred as a stop bit.
These 130 bit periods correspond to at least 2600 cycles at the recommended rate.

The structure of the first packet in a transmission is:

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
Unused bytes at the end of the last packet don't matter.
The GB program should wait 60 ms (4 frames) between each packet transfer and the next,
as the "bomb" tool to erase a user-drawn border can cause the SGB system software not to check for packets for 4 frames.

:::tip

Bytes with no indicated purpose are simply ignored by the SGB BIOS.
They can be set to any value (but they must still be transferred).

:::
