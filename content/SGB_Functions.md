# SGB Description

### General Description

Basically, the SGB (Super Gameboy) is an adapter cartridge that allows
to play Game Boy games on a SNES (Super Nintendo Entertainment System)
gaming console. In detail, you plug the Game Boy cartridge into the SGB
cartridge, then plug the SGB cartridge into the SNES, and then connect
the SNES to your TV Set. In result, games can be played and viewed on
the TV Set, and are controlled by using the SNES joypad(s).

### More Technical Description

The SGB cartridge just contains a normal Game Boy CPU and normal gameboy
video controller. Normally the video signal from this controller would
be sent to the LCD screen, however, in this special case the SNES read
out the video signal and displays it on the TV set by using a special
SNES BIOS ROM which is located in the SGB cartridge. Also, normal
gameboy sound output is forwared to the SNES and output to the TV Set,
vice versa, joypad input is forwared from the SNES controller(s) to the
gameboy joypad inputs.

### Normal Monochrome Games

Any Game Boy games which have been designed for normal monochrome
handheld gameboys will work with the SGB hardware as well. The SGB will
apply a four color palette to these games by replacing the normal four
grayshades. The 160x144 pixel gamescreen is displayed in the middle of
the 256x224 pixel SNES screen (the unused area is filled by a screen
border bitmap). The user may access built-in menues, allowing to change
color palette data, to select between several pre-defined borders, etc.

Games that have been designed to support SGB functions may also access
the following additional features:

### Colorized Game Screen

There's limited ability to colorize the gamescreen by assigning custom
color palettes to each 20x18 display characters, however, this works
mainly for static display data such like title screens or status bars,
the 20x18 color attribute map is non-scrollable, and it is not possible
to assign separate colors to moveable foreground sprites (OBJs), so that
animated screen regions will be typically restricted to using a single
palette of four colors only.

### SNES Foreground Sprites

Up to 24 foreground sprites (OBJs) of 8x8 or 16x16 pixels, 16 colors can
be displayed. When replacing (or just overlaying) the normal gameboy
OBJs by SNES OBJs it'd be thus possible to display OBJs with other
colors than normal background area. This method doesn't appear to be
very popular, even though it appears to be quite easy to implement,
however, the bottommost character line of the gamescreen will be masked
out because this area is used to transfer OAM data to the SNES.

### The SGB Border

The possibly most popular and most impressive feature is to replace the
default SGB screen border by a custom bitmap which is stored in the game
cartridge.

### Multiple Joypads

Up to four joypads can be conected to the SNES, and SGB software may
read-out each of these joypads separately, allowing up to four players
to play the same game simultaneously. Unlike for multiplayer handheld
games, this requires only one game cartridge and only one SGB/SNES, and
no link cables are required, the downside is that all players must share
the same display screen.

### Sound Functions

Beside for normal Game Boy sound, a number of digital sound effects is
pre-defined in the SNES BIOS, these effects may be accessed quite
easily. Programmers whom are familiar with SNES sounds may also access
the SNES sound chip, or use the SNES MIDI engine directly in order to
produce other sound effects or music.

### Taking Control of the SNES CPU

Finally, it is possible to write program code or data into SNES memory,
and to execute such program code by using the SNES CPU.

### SGB System Clock

Because the SGB is synchronized to the SNES CPU, the Game Boy system
clock is directly chained to the SNES system clock. In result, the
gameboy CPU, video controller, timers, and sound frequencies will be all
operated approx 2.4% faster as by normal gameboys. Basically, this
should be no problem, and the game will just run a little bit faster.
However sensitive musicians may notice that sound frequencies are a bit
too high, programs that support SGB functions may avoid this effect by
reducing frequencies of Game Boy sounds when having detected SGB
hardware. Also, I think that I've heard that SNES models which use a
50Hz display refresh rate (rather than 60Hz) are resulting in
respectively slower SGB/gameboy timings ???

# SGB Unlocking and Detecting SGB Functions

### Cartridge Header

SGB games are required to have a cartridge header with Nintendo and
proper checksum just as normal Game Boy games. Also, two special entries
must be set in order to unlock SGB functions:

` 146h - SGB Flag - Must be set to 03h for SGB games`<br>
` 14Bh - Old Licensee Code - Must be set 33h for SGB games`<br>

When these entries aren't set, the game will still work just like all
'monochrome' Game Boy games, but it cannot access any of the special
SGB functions.

### Detecting SGB hardware

The recommended detection method is to send a MLT_REQ command which
enables two (or four) joypads. A normal handheld Game Boy will ignore
this command, a SGB will now return incrementing joypad IDs each time
when deselecting keyboard lines (see MLT_REQ description for details).
Now read-out joypad state/IDs several times, and if the ID-numbers are
changing, then it is a SGB (a normal Game Boy would typically always
return 0Fh as ID). Finally, when not intending to use more than one
joypad, send another MLT_REQ command in order to re-disable the
multi-controller mode. Detection works regardless of whether and how
many joypads are physically connected to the SNES. However, detection
works only when having unlocked SGB functions in the cartridge header,
as described above.

### Separating between SGB and SGB2

It is also possible to separate between SGB and SGB2 models by examining
the inital value of the accumulator (A-register) directly after startup.

` 01h  SGB or Normal Gameboy (DMG)`<br>
` FFh  SGB2 or Pocket Gameboy`<br>
` 11h  CGB or GBA`<br>

