# 4-Player Adapter

The 4-Player Adapter (DMG-07) is an accessory that allows 4 Game Boys
to connect for multiplayer via [serial data transfer](./Serial_Data_Transfer_(Link_Cable).md).
The device is primarily designed for DMG consoles, with later models
requiring Link Cable adapters.

## Communication

The DMG-07 protocol can be divided into 2 sections, the "ping" phase, and
the "transmission" phase. The initial ping phase involves sending packets
back and forth between connected Game Boys probing for their current
connection status. Afterwards, the DMG-07 enters into transmission mode
where the Game Boys exchange data across the network.

A very important thing to note is that all Game Boys transfer data across
the DMG-07 via an external clock source. Apparently, the clock source is
provided by the DMG-07 itself. Trying to send data via an internal clock
results in garbage data.

### Ping Phase

When a "master" Game Boy (Player 1) is first connected to the adapter,
setting Bit 7 of SC to 1 and setting Bit 0 of SC to 0 causes the accessory
to send out "ping" packets periodically. All connected Game Boys will
receive 4 bytes as part of the ping packet at a rate of about 2048 bits
per second, or about 256 bytes per second. Essentially, the ping seems to
run 1/4 as fast as the clock used for normal serial transfers on the DMG
(1KB/s). The ping data looks like this:

Byte | Value | Description
-----|-------|-------------
  1  | \$FE  | ID Byte
  2  |   ??  | STAT1
  3  |   ??  | STAT2
  4  |   ??  | STAT3

3 "STAT" bytes are sent indicating the current connection status of the other
Game Boys. Each byte is usually the same, however, sometimes the status can
change midway through a ping, typically on STAT2 or STAT3. Each STAT byte
looks like such:

Bit | Name
----|------------------------
 7  | Player 4 Connected
 6  | Player 3 Connected
 5  | Player 2 Connected
 4  | Player 1 Connected
0-2 | Player ID (1-4)

The Player ID's value is determined by whichever port a Game Boy is connected
to. As more Game Boys connect, the upper bits of the STAT bytes are turned on.

When talking about Game Boys and the "connection", this refers to a Game Boy
properly responding to STAT1 and STAT2 bytes when receiving a ping packet from
the DMG-07. In this way, the Game Boy broadcasts across the Link Cable network
that it is an active participant in communications. It also acts as a sort of
acknowledgement signal, where software can drop a Game Boy if the DMG-07
detects an improper response during a ping, or a Game Boy simply quits the
network. The proper response is to send \$88 *after* receiving the ID Byte and
STAT1, in which case the upper-half of STAT1, STAT2, and STAT3 are updated to
show that a Game Boy is "connected". If for whatever reason, the
acknowledgement codes are not sent, the above bits are unset.

Some examples of ping packets are shown below:

Packet        | Description
--------------|-------------------------------------------------------
`FE 01 01 01` | Ping packet received by Player 1 with no other Game Boys connected.
`FE 11 11 11` | Ping packet received by Player 1 when Player 1 has connected.
`FE 31 31 31` | Ping packet received by Player 1 when Players 1 & 2 have connected.
`FE 71 71 71` | Ping packet received by Player 1 when Players 1, 2, & 3 have connected.
`FE 62 62 62` | Ping packet received by Player 2 when Players 2 & 3 are connected (but not Player 1).

It's possible to have situations where some players are connected but others
are not; the gaps don't matter. For example, Player 1 and Player 4 can be
connected, while Player 2 and Player 3 can be disconnected (or non-existent,
same thing); most games do not care so long as Player 1 is active, as that
Game Boy acts as master and orchestrates the multiplayer session from a
software point of view. Because of the way the DMG-07 hardcodes player IDs
based on which port a Game Boy is physically connected to, in the above
situation Player 4 wouldn't suddenly become Player 2.

During the ping phase, the master Game Boy is capable of setting up two
parameters that will be used during the transmission phase. The clock rate for
the transmission phase can be adjusted, as well as the packet size each Game
Boy will use. The master Game Boy needs to respond with one byte for STAT2
and STAT3 respectively. The chart below illustrates how a master Game Boy
should respond to all bytes in a ping packet:

```
----------------------------
DMG-07		Game Boy
----------------------------
\$FE	<-->	(ACK1) = \$88
STAT1	<-->	(ACK2) = \$88	
STAT2	<-->	(RATE) = Link Cable Speed 
STAT3	<-->	(SIZE) = Packet Size
```

The new clock rate is only applied when entering the transmission phase; the
ping phase runs at a constant 2048 bits-per-second. The formula for the new
clock rate is as follows:

```
DMG-07 Bits-Per-Second --> 4194304 / ((6 * RATE) + 512)
```

The lowest setting (RATE = 0) runs the DMG-07 at the normal speed DMGs usually
transfer data (1KB/s), while setting it to \$FF runs it close to the slowest
speed (2042 bits-per-second).

SIZE sets the length of packets exchanged between all Game Boys. Nothing fancy,
just the number of bytes in each packet. It probably shouldn't be set to zero.

### Transmission Phase

