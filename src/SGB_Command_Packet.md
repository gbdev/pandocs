# Command Packet Transfers

Command packets are transferred from the Game Boy to the SNES by using bits 6 and 7 of of [the `JOYP` register][JOYP].

## Transferring Bits

A command packet transfer must be initiated by setting [`JOYP`][JOYP] bits 6 and 7 both to 0; this will reset and start the ICD2 packet receiving circuit.
Data is then transferred (LSB first), setting bit 6 to 0 will indicate a `0` bit, and setting bit 7 to 0 will indicate a `1` bit.
For example:

{{#include imgs/sgb_bits.svg}}

[The boot ROM](<#Super Game Boy (SGB, SGB2)>) and licensed software keep data and reset pulses LOW for at least 5 M-cycles and leave bit 6 and 7 both to 1 for at least 15 M-cycles after each pulse.
Though the hardware is capable of receiving pulses and spaces as short as 2 M-cycles (as tested using [sgb-speedtest]), following the common practice of 5 M-cycle pulses and 15 M-cycle spaces may improve reliability in some corner case that the community has not yet discovered.

:::tip

Obviously, it'd be no good idea to modify [the joypad register][JOYP] in the middle of a transfer.
For example, if your VBlank interrupt procedure normally reads out button states each frame, you should disable that behavior using a variable (or disable the interrupt handler entirely).

:::

:::warning

The GB program should wait 60 ms (4 frames) between each packet transfer and the next, as the "bomb" tool to erase a user-drawn border can cause the SGB system software not to check for packets for 4 frames.

:::

## Packet format

Each packet transfer is started by a "Start" pulse, then 16 bytes (128 bits) of data are transferred, the LSB of each byte first; and finally, a `0` bit must be transferred as a stop bit.
These 130 bit periods correspond to at least 2600 M-cycles at the recommended rate.

The structure of the first packet in a transmission is:

1. 1 pulse: Start signal
2. 1 byte: Header byte (<var>Command Code</var> × 8 + <var>Length</var>)
3. 15 bytes: <var>Data</var>
4. 1 bit: Stop Bit (`0`)

The above <var>Length</var> indicates the total number of packets (1-7, including the first packet) which will be sent.
If more than 15 data bytes are used, then further packet(s) will follow, as such:

1. 1 pulse: Start signal
2. 16 bytes: <var>Data</var>
3. 1 bit: Stop Bit (`0`)

By using all 7 packets, up to 111 data bytes (15 + 16 × 6) may be sent.

:::tip

Bytes with no indicated purpose are simply ignored by the SGB BIOS.
They can be set to any value (but they must still be transferred).

:::

[JOYP]: <#FF00 — P1/JOYP: Joypad>
[sgb-speedtest]: https://github.com/gb-archive/sgb-speedtest