Because values 01h and FFh are shared for both handhelds and SGBs, it is
still required to use the above MLT_REQ detection procedure. As far as
I know the SGB2 doesn't have any extra features which'd require
separate SGB2 detection except for curiosity purposes, for example, the
game "Tetris DX" chooses to display an alternate SGB border on SGB2s.

Reportedly, some SGB models include link ports (just like handheld
gameboy) (my own SGB does not have such an port), possibly this feature
is available in SGB2-type models only ???

# SGB Command Packet Transfers

Command packets (aka Register Files) are transferred from the Game Boy to
the SNES by using P14 and P15 output lines of the JOYPAD register
(FF00h), these lines are normally used to select the two rows in the
gameboy keyboard matrix (which still works).

### Transferring Bits

A command packet transfer must be initiated by setting both P14 and P15
to LOW, this will reset and start the SNES packet receiving program.
Data is then transferred (LSB first), setting P14=LOW will indicate a
"0" bit, and setting P15=LOW will indicate a "1" bit. For example:

`      RESET 0   0   1   1   0   1   0`<br>
` P14  --_---_---_-----------_-------_--...`<br>
` P15  --_-----------_---_-------_------...`<br>

Data and reset pulses must be kept LOW for at least 5us. P14 and P15
must be kept both HIGH for at least 15us between any pulses. Obviously,
it'd be no good idea to access the JOYPAD register during the transfer,
for example, in case that your VBlank interrupt procedure reads-out
joypad states each frame, be sure to disable that interrupt during the
transfer (or disable only the joypad procedure by using a software
flag).

### Transferring Packets

Each packet is invoked by a RESET pulse, then 128 bits of data are
transferred (16 bytes, LSB of first byte first), and finally, a
"0"-bit must be transferred as stop bit. The structure of normal
packets is:

`  1 PULSE Reset`<br>
`  1 BYTE  Command Code*8+Length`<br>
` 15 BYTES Parameter Data`<br>
`  1 BIT   Stop Bit (0)`<br>

The above 'Length' indicates the total number of packets (1-7,
including the first packet) which will be sent, ie. if more than 15
parameter bytes are used, then further packet(s) will follow, as such:

`  1 PULSE Reset`<br>
` 16 BYTES Parameter Data`<br>
`  1 BIT   Stop Bit (0)`<br>

By using all 7 packets, up to 111 data bytes (15+16\*6) may be sent.
Unused bytes at the end of the last packet don't matter. A 60ms (4
frames) delay should be invoked between each packet transfer.

# SGB VRAM Transfers

### Overview

Beside for the packet transfer method, larger data blocks of 4KBytes can
be transferred by using the video signal. These transfers are invoked by
first sending one of the commands with the ending \_TRN (by using normal
packet transfer), the 4K data block is then read-out by the SNES from
gameboy display memory during the next frame.

### Transfer Data

Normally, transfer data should be stored at 8000h-8FFFh in Game Boy VRAM,
even though the SNES receives the data in from display scanlines, it
will automatically re-produce the same ordering of bits and bytes, as
being originally stored at 8000h-8FFFh in Game Boy memory.

### Preparing the Display

The above method works only when recursing the following things: BG Map
must display unsigned characters 00h-FFh on the screen; 00h..13h in
first line, 14h..27h in next line, etc. The Game Boy display must be
enabled, the display may not be scrolled, OBJ sprites should not overlap
the background tiles, the BGP palette register must be set to E4h.

### Transfer Time

Note that the transfer data should be prepared in VRAM **before** sending
the transfer command packet. The actual transfer starts at the beginning
of the next frame after the command has been sent, and the transfer ends
at the end of the 5th frame after the command has been sent (not
counting the frame in which the command has been sent). The displayed
data must not be modified during the transfer, as the SGB reads it in
multiple chunks.

### Avoiding Screen Garbage

The display will contain 'garbage' during the transfer, this
dirt-effect can be avoided by freezing the screen (in the state which
has been displayed before the transfer) by using the MASK_EN command.
Of course, this works only when actually executing the game on a SGB
(and not on normal handheld gameboys), it'd be thus required to detect
the presence of SGB hardware before blindly sending VRAM data.

# SGB Command Summary

### SGB System Command Table

` Code Name      Expl.`<br>
` 00   PAL01     Set SGB Palette 0,1 Data`<br>
` 01   PAL23     Set SGB Palette 2,3 Data`<br>
` 02   PAL03     Set SGB Palette 0,3 Data`<br>
` 03   PAL12     Set SGB Palette 1,2 Data`<br>
` 04   ATTR_BLK  "Block" Area Designation Mode`<br>
` 05   ATTR_LIN  "Line" Area Designation Mode`<br>
` 06   ATTR_DIV  "Divide" Area Designation Mode`<br>
` 07   ATTR_CHR  "1CHR" Area Designation Mode`<br>
` 08   SOUND     Sound On/Off`<br>
` 09   SOU_TRN   Transfer Sound PRG/DATA`<br>
` 0A   PAL_SET   Set SGB Palette Indirect`<br>
` 0B   PAL_TRN   Set System Color Palette Data`<br>
` 0C   ATRC_EN   Enable/disable Attraction Mode`<br>
` 0D   TEST_EN   Speed Function`<br>
` 0E   ICON_EN   SGB Function`<br>
` 0F   DATA_SND  SUPER NES WRAM Transfer 1`<br>
` 10   DATA_TRN  SUPER NES WRAM Transfer 2`<br>
` 11   MLT_REG   Controller 2 Request`<br>
` 12   JUMP      Set SNES Program Counter`<br>
` 13   CHR_TRN   Transfer Character Font Data`<br>
` 14   PCT_TRN   Set Screen Data Color Data`<br>
` 15   ATTR_TRN  Set Attribute from ATF`<br>
` 16   ATTR_SET  Set Data to ATF`<br>
` 17   MASK_EN   Game Boy Window Mask`<br>
` 18   OBJ_TRN   Super NES OBJ Mode`<br>

