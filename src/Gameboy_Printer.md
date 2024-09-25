# Game Boy Printer

The Game Boy Printer is a portable thermal printer made by
[SII](https://en.wikipedia.org/wiki/Seiko_Instruments) for Nintendo, which a few games used to
print out bonus artwork, certificates, pictures ([Game Boy
Camera](<#Game Boy Camera>)).

It can use standard 38mm paper and interfaces with the Game Boy through
the Link port.

It is operated by an embedded 8-bit microcontroller which has its own
8 KiB of RAM to buffer incoming graphics data. Those 8 KiB allow a maximum
bitmap area of 160\*200 (8192/160\*4) pixels between prints.

## Communication

The Game Boy Printer doesn't use the full-duplex capability of the Link
port. It accepts variable length data packets and then answers back its
status after two $00 writes.

The packets all follow this format:

Content               | Size (bytes) | GB -> Printer | Printer -> GB |
----------------------|--------------|---------------|---------------|
Magic bytes           | 2            | $88, $33      | $00           |
Command               | 1            | See below     | $00           |
 Compression flag     | 1            | 0/1           | $00           |
Length of data        | 2            | LSB, MSB      | $00           |
Command-specific data | Variable     | See below     | $00           |
Checksum              | 2            | LSB, MSB      | $00           |
Alive indicator       | 1            |               | $00           |
Status                | 1            | See below     | $00           |

The checksum is simply a sum of every byte sent except the magic bytes
and obviously, the checksum itself.

## Detection

Send these 9 bytes: $88,$33,$0F,$00,$00,$00,$0F,$00 (Command
$0F, no data).

Send $00 and read input, if the byte is $81, then the printer is
there. Send a last $00, just for good measure. Input can be ignored.

## Commands

### Command 1: Initialize

This clears the printer's buffer RAM.

No data required. The normal status replied should be $00.

### Command 2: Start printing

Data length: 4 bytes

-   Byte 1: Number of sheets to print (0-255). 0 means line feed only.
-   Byte 2: Margins, high nibble is the feed before printing, low nibble
    is after printing. GB Camera sends $13 by default.
-   Byte 3: Palette, typically $E4 (%11100100)
-   Byte 4: 7 bits exposure value, sets the burning time for the print
    head. GB Camera sends $40 by default. Official manual mentions -25%
    darkness for $00 and +25% for $7F.

### Command 4: Fill buffer

Data length: max. $280 (160\*16 pixels in 2BPP) To transfer more than
$280 bytes, multiple \"command 4 packets\" have to be sent.

The graphics are organized in the normal tile format (16 bytes per
tile), and the tiles are sent in the same order they occur on your
tilemap (do keep in mind though that the printer does \*not\* have 32×32
tiles space for a map, but only 20×18).

An empty data packet must be sent before sending command 2 to print the
data, otherwise the print command will be ignored.

### Command $F: Read status

No data required, this is a \"nop\" command used only to read the Status
byte.

#### Status byte

A non-zero value for the higher nibble indicates something went wrong.

Bit \# | Name                | Description
-------|---------------------|---------------
   7   | Low Battery         | Set when the voltage is below threshold
   6   | Other error         |
   5   | Paper jam           | Set when the encoder gives no pulses when the motor is powered
   4   | Packet error        |
   3   | Unprocessed data    | Set when there's unprocessed data in memory - AKA ready to print
   2   | Image data full     |
   1   | Currently printing  | Set as long as the printer's burnin' paper
   0   | Checksum error      | Set when the calculated checksum doesn't match the received one

## Example

- Send command 1, the answer should be $81, $00
- Send command 4 with $280 of your graphics, the answer should still
  be $81, $00
- Ask for status with command $F, the answer should now be $81, $08
  (ready to print)
- Send an empty command 4 packet, the answer should still be $81,
  $08
- Send command 2 with your arguments (margins, palette, exposure), the
  answer should still be $81, $08
- Ask for status with command $F until it changes to $81, $06
  (printing !)
- Ask for status with command $F until it changes to $81, $04
  (printing done)
- Optionally send 16 zero bytes to clear the printer's receive buffer
  (GB Camera does it)

## Tips

- **The printer has a timeout of 100ms for packets. If no packet is
  received within that time, the printer will return to an initialized
  state (meaning the link and graphics buffers are reset).**
- **There appears to be an undocumented timeout for the bytes of a
  packet. It's best to send a packet completely or with very little
  delay between the individual bytes, otherwise the packet may not be
  accepted.**
- To print things larger than 20×18 (like GB Camera images with big
  borders), multiple data packets with a following print command need
  to be sent. The print command should be set to no linefeed (neither
  before nor after) to allow for continuous printing.

## Compression

Some sort of RLE? The GB Camera doesn't use it.

([Details and pictures](http://furrtek.free.fr/?a=gbprinter&i=2), need
to be copied here)
