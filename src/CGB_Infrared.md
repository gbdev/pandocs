# CGB Infrared

The GBC comes with an infrared port on the very top of the handheld,
which can be used to send and receive IR signals via its two separate
diodes. This functionality is not limited to other GBCs - it can be
used to communicate with external devices as well.

Note that the GBA does not include an infrared port and, as such, this
method of communication is unavailable in its CGB compatibility mode.

## FF56 - RP - Infrared Communications Port

This register allows data to be transmitted and received through the
infrared port.

Bit | Name             | Usage notes                     | R/W?
----|------------------|---------------------------------|-------------
6-7 | Data read enable | 0=Disable, 3=Enable             | Read/Write
 1  | Read data        | 0=Receiving IR Signal, 1=Normal | Read Only
 0  | Write data       | 0=LED Off, 1=Led On             | Read/Write

In order to read data, both Bit 6 and 7 must be set. Note that if Bit 0
is not cleared, the CGB will end up reading its own IR signal.

Both sending and receiving IR data increase power consumption - when not
used, RP should be reset to 00h.

## IR Receiver Adaptation

The IR sensor in the GBC adapts to the current level of infrared light
as a way to handle potential light pollution. In other words, if the
GBC receives a sustained IR signal beyond a certain amount of time,
eventually the sensor treats this as a new "normal" level of light,
which causes Bit 1 of RP to go back to 1. This is called the signal
"fade" because it may appear as if the signal disappears.

The following should be kept in mind while working with the infrared
port:

* IR communication protocols should be designed with this effect in mind.
  For example, a TV remote control may send a series of 32 LED ON/OFF
  pulses (length 10us ON, 17.5us OFF each) instead of a permanent 880us
  LED ON signal.
* When enabling data read in the RP register under brighter light
  conditions, one may initially read 0 from Bit 1, implying that an IR
  signal is being received. As such, a short amount of time should pass
  between writing 0xC0 to RP and attempting to read an IR signal. (TODO:
  How much? The upper bound is known to be 3-4 ms; the lower bound
  may be as little as 0.05-0.1 ms.)