# SGB Color Palettes Overview

### Available SNES Palettes

The SGB/SNES provides 8 palettes of 16 colors each, each color may be
defined out of a selection of 34768 colors (15 bit). Palettes 0-3 are
used to colorize the gamescreen, only the first four colors of each of
these palettes are used. Palettes 4-7 are used for the SGB Border, all
16 colors of each of these palettes may be used.

### Color format

Colors are encoded as 16-bit RGB numbers, in the following way:

` FEDC BA98 7654 3210`<br>
` 0BBB BBGG GGGR RRRR`<br>

Here's a formula to convert 24-bit RGB into SNES format:
`(color & 0xF8) << 7 | (color & 0xF800) >> 6 | (color & 0xF80000) >> 19`<br>

The palettes are encoded **little-endian**, thus, the R/G byte comes
first in memory.

### Color 0 Restriction

Color 0 of each of the eight palettes is transparent, causing the
backdrop color to be displayed instead. The backdrop color is typically
defined by the most recently color being assigned to Color 0 (regardless
of the palette number being used for that operation). Effectively,
gamescreen palettes can have only three custom colors each, and SGB
border palettes only 15 colors each, additionally, color 0 can be used
for for all palettes, which will then all share the same color though.

### Translation of Grayshades into Colors

Because the SGB/SNES reads out the Game Boy video controllers display
signal, it translates the different grayshades from the signal into SNES
colors as such:

` White       -->  Color 0`<br>
` Light Gray  -->  Color 1`<br>
` Dark Gray   -->  Color 2`<br>
` Black       -->  Color 3`<br>

Note that Game Boy colors 0-3 are assigned to user-selectable grayshades
by the gameboys BGP, OBP1, and OBP2 registers. There is thus no fixed
relationship between Game Boy colors 0-3 and SNES colors 0-3.

#### Using Game Boy BGP/OBP Registers

A direct translation of GB color 0-3 into SNES color 0-3 may be produced
by setting BGP/OBP registers to a value of 0E4h each. However, in case
that your program uses black background for example, then you may
internally assign background as "White" at the Game Boy side by BGP/OBP
registers (which is then interpreted as SNES color 0, which is shared
for all SNES palettes). The advantage is that you may define Color 0 as
Black at the SNES side, and may assign custom colors for Colors 1-3 of
each SNES palette.

### System Color Palette Memory

Beside for the actually visible palettes, up to 512 palettes of 4 colors
each may be defined in SNES RAM. The palettes are just stored in RAM
without any relationship to the displayed picture; however, these
pre-defined colors may be transferred to actually visible palettes
slightly faster than when transferring palette data by separate command
packets.

# SGB Palette Commands

### SGB Command 00h - PAL01

Transmit color data for SGB palette 0, color 0-3, and for SGB palette 1,
color 1-3 (without separate color 0).

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=01h)`<br>
` 1-E   Color Data for 7 colors of 2 bytes (16bit) each:`<br>
`         Bit 0-4   - Red Intensity   (0-31)`<br>
`         Bit 5-9   - Green Intensity (0-31)`<br>
`         Bit 10-14 - Blue Intensity  (0-31)`<br>
`         Bit 15    - Not used (zero)`<br>
` F     Not used (00h)`<br>

This is the same RGB5 format as [Game Boy Color palette
entry](#lcd-color-palettes-cgb-only), though
without the LCD correction. The value transferred as color 0 will be
applied for all four palettes.

### SGB Command 01h - PAL23

Same as above PAL01, but for Palettes 2 and 3 respectively.

### SGB Command 02h - PAL03

Same as above PAL01, but for Palettes 0 and 3 respectively.

### SGB Command 03h - PAL12

Same as above PAL01, but for Palettes 1 and 2 respectively.

### SGB Command 0Ah - PAL_SET

Used to copy pre-defined palette data from SGB system color palettes to
actual SNES palettes.

Note: all palette numbers are little-endian.

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1-2   System Palette number for SGB Color Palette 0 (0-511)`<br>
` 3-4   System Palette number for SGB Color Palette 1 (0-511)`<br>
` 5-6   System Palette number for SGB Color Palette 2 (0-511)`<br>
` 7-8   System Palette number for SGB Color Palette 3 (0-511)`<br>
` 9     Attribute File`<br>
`         Bit 0-5 - Attribute File Number (00h-2Ch) (Used only if Bit7=1)`<br>
`         Bit 6   - Cancel Mask           (0=No change, 1=Yes)`<br>
`         Bit 7   - Use Attribute File    (0=No, 1=Apply above ATF Number)`<br>
` A-F   Not used (zero)`<br>

Before using this function, System Palette data should be initialized by
PAL_TRN command, and (when used) Attribute File data should be
initialized by ATTR_TRN.

### SGB Command 0Bh - PAL_TRN

Used to initialize SGB system color palettes in SNES RAM. System color
palette memory contains 512 pre-defined palettes, these palettes do not
directly affect the display, however, the PAL_SET command may be later
used to transfer four of these 'logical' palettes to actual visible
'physical' SGB palettes. Also, the OBJ_TRN function will use groups
of 4 System Color Palettes (4\*4 colors) for SNES OBJ palettes (16
colors).

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1-F   Not used (zero)`<br>

The palette data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Data for System Color Palette 0-511`<br>