When the master Game Boy (Player 1) is ready, it should send 4 bytes
(`AA AA AA AA`, if those are actually required should be investigated further).
This alerts the DMG-07 to start the transmission phase. The RATE and SIZE parameters 
are applied at this point. The protocol is simple: Each Game Boy sends a packet to
the DMG-07 simultaneously, then the DMG-07 outputs each packet to all connected
Game Boys. All data is buffered, so there is a 4 packet delay after each Game
Boy submits their data (the delay is still 4 packets long even if some Game Boys
are not connected). For example, say the packet size is 4 bytes; the flow of
data would look like this when sending:

P1 send       | P2 send       | P3 send       | P4 send       | Transfer count
--------------|---------------|---------------|---------------|-----------------------------
P1 (byte 1)   | P2 (byte 1)   | P3 (byte 1)   | P4 (byte 1)   | 0
P1 (byte 2)   | P2 (byte 2)   | P3 (byte 2)   | P4 (byte 2)   | 1
P1 (byte 3)   | P2 (byte 3)   | P3 (byte 3)   | P4 (byte 3)   | 2
P1 (byte 4)   | P2 (byte 4)   | P3 (byte 4)   | P4 (byte 4)   | 3
0             | 0             | 0             | 0             | 4 (Typically supposed to be zero, but DMG-07 ignores anything here)
0             | 0             | 0             | 0             | 5
0             | 0             | 0             | 0             | 6
0             | 0             | 0             | 0             | 7
0             | 0             | 0             | 0             | 8
0             | 0             | 0             | 0             | 9
0             | 0             | 0             | 0             | 10
0             | 0             | 0             | 0             | 11
0             | 0             | 0             | 0             | 12
0             | 0             | 0             | 0             | 13
0             | 0             | 0             | 0             | 14
0             | 0             | 0             | 0             | 15

And when receiving, the flow of data would look like this:

P1 receive    | P2 receive    | P3 receive    | P4 receive    | Transfer count
--------------|---------------|---------------|---------------|-----------------------------
P1 (byte 1)   | P1 (byte 1)   | P1 (byte 1)   | P1 (byte 1)   | 16
P1 (byte 2)   | P1 (byte 2)   | P1 (byte 2)   | P1 (byte 2)   | 17
P1 (byte 3)   | P1 (byte 3)   | P1 (byte 3)   | P1 (byte 3)   | 18
P1 (byte 4)   | P1 (byte 4)   | P1 (byte 4)   | P1 (byte 4)   | 19
P2 (byte 1)   | P2 (byte 1)   | P2 (byte 1)   | P2 (byte 1)   | 20
P2 (byte 2)   | P2 (byte 2)   | P2 (byte 2)   | P2 (byte 2)   | 21
P2 (byte 3)   | P2 (byte 3)   | P2 (byte 3)   | P2 (byte 3)   | 22
P2 (byte 4)   | P2 (byte 4)   | P2 (byte 4)   | P2 (byte 4)   | 23
P3 (byte 1)   | P3 (byte 1)   | P3 (byte 1)   | P3 (byte 1)   | 24
P3 (byte 2)   | P3 (byte 2)   | P3 (byte 2)   | P3 (byte 2)   | 25
P3 (byte 3)   | P3 (byte 3)   | P3 (byte 3)   | P3 (byte 3)   | 26
P3 (byte 4)   | P3 (byte 4)   | P3 (byte 4)   | P3 (byte 4)   | 27
P4 (byte 1)   | P4 (byte 1)   | P4 (byte 1)   | P4 (byte 1)   | 28
P4 (byte 2)   | P4 (byte 2)   | P4 (byte 2)   | P4 (byte 2)   | 29
P4 (byte 3)   | P4 (byte 3)   | P4 (byte 3)   | P4 (byte 3)   | 30
P4 (byte 4)   | P4 (byte 4)   | P4 (byte 4)   | P4 (byte 4)   | 31

Again, due to buffering, data output to the DMG-07 is actually delayed by
several transfers according to the size of the packets. All connected Game
Boys should send their data into the buffer during the first few transfers.
Here, the packet size is 4 bytes, so each Game Boy should submit their data
during the first 4 transfers. The other 12 transfers don't care what the
Game Boys send; it won't enter into the buffer. The next 16 transfers return
the packets each Game Boy previously sent (if no Game Boy exists for player,
that slot is filled with zeroes).

With the buffering system, Game Boys would normally be reading data from
previous packets during transfers 0-15, in addition to sending new packets.
Likewise, during transfers 16-19 each Game Boy is sending new packets. In
effect, while receiving old data, Game Boys are supposed to pump new data into
the network.

When the DMG-07 enters the transmission phase, the buffer is initially filled
with garbage data that is based on output the master Game Boy had sent during
the ping phase. At this time, it is recommended to ignore the earliest packets
received, however, it is safe to start putting new, relevant data into the
buffer.

### Restarting Ping Phase

It's possible to restart the ping phase while operating in the transmission
phase. To do so, the master Game Boy should send 4 or more bytes
(`FF FF FF FF`, it's possible fewer \$FF bytes need to be sent,
but this has not been extensively investigated yet). The bytes alert the DMG-07
that the ping phase should begin again, after which it sends ping packets after
a brief delay. During this delay, the transmission protocol is still working as
intended until the switch happens.