Each Palette consists of four 16bit-color definitions (8 bytes). Note:
The data is stored at 3000h-3FFFh in SNES memory.

# SGB Color Attribute Commands

### SGB Command 04h - ATTR_BLK

Used to specify color attributes for the inside or outside of one or
more rectangular screen regions.

` Byte  Content`<br>
` 0     Command*8+Length (length=1..7)`<br>
` 1     Number of Data Sets (01h..12h)`<br>
` 2-7   Data Set #1`<br>
`         Byte 0 - Control Code (0-7)`<br>
`           Bit 0 - Change Colors inside of surrounded area     (1=Yes)`<br>
`           Bit 1 - Change Colors of surrounding character line (1=Yes)`<br>
`           Bit 2 - Change Colors outside of surrounded area    (1=Yes)`<br>
`           Bit 3-7 - Not used (zero)`<br>
`           Exception: When changing only the Inside or Outside, then the`<br>
`           Surrounding line becomes automatically changed to same color.`<br>
`         Byte 1 - Color Palette Designation`<br>
`           Bit 0-1 - Palette Number for inside of surrounded area`<br>
`           Bit 2-3 - Palette Number for surrounding character line`<br>
`           Bit 4-5 - Palette Number for outside of surrounded area`<br>
`           Bit 6-7 - Not used (zero)`<br>
`         Data Set Byte 2 - Coordinate X1 (left)`<br>
`         Data Set Byte 3 - Coordinate Y1 (upper)`<br>
`         Data Set Byte 4 - Coordinate X2 (right)`<br>
`         Data Set Byte 5 - Coordinate Y2 (lower)`<br>
`           Specifies the coordinates of the surrounding rectangle.`<br>
` 8-D   Data Set #2 (if any)`<br>
` E-F   Data Set #3 (continued at 0-3 in next packet) (if any)`<br>

When sending three or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets is described below.

### SGB Command 05h - ATTR_LIN

Used to specify color attributes of one or more horizontal or vertical
character lines.

` Byte  Content`<br>
` 0     Command*8+Length (length=1..7)`<br>
` 1     Number of Data Sets (01h..6Eh) (one byte each)`<br>
` 2     Data Set #1`<br>
`         Bit 0-4 - Line Number    (X- or Y-coordinate, depending on bit 7)`<br>
`         Bit 5-6 - Palette Number (0-3)`<br>
`         Bit 7   - H/V Mode Bit   (0=Vertical line, 1=Horizontal Line)`<br>
` 3     Data Set #2 (if any)`<br>
` 4     Data Set #3 (if any)`<br>
` etc.`<br>

When sending 15 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets (one byte each) is described
below. The length of each line reaches from one end of the screen to the
other end. In case that some lines overlap each other, then lines from
lastmost data sets will overwrite lines from previous data sets.

### SGB Command 06h - ATTR_DIV

Used to split the screen into two halfes, and to assign separate color
attributes to each half, and to the division line between them.

` Byte  Content`<br>
` 0     Command*8+Length   (fixed length=1)`<br>
` 1     Color Palette Numbers and H/V Mode Bit`<br>
`         Bit 0-1  Palette Number below/right of division line`<br>
`         Bit 2-3  Palette Number above/left of division line`<br>
`         Bit 4-5  Palette Number for division line`<br>
`         Bit 6    H/V Mode Bit  (0=split left/right, 1=split above/below)`<br>
` 2     X- or Y-Coordinate (depending on H/V bit)`<br>
` 3-F   Not used (zero)`<br>

### SGB Command 07h - ATTR_CHR

Used to specify color attributes for separate characters.

` Byte  Content`<br>
` 0     Command*8+Length (length=1..6)`<br>
` 1     Beginning X-Coordinate`<br>
` 2     Beginning Y-Coordinate`<br>
` 3-4   Number of Data Sets (1-360)`<br>
` 5     Writing Style   (0=Left to Right, 1=Top to Bottom)`<br>
` 6     Data Sets 1-4   (Set 1 in MSBs, Set 4 in LSBs)`<br>
` 7     Data Sets 5-8   (if any)`<br>
` 8     Data Sets 9-12  (if any)`<br>
` etc.`<br>

When sending 41 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. Each data set consists of two bits, indicating the palette number
for one character. Depending on the writing style, data sets are written
from left to right, or from top to bottom. In either case the function
wraps to the next row/column when reaching the end of the screen.

### SGB Command 15h - ATTR_TRN

Used to initialize Attribute Files (ATFs) in SNES RAM. Each ATF consists
of 20x18 color attributes for the Game Boy screen. This function does not
directly affect display attributes. Instead, one of the defined ATFs may
be copied to actual display memory at a later time by using ATTR_SET or
PAL_SET functions.

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1-F   Not used (zero)`<br>

The ATF data is sent by VRAM-Transfer (4 KBytes).

` 000-FD1  Data for ATF0 through ATF44 (4050 bytes)`<br>
` FD2-FFF  Not used`<br>

Each ATF consists of 90 bytes, that are 5 bytes (20x2bits) for each of
the 18 character lines of the Game Boy window. The two most significant
bits of the first byte define the color attribute (0-3) for the first
character of the first line, the next two bits the next character, and
so on.

### SGB Command 16h - ATTR_SET

Used to transfer attributes from Attribute File (ATF) to Game Boy window.

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1     Attribute File Number (00-2Ch), Bit 6=Cancel Mask`<br>
` 2-F   Not used (zero)`<br>

When above Bit 6 is set, the Game Boy screen becomes re-enabled after the
transfer (in case it has been disabled/frozen by MASK_EN command).
Note: The same functions may be (optionally) also included in PAL_SET
commands, as described in the chapter about Color Palette Commands.

# SGB Sound Functions

### SGB Command 08h - SOUND

Used to start/stop internal sound effect, start/stop sound using
internal tone data.

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1     Sound Effect A (Port 1) Decrescendo 8bit Sound Code`<br>
` 2     Sound Effect B (Port 2) Sustain     8bit Sound Code`<br>
` 3     Sound Effect Attributes`<br>
`         Bit 0-1 - Sound Effect A Pitch  (0..3=Low..High)`<br>
`         Bit 2-3 - Sound Effect A Volume (0..2=High..Low, 3=Mute on)`<br>
`         Bit 4-5 - Sound Effect B Pitch  (0..3=Low..High)`<br>
`         Bit 6-7 - Sound Effect B Volume (0..2=High..Low, 3=Not used)`<br>
` 4     Music Score Code (must be zero if not used)`<br>
` 5-F   Not used (zero)`<br>

See Sound Effect Tables below for a list of available pre-defined
effects.

Notes:

1.  Mute is only active when both bits D2 and D3 are 1.
2.  When the volume is set for either Sound Effect A or Sound Effect B,
    mute is turned off.
3.  When Mute on/off has been executed, the sound fades out/fades in.
4.  Mute on/off operates on the (BGM) which is reproduced by Sound
    Effect A, Sound Effect B, and the Super NES APU. A "mute off" flag
    does not exist by itself. When mute flag is set, volume and pitch of
    Sound Effect A (port 1) and Sound Effect B (port 2) must be set.

### SGB Command 09h - SOU_TRN

Used to transfer sound code or data to SNES Audio Processing Unit memory
(APU-RAM).

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1-F   Not used (zero)`<br>

The sound code/data is sent by VRAM-Transfer (4 KBytes).

` 000      One (or two ???) 16bit expression(s ???) indicating the`<br>
`          transfer destination address and transfer length.`<br>
` ...-...  Transfer Data`<br>
` ...-FFF  Remaining bytes not used`<br>

Possible destinations in APU-RAM are:

` 0400h-2AFFh  APU-RAM Program Area (9.75KBytes)`<br>
` 2B00h-4AFFh  APU-RAM Sound Score Area (8Kbytes)`<br>
` 4DB0h-EEFFh  APU-RAM Sampling Data Area (40.25 Kbytes)`<br>

This function may be used to take control of the SNES sound chip, and/or
to access the SNES MIDI engine. In either case it requires deeper
knowledge of SNES sound programming.

### SGB Sound Effect A/B Tables

Below lists the digital sound effects that are pre-defined in the
SGB/SNES BIOS, and which can be used with the SGB "SOUND" Command.
Effect A and B may be simultaneously reproduced. The P-column indicates
the recommended Pitch value, the V-column indicates the numbers of
Voices used. Sound Effect A uses voices 6,7. Sound Effect B uses voices
0,1,4,5. Effects that use less voices will use only the upper voices
(eg. 4,5 for Effect B with only two voices).

### Sound Effect A Flag Table

` Code Description             P V     Code Description             P V`<br>
` 00  Dummy flag, re-trigger   - 2     18  Fast Jump                3 1`<br>
` 80  Effect A, stop/silent    - 2     19  Jet (rocket) takeoff     0 1`<br>
` 01  Nintendo                 3 1     1A  Jet (rocket) landing     0 1`<br>
` 02  Game Over                3 2     1B  Cup breaking             2 2`<br>
` 03  Drop                     3 1     1C  Glass breaking           1 2`<br>
` 04  OK ... A                 3 2     1D  Level UP                 2 2`<br>
` 05  OK ... B                 3 2     1E  Insert air               1 1`<br>
` 06  Select...A               3 2     1F  Sword swing              1 1`<br>
` 07  Select...B               3 1     20  Water falling            2 1`<br>
` 08  Select...C               2 2     21  Fire                     1 1`<br>
` 09  Mistake...Buzzer         2 1     22  Wall collapsing          1 2`<br>
` 0A  Catch Item               2 2     23  Cancel                   1 2`<br>
` 0B  Gate squeaks 1 time      2 2     24  Walking                  1 2`<br>
` 0C  Explosion...small        1 2     25  Blocking strike          1 2`<br>
` 0D  Explosion...medium       1 2     26  Picture floats on & off  3 2`<br>
` 0E  Explosion...large        1 2     27  Fade in                  0 2`<br>
` 0F  Attacked...A             3 1     28  Fade out                 0 2`<br>
` 10  Attacked...B             3 2     29  Window being opened      1 2`<br>
` 11  Hit (punch)...A          0 2     2A  Window being closed      0 2`<br>
` 12  Hit (punch)...B          0 2     2B  Big Laser                3 2`<br>
` 13  Breath in air            3 2     2C  Stone gate closes/opens  0 2`<br>
` 14  Rocket Projectile...A    3 2     2D  Teleportation            3 1`<br>
` 15  Rocket Projectile...B    3 2     2E  Lightning                0 2`<br>
` 16  Escaping Bubble          2 1     2F  Earthquake               0 2`<br>
` 17  Jump                     3 1     30  Small Laser              2 2`<br>

Sound effect A is used for formanto sounds (percussion sounds).

### Sound Effect B Flag Table

` Code Description             P V     Code Description             P V`<br>
` 00  Dummy flag, re-trigger   - 4     0D  Waterfall                2 2`<br>
` 80  Effect B, stop/silent    - 4     0E  Small character running  3 1`<br>
` 01  Applause...small group   2 1     0F  Horse running            3 1`<br>
` 02  Applause...medium group  2 2     10  Warning sound            1 1`<br>
` 03  Applause...large group   2 4     11  Approaching car          0 1`<br>
` 04  Wind                     1 2     12  Jet flying               1 1`<br>
` 05  Rain                     1 1     13  UFO flying               2 1`<br>
` 06  Storm                    1 3     14  Electromagnetic waves    0 1`<br>
` 07  Storm with wind/thunder  2 4     15  Score UP                 3 1`<br>
` 08  Lightning                0 2     16  Fire                     2 1`<br>
` 09  Earthquake               0 2     17  Camera shutter, formanto 3 4`<br>
` 0A  Avalanche                0 2     18  Write, formanto          0 1`<br>
` 0B  Wave                     0 1     19  Show up title, formanto  0 1`<br>
` 0C  River                    3 2`<br>

Sound effect B is mainly used for looping sounds (sustained sounds).

# SGB System Control Commands

### SGB Command 17h - MASK_EN

Used to mask the Game Boy window, among others this can be used to freeze
the Game Boy screen before transferring data through VRAM (the SNES then
keeps displaying the Game Boy screen, even though VRAM doesn't contain
meaningful display information during the transfer).

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1     Gameboy Screen Mask (0-3)`<br>
`         0  Cancel Mask   (Display activated)`<br>
`         1  Freeze Screen (Keep displaying current picture)`<br>
`         2  Blank Screen  (Black)`<br>
`         3  Blank Screen  (Color 0)`<br>
` 2-F   Not used (zero)`<br>

Freezing works only if the SNES has stored a picture, ie. if necessary
wait one or two frames before freezing (rather than freezing directly
after having displayed the picture). The Cancel Mask function may be
also invoked (optionally) by completion of PAL_SET and ATTR_SET
commands.

### SGB Command 0Ch - ATRC_EN

Used to enable/disable Attraction mode, which is enabled by default.

Built-in borders other than the Game Boy frame and the plain black
border have a "screen saver" activated by pressing R, L, L, L, L, R or
by leaving the controller alone for roughly 7 minutes (tested with 144p
Test Suite). It is speculated that the animation may have interfered
with rarely-used SGB features, such as OBJ_TRN or JUMP, and that
Attraction Disable disables this animation.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     Attraction Disable  (0=Enable, 1=Disable)`<br>
` 2-F   Not used (zero)`<br>

### SGB Command 0Dh - TEST_EN

Used to enable/disable test mode for "SGB-CPU variable clock speed
function". This function is disabled by default.

This command does nothing on some SGB revisions. (SGBv2 confirmed,
unknown on others)

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     Test Mode Enable    (0=Disable, 1=Enable)`<br>
` 2-F   Not used (zero)`<br>

Maybe intended to determine whether SNES operates at 50Hz or 60Hz
display refresh rate ??? Possibly result can be read-out from joypad
register ???

### SGB Command 0Eh - ICON_EN

Used to enable/disable ICON function. Possibly meant to enable/disable
SGB/SNES popup menues which might otherwise activated during gameboy
game play. By default all functions are enabled (0).

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     Disable Bits`<br>
`         Bit 0 - Use of SGB-Built-in Color Palettes    (1=Disable)`<br>
`         Bit 1 - Controller Set-up Screen    (0=Enable, 1=Disable)`<br>
`         Bit 2 - SGB Register File Transfer (0=Receive, 1=Disable)`<br>
`         Bit 3-6 - Not used (zero)`<br>
` 2-F   Not used (zero)`<br>

Above Bit 2 will suppress all further packets/commands when set, this
might be useful when starting a monochrome game from inside of the
SGB-menu of a multi-gamepak which contains a collection of different
games.

### SGB Command 0Fh - DATA_SND

Used to write one or more bytes directly into SNES Work RAM.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     SNES Destination Address, low`<br>
` 2     SNES Destination Address, high`<br>
` 3     SNES Destination Address, bank number`<br>
` 4     Number of bytes to write (01h-0Bh)`<br>
` 5     Data Byte #1`<br>
` 6     Data Byte #2 (if any)`<br>
` 7     Data Byte #3 (if any)`<br>
` etc.`<br>

Unused bytes at the end of the packet should be set to zero, this
function is restricted to a single packet, so that not more than 11
bytes can be defined at once. Free Addresses in SNES memory are Bank 0
1800h-1FFFh, Bank 7Fh 0000h-FFFFh.

### SGB Command 10h - DATA_TRN

Used to transfer binary code or data directly into SNES RAM.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     SNES Destination Address, low`<br>
` 2     SNES Destination Address, high`<br>
` 3     SNES Destination Address, bank number`<br>
` 4-F   Not used (zero)`<br>

The data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Data`<br>

Free Addresses in SNES memory are Bank 0 1800h-1FFFh, Bank 7Fh
0000h-FFFFh. The transfer length is fixed at 4KBytes ???, so that
directly writing to the free 2KBytes at 0:1800h would be a not so good
idea ???

### SGB Command 12h - JUMP

Used to set the SNES program counter and NMI (vblank interrupt) handler
to specific addresses.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     SNES Program Counter, low`<br>
` 2     SNES Program Counter, high`<br>
` 3     SNES Program Counter, bank number`<br>
` 4     SNES NMI Handler, low`<br>
` 5     SNES NMI Handler, high`<br>
` 6     SNES NMI Handler, bank number`<br>
` 7-F   Not used, zero`<br>

The game *Space Invaders* uses this function when selecting "Arcade
mode" to execute SNES program code which has been previously
transferred from the SGB to the SNES. The SNES CPU is a Ricoh 5A22,
which combines a 65C816 core licensed from WDC with a custom memory
controller. For more information, see ["fullsnes" by
nocash](https://problemkaputt.de/fullsnes.htm).

Some notes for intrepid Super NES programmers seeking to use a flash
cartridge in a Super Game Boy as a storage server:

-   JUMP overwrites the NMI handler even if it is $000000.
-   The SGB system software does not appear to use NMIs.
-   JUMP can return to SGB system software via a 16-bit RTS. To do this,
    JML to a location in bank $00 containing byte value $60, such as
    any of the [stubbed commands](#stubbed-commands).
-   IRQs and COP and BRK instructions are not useful because their
    handlers still point into SGB ROM. Use SEI WAI.
-   If a program called through JUMP does not intend to return to SGB
    system software, it can overwrite all Super NES RAM except $0000BB
    through $0000BD, the NMI vector.
-   To enter APU boot ROM, write $FE to $2140. Echo will still be on
    though.

# SGB Multiplayer Command

### SGB Command 11h - MLT_REQ

Used to request multiplayer mode (ie. input from more than one joypad).
Because this function provides feedback from the SGB/SNES to the Game
Boy program, it is also used to detect SGB hardware.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     Multiplayer Control (0-3) (Bit0=Enable, Bit1=Two/Four Players)`<br>
`         0 = One player`<br>
`         1 = Two players`<br>
`         3 = Four players`<br>
` 2-F   Not used (zero)`<br>

In one player mode, the second joypad (if any) is used for the SGB
system program. In two player mode, both joypads are used for the game.
Because SNES have only two joypad sockets, four player mode requires an
external "Multiplayer 5" adapter.

Changing the number of active players ANDs the currently selected player
minus one with the number of players in that mode minus one. For example
if you go from four players to two players and player 4 was active
player 2 will then be active because 3 AND 1 is 1. However, sending the
MLT_REQ command will increment the counter several times so results may
not be exactly as expected. The most frequent case is going from one
player to two-or-four player which will always start with player 1
active.

### Reading Multiple Controllers (Joypads)

When having enabled multiple controllers by MLT_REQ, data for each
joypad can be read out through JOYPAD register (FF00) as follows: First
set P14 and P15 both HIGH (deselect both Buttons and Cursor keys), you
can now read the lower 4bits of FF00 which indicate the joypad ID for
the following joypad input:

` 0Fh  Joypad 1`<br>
` 0Eh  Joypad 2`<br>
` 0Dh  Joypad 3`<br>
` 0Ch  Joypad 4`<br>

Next, read joypad state as normally. When completed, set P14 and P15
back HIGH, this automatically increments the joypad number (or restarts
counting once reached the lastmost joypad). Repeat the procedure until
you have read-out states for all two (or four) joypads.

If for whatever reason you want to increment the joypad number without
reading the joypad state you only need to set P15 to LOW before setting
it back to HIGH. Adjusting P14 does not affect whether or not the joypad
number will advance, However, if you set P15 to LOW then HIGH then LOW
again without bringing both P14 and P15 HIGH at any point, it cancels
the increment until P15 is lowered again. There are games, such as
Pokémon Yellow, which rely on this cancelling when detecting the SGB.

# SGB Border and OBJ Commands

### SGB Command 13h - CHR_TRN

Used to transfer tile data (characters) to SNES Tile memory in VRAM.
This normally used to define BG tiles for the SGB Border (see PCT_TRN),
but might be also used to define moveable SNES foreground sprites (see
OBJ_TRN).

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1     Tile Transfer Destination`<br>
`         Bit 0   - Tile Numbers   (0=Tiles 00h-7Fh, 1=Tiles 80h-FFh)`<br>
`         Bit 1   - Tile Type      (0=BG Tiles, 1=OBJ Tiles)`<br>
`         Bit 2-7 - Not used (zero)`<br>
` 2-F   Not used (zero)`<br>

The tile data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Bitmap data for 128 Tiles`<br>

Each tile occupies 32 bytes (8x8 pixels, 16 colors each). When intending
to transfer more than 128 tiles, call this function twice (once for
tiles 00h-7Fh, and once for tiles 80h-FFh). Note: The BG/OBJ Bit seems
to have no effect and writes to the same VRAM addresses for both BG and
OBJ ???

TODO: explain tile format

### SGB Command 14h - PCT_TRN

Used to transfer tile map data and palette data to SNES BG Map memory in
VRAM to be used for the SGB border. The actual tiles must be separately
transferred by using the CHR_TRN function.

` Byte  Content`<br>
` 0     Command*8+Length    (fixed length=1)`<br>
` 1-F   Not used (zero)`<br>

The map data is sent by VRAM-Transfer (4 KBytes).

` 000-6FF  BG Map 32x28 Entries of 16bit each (1792 bytes)`<br>
` 700-7FF  Not used, don't care`<br>
` 800-87F  BG Palette Data (Palettes 4-7, each 16 colors of 16bits each)`<br>
` 880-FFF  Not used, don't care`<br>

Each BG Map Entry consists of a 16bit value as such:
`VH01 PP00 NNNN NNNN`<br>

` Bit 0-9   - Character Number (use only 00h-FFh, upper 2 bits zero)`<br>
` Bit 10-12 - Palette Number   (use only 4-7, officially use only 4-6)`<br>
` Bit 13    - BG Priority      (use only 0)`<br>
` Bit 14    - X-Flip           (0=Normal, 1=Mirror horizontally)`<br>
` Bit 15    - Y-Flip           (0=Normal, 1=Mirror vertically)`<br>

The 32x28 map entries correspond to 256x224 pixels of the Super NES
screen. The 20x18 entries in the center of the 32x28 area should be set
to a blank (solid color 0) tile as transparent space for the Game Boy
window to be displayed inside. Non-transparent border data will cover
the Game Boy window (for example, *Mario's Picross* does this, as does
*WildSnake* to a lesser extent).

All borders repeat tiles. Assuming that the blank space for the GB
screen is a single tile, as is the letterbox in a widescreen border, a
border defining all unique tiles would have to define this many tiles:

-   (256\*224-160\*144)/64+1 = 537 tiles in fullscreen border
-   (256\*176-160\*144)/64+2 = 346 tiles in widescreen border

But the CHR RAM allocated by SGB for border holds only 256 tiles. This
means a fullscreen border must repeat at least 281 tiles and a
widescreen border at least 90.

### SGB Command 18h - OBJ_TRN

Used to transfer OBJ attributes to SNES OAM memory. Unlike all other
functions with the ending \_TRN, this function does not use the usual
one-shot 4KBytes VRAM transfer method. Instead, when enabled (below
execute bit set), data is permanently (each frame) read out from the
lower character line of the Game Boy screen. To suppress garbage on the
display, the lower line is masked, and only the upper 20x17 characters
of the Game Boy window are used - the masking method is unknwon - frozen,
black, or recommended to be covered by the SGB border, or else ??? Also,
when the function is enabled, "system attract mode is not performed" -
whatever that means ???

This command does nothing on some SGB revisions. (SGBv2, SGB2?)

` Byte  Content`<br>
` 0     Command*8+Length (fixed length=1)`<br>
` 1     Control Bits`<br>
`         Bit 0   - SNES OBJ Mode enable (0=Cancel, 1=Enable)`<br>
`         Bit 1   - Change OBJ Color     (0=No, 1=Use definitions below)`<br>
`         Bit 2-7 - Not used (zero)`<br>
` 2-3   System Color Palette Number for OBJ Palette 4 (0-511)`<br>
` 4-5   System Color Palette Number for OBJ Palette 5 (0-511)`<br>
` 6-7   System Color Palette Number for OBJ Palette 6 (0-511)`<br>
` 8-9   System Color Palette Number for OBJ Palette 7 (0-511)`<br>
`         These color entries are ignored if above Control Bit 1 is zero.`<br>
`         Because each OBJ palette consists of 16 colors, four system`<br>
`         palette entries (of 4 colors each) are transferred into each`<br>
`         OBJ palette. The system palette numbers are not required to be`<br>
`         aligned to a multiple of four, and will wrap to palette number`<br>
`         0 when exceeding 511. For example, a value of 511 would copy`<br>
`         system palettes 511, 0, 1, 2 to the SNES OBJ palette.`<br>
` A-F   Not used (zero)`<br>

The recommended method is to "display" Game Boy BG tiles F9h..FFh from
left to right as first 7 characters of the bottom-most character line of
the Game Boy screen. As for normal 4KByte VRAM transfers, this area
should not be scrolled, should not be overlapped by Game Boy OBJs, and
the Game Boy BGP palette register should be set up properly. By following
that method, SNES OAM data can be defined in the 70h bytes of the
gameboy BG tile memory at following addresses:

` 8F90-8FEF  SNES OAM, 24 Entries of 4 bytes each (96 bytes)`<br>
` 8FF0-8FF5  SNES OAM MSBs, 24 Entries of 2 bits each (6 bytes)`<br>
` 8FF6-8FFF  Not used, don't care (10 bytes)`<br>

The format of SNES OAM Entries is:

```
  Byte 0  OBJ X-Position (0-511, MSB is separately stored, see below) 
  Byte 1  OBJ Y-Position (0-255) 
  Byte 2-3  Attributes (16bit) 
    Bit 0-8    Tile Number     (use only 00h-FFh, upper bit zero) 
    Bit 9-11   Palette Number  (use only 4-7) 
    Bit 12-13  OBJ Priority    (use only 3) 
    Bit 14     X-Flip          (0=Normal, 1=Mirror horizontally) 
    Bit 15     Y-Flip          (0=Normal, 1=Mirror vertically) 
```

The format of SNES OAM MSB Entries is:

Actually, the format is unknown ??? However, 2 bits are used per entry:
One bit is the most significant bit of the OBJ X-Position.
The other bit specifies the OBJ size (8x8 or 16x16 pixels).

# Undocumented SGB commands

The following information has been extracted from disassembling a SGBv2
firmware; it should be verified on other SGB revisions.

The SGB firmware explicitly ignores all commands with ID >= $1E. This
leaves undocumented commands $19 to $1D inclusive.

### Stubbed commands

Commands $1A to $1F (inclusive)'s handlers are stubs (only contain a
`RTS`). This is interesting, since the command-processing function
explicitly ignores commands $1E and $1F.

### SGB command 19h

The game Donkey Kong '94 appears to send this command, and it appears
to set a flag in the SGB's memory. It's not known yet what it does,
though.