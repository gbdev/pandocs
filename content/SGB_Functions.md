SGB Description
---------------

### General Description

Basically, the SGB (Super Gameboy) is an adapter cartridge that allows
to play gameboy games on a SNES (Super Nintendo Entertainment System)
gaming console. In detail, you plug the gameboy cartridge into the SGB
cartridge, then plug the SGB cartridge into the SNES, and then connect
the SNES to your TV Set. In result, games can be played and viewed on
the TV Set, and are controlled by using the SNES joypad(s).

### More Technical Description

The SGB cartridge just contains a normal gameboy CPU and normal gameboy
video controller. Normally the video signal from this controller would
be sent to the LCD screen, however, in this special case the SNES read
out the video signal and displays it on the TV set by using a special
SNES BIOS ROM which is located in the SGB cartridge. Also, normal
gameboy sound output is forwared to the SNES and output to the TV Set,
vice versa, joypad input is forwared from the SNES controller(s) to the
gameboy joypad inputs.

### Normal Monochrome Games

Any gameboy games which have been designed for normal monochrome
handheld gameboys will work with the SGB hardware as well. The SGB will
apply a four color palette to these games by replacing the normal four
grayshades. The 160x144 pixel gamescreen is displayed in the middle of
the 256x224 pixel SNES screen (the unused area is filled by a screen
border bitmap). The user may access built-in menues, allowing to change
color palette data, to select between several pre-defined borders, etc.

Games that have been designed to support SGB functions may also access
the following additional features:

### Colorized Game Screen

There\'s limited ability to colorize the gamescreen by assigning custom
color palettes to each 20x18 display characters, however, this works
mainly for static display data such like title screens or status bars,
the 20x18 color attribute map is non-scrollable, and it is not possible
to assign separate colors to moveable foreground sprites (OBJs), so that
animated screen regions will be typically restricted to using a single
palette of four colors only.

### SNES Foreground Sprites

Up to 24 foreground sprites (OBJs) of 8x8 or 16x16 pixels, 16 colors can
be displayed. When replacing (or just overlaying) the normal gameboy
OBJs by SNES OBJs it\'d be thus possible to display OBJs with other
colors than normal background area. This method doesn\'t appear to be
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

Beside for normal gameboy sound, a number of digital sound effects is
pre-defined in the SNES BIOS, these effects may be accessed quite
easily. Programmers whom are familiar with SNES sounds may also access
the SNES sound chip, or use the SNES MIDI engine directly in order to
produce other sound effects or music.

### Taking Control of the SNES CPU

Finally, it is possible to write program code or data into SNES memory,
and to execute such program code by using the SNES CPU.

### SGB System Clock

Because the SGB is synchronized to the SNES CPU, the gameboy system
clock is directly chained to the SNES system clock. In result, the
gameboy CPU, video controller, timers, and sound frequencies will be all
operated approx 2.4% faster as by normal gameboys. Basically, this
should be no problem, and the game will just run a little bit faster.
However sensitive musicians may notice that sound frequencies are a bit
too high, programs that support SGB functions may avoid this effect by
reducing frequencies of gameboy sounds when having detected SGB
hardware. Also, I think that I\'ve heard that SNES models which use a
50Hz display refresh rate (rather than 60Hz) are resulting in
respectively slower SGB/gameboy timings ???

SGB Unlocking and Detecting SGB Functions
-----------------------------------------

### Cartridge Header

SGB games are required to have a cartridge header with Nintendo and
proper checksum just as normal gameboy games. Also, two special entries
must be set in order to unlock SGB functions:

` 146h - SGB Flag - Must be set to 03h for SGB games`\
` 14Bh - Old Licensee Code - Must be set 33h for SGB games`

When these entries aren\'t set, the game will still work just like all
\'monochrome\' gameboy games, but it cannot access any of the special
SGB functions.

### Detecting SGB hardware

The recommended detection method is to send a MLT\_REQ command which
enables two (or four) joypads. A normal handheld gameboy will ignore
this command, a SGB will now return incrementing joypad IDs each time
when deselecting keyboard lines (see MLT\_REQ description for details).
Now read-out joypad state/IDs several times, and if the ID-numbers are
changing, then it is a SGB (a normal gameboy would typically always
return 0Fh as ID). Finally, when not intending to use more than one
joypad, send another MLT\_REQ command in order to re-disable the
multi-controller mode. Detection works regardless of whether and how
many joypads are physically connected to the SNES. However, detection
works only when having unlocked SGB functions in the cartridge header,
as described above.

### Separating between SGB and SGB2

It is also possible to separate between SGB and SGB2 models by examining
the inital value of the accumulator (A-register) directly after startup.

` 01h  SGB or Normal Gameboy (DMG)`\
` FFh  SGB2 or Pocket Gameboy`\
` 11h  CGB or GBA`

Because values 01h and FFh are shared for both handhelds and SGBs, it is
still required to use the above MLT\_REQ detection procedure. As far as
I know the SGB2 doesn\'t have any extra features which\'d require
separate SGB2 detection except for curiosity purposes, for example, the
game \"Tetris DX\" chooses to display an alternate SGB border on SGB2s.

Reportedly, some SGB models include link ports (just like handheld
gameboy) (my own SGB does not have such an port), possibly this feature
is available in SGB2-type models only ???

SGB Command Packet Transfers
----------------------------

Command packets (aka Register Files) are transferred from the gameboy to
the SNES by using P14 and P15 output lines of the JOYPAD register
(FF00h), these lines are normally used to select the two rows in the
gameboy keyboard matrix (which still works).

### Transferring Bits

A command packet transfer must be initiated by setting both P14 and P15
to LOW, this will reset and start the SNES packet receiving program.
Data is then transferred (LSB first), setting P14=LOW will indicate a
\"0\" bit, and setting P15=LOW will indicate a \"1\" bit. For example:

`      RESET 0   0   1   1   0   1   0`\
` P14  --_---_---_-----------_-------_--...`\
` P15  --_-----------_---_-------_------...`

Data and reset pulses must be kept LOW for at least 5us. P14 and P15
must be kept both HIGH for at least 15us between any pulses. Obviously,
it\'d be no good idea to access the JOYPAD register during the transfer,
for example, in case that your VBlank interrupt procedure reads-out
joypad states each frame, be sure to disable that interrupt during the
transfer (or disable only the joypad procedure by using a software
flag).

### Transferring Packets

Each packet is invoked by a RESET pulse, then 128 bits of data are
transferred (16 bytes, LSB of first byte first), and finally, a
\"0\"-bit must be transferred as stop bit. The structure of normal
packets is:

`  1 PULSE Reset`\
`  1 BYTE  Command Code*8+Length`\
` 15 BYTES Parameter Data`\
`  1 BIT   Stop Bit (0)`

The above \'Length\' indicates the total number of packets (1-7,
including the first packet) which will be sent, ie. if more than 15
parameter bytes are used, then further packet(s) will follow, as such:

`  1 PULSE Reset`\
` 16 BYTES Parameter Data`\
`  1 BIT   Stop Bit (0)`

By using all 7 packets, up to 111 data bytes (15+16\*6) may be sent.
Unused bytes at the end of the last packet must be set to zero. A 60ms
(4 frames) delay should be invoked between each packet transfer.

SGB VRAM Transfers
------------------

### Overview

Beside for the packet transfer method, larger data blocks of 4KBytes can
be transferred by using the video signal. These transfers are invoked by
first sending one of the commands with the ending \_TRN (by using normal
packet transfer), the 4K data block is then read-out by the SNES from
gameboy display memory during the next frame.

### Transfer Data

Normally, transfer data should be stored at 8000h-8FFFh in gameboy VRAM,
even though the SNES receives the data in from display scanlines, it
will automatically re-produce the same ordering of bits and bytes, as
being originally stored at 8000h-8FFFh in gameboy memory.

### Preparing the Display

The above method works only when recursing the following things: BG Map
must display unsigned characters 00h-FFh on the screen; 00h..13h in
first line, 14h..27h in next line, etc. The gameboy display must be
enabled, the display may not be scrolled, OBJ sprites should not overlap
the background tiles, the BGP palette register must be set to E4h.

### Transfer Time

Note that the transfer data should be prepared in VRAM <before> sending
the transfer command packet. The actual transfer starts at the beginning
of the next frame after the command has been sent, and the transfer ends
at the end of the 5th frame after the command has been sent (not
counting the frame in which the command has been sent).

Avoiding Screen Garbage The display will contain \'garbage\' during the
transfer, this dirt-effect can be avoided by freezing the screen (in the
state which has been displayed before the transfer) by using the
MASK\_EN command. Of course, this works only when actually executing the
game on a SGB (and not on normal handheld gameboys), it\'d be thus
required to detect the presence of SGB hardware before blindly sending
VRAM data.

SGB Command Summary
-------------------

### SGB System Command Table

` Code Name      Expl.`\
` 00   PAL01     Set SGB Palette 0,1 Data`\
` 01   PAL23     Set SGB Palette 2,3 Data`\
` 02   PAL03     Set SGB Palette 0,3 Data`\
` 03   PAL12     Set SGB Palette 1,2 Data`\
` 04   ATTR_BLK  "Block" Area Designation Mode`\
` 05   ATTR_LIN  "Line" Area Designation Mode`\
` 06   ATTR_DIV  "Divide" Area Designation Mode`\
` 07   ATTR_CHR  "1CHR" Area Designation Mode`\
` 08   SOUND     Sound On/Off`\
` 09   SOU_TRN   Transfer Sound PRG/DATA`\
` 0A   PAL_SET   Set SGB Palette Indirect`\
` 0B   PAL_TRN   Set System Color Palette Data`\
` 0C   ATRC_EN   Enable/disable Attraction Mode`\
` 0D   TEST_EN   Speed Function`\
` 0E   ICON_EN   SGB Function`\
` 0F   DATA_SND  SUPER NES WRAM Transfer 1`\
` 10   DATA_TRN  SUPER NES WRAM Transfer 2`\
` 11   MLT_REG   Controller 2 Request`\
` 12   JUMP      Set SNES Program Counter`\
` 13   CHR_TRN   Transfer Character Font Data`\
` 14   PCT_TRN   Set Screen Data Color Data`\
` 15   ATTR_TRN  Set Attribute from ATF`\
` 16   ATTR_SET  Set Data to ATF`\
` 17   MASK_EN   Game Boy Window Mask`\
` 18   OBJ_TRN   Super NES OBJ Mode`

SGB Color Palettes Overview
---------------------------

### Available SNES Palettes

The SGB/SNES provides 8 palettes of 16 colors each, each color may be
defined out of a selection of 34768 colors (15 bit). Palettes 0-3 are
used to colorize the gamescreen, only the first four colors of each of
these palettes are used. Palettes 4-7 are used for the SGB Border, all
16 colors of each of these palettes may be used.

### Color 0 Restriction

Color 0 of each of the eight palettes is transparent, causing the
backdrop color to be displayed instead. The backdrop color is typically
defined by the most recently color being assigned to Color 0 (regardless
of the palette number being used for that operation). Effectively,
gamescreen palettes can have only three custom colors each, and SGB
border palettes only 15 colors each, additionally, color 0 can be used
for for all palettes, which will then all share the same color though.

### Translation of Grayshades into Colors

Because the SGB/SNES reads out the gameboy video controllers display
signal, it translates the different grayshades from the signal into SNES
colors as such:

` White       -->  Color 0`\
` Light Gray  -->  Color 1`\
` Dark Gray   -->  Color 2`\
` Black       -->  Color 3`

Note that gameboy colors 0-3 are assigned to user-selectable grayshades
by the gameboys BGP, OBP1, and OBP2 registers. There is thus no fixed
relationship between gameboy colors 0-3 and SNES colors 0-3.

### Using Gameboy BGP/OBP Registers

A direct translation of color 0-3 into color 0-3 may be produced by
setting BGP/OBP registers to a value of 0E4h each. However, in case that
your program uses black background for example, then you may internally
assign background as \"White\" at the gameboy side by BGP/OBP registers
(which is then interpreted as SNES color 0, which is shared for all SNES
palettes). The advantage is that you may define Color 0 as Black at the
SNES side, and may assign custom colors for Colors 1-3 of each SNES
palette.

### System Color Palette Memory

Beside for the actually visible palettes, up to 512 palettes of 4 colors
each may be defined in SNES RAM. Basically, this is completely
irrelevant because the palettes are just stored in RAM whithout any
relationship to the displayed picture, anyways, these pre-defined colors
may be transferred to actually visible palettes slightly faster as when
transferring palette data by separate command packets.

SGB Palette Commands
--------------------

### SGB Command 00h - PAL01

Transmit color data for SGB palette 0, color 0-3, and for SGB palette 1,
color 1-3 (without separate color 0).

` Byte  Content`\
` 0     Command*8+Length (fixed length=01h)`\
` 1-E   Color Data for 7 colors of 2 bytes (16bit) each:`\
`         Bit 0-4   - Red Intensity   (0-31)`\
`         Bit 5-9   - Green Intensity (0-31)`\
`         Bit 10-14 - Blue Intensity  (0-31)`\
`         Bit 15    - Not used (zero)`\
` F     Not used (00h)`

The value transferred as color 0 will be applied for all eight palettes.

### SGB Command 01h - PAL23

Same as above PAL01, but for Palettes 2 and 3 respectively.

### SGB Command 02h - PAL03

Same as above PAL01, but for Palettes 0 and 3 respectively.

### SGB Command 03h - PAL12

Same as above PAL01, but for Palettes 1 and 2 respectively.

### SGB Command 0Ah - PAL\_SET

Used to copy pre-defined palette data from SGB system color palette to
actual SGB palette.

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1-2   System Palette number for SGB Color Palette 0 (0-511)`\
` 3-4   System Palette number for SGB Color Palette 1 (0-511)`\
` 5-6   System Palette number for SGB Color Palette 2 (0-511)`\
` 7-8   System Palette number for SGB Color Palette 3 (0-511)`\
` 9     Attribute File`\
`         Bit 0-5 - Attribute File Number (00h-2Ch) (Used only if Bit7=1)`\
`         Bit 6   - Cancel Mask           (0=No change, 1=Yes)`\
`         Bit 7   - Use Attribute File    (0=No, 1=Apply above ATF Number)`\
` A-F   Not used (zero)`

Before using this function, System Palette data should be initialized by
PAL\_TRN command, and (when used) Attribute File data should be
initialized by ATTR\_TRN.

### SGB Command 0Bh - PAL\_TRN

Used to initialize SGB system color palettes in SNES RAM. System color
palette memory contains 512 pre-defined palettes, these palettes do not
directly affect the display, however, the PAL\_SET command may be later
used to transfer four of these \'logical\' palettes to actual visible
\'physical\' SGB palettes. Also, the OBJ\_TRN function will use groups
of 4 System Color Palettes (4\*4 colors) for SNES OBJ palettes (16
colors).

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1-F   Not used (zero)`

The palette data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Data for System Color Palette 0-511`

Each Palette consists of four 16bit-color definitions (8 bytes). Note:
The data is stored at 3000h-3FFFh in SNES memory.

SGB Color Attribute Commands
----------------------------

### SGB Command 04h - ATTR\_BLK

Used to specify color attributes for the inside or outside of one or
more rectangular screen regions.

` Byte  Content`\
` 0     Command*8+Length (length=1..7)`\
` 1     Number of Data Sets (01h..12h)`\
` 2-7   Data Set #1`\
`         Byte 0 - Control Code (0-7)`\
`           Bit 0 - Change Colors inside of surrounded area     (1=Yes)`\
`           Bit 1 - Change Colors of surrounding character line (1=Yes)`\
`           Bit 2 - Change Colors outside of surrounded area    (1=Yes)`\
`           Bit 3-7 - Not used (zero)`\
`           Exception: When changing only the Inside or Outside, then the`\
`           Surrounding line becomes automatically changed to same color.`\
`         Byte 1 - Color Palette Designation`\
`           Bit 0-1 - Palette Number for inside of surrounded area`\
`           Bit 2-3 - Palette Number for surrounding character line`\
`           Bit 4-5 - Palette Number for outside of surrounded area`\
`           Bit 6-7 - Not used (zero)`\
`         Data Set Byte 2 - Coordinate X1 (left)`\
`         Data Set Byte 3 - Coordinate Y1 (upper)`\
`         Data Set Byte 4 - Coordinate X2 (right)`\
`         Data Set Byte 5 - Coordinate Y2 (lower)`\
`           Specifies the coordinates of the surrounding rectangle.`\
` 8-D   Data Set #2 (if any)`\
` E-F   Data Set #3 (continued at 0-3 in next packet) (if any)`

When sending three or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets is described below.

### SGB Command 05h - ATTR\_LIN

Used to specify color attributes of one or more horizontal or vertical
character lines.

` Byte  Content`\
` 0     Command*8+Length (length=1..7)`\
` 1     Number of Data Sets (01h..6Eh) (one byte each)`\
` 2     Data Set #1`\
`         Bit 0-4 - Line Number    (X- or Y-coordinate, depending on bit 7)`\
`         Bit 5-6 - Palette Number (0-3)`\
`         Bit 7   - H/V Mode Bit   (0=Vertical line, 1=Horizontal Line)`\
` 3     Data Set #2 (if any)`\
` 4     Data Set #3 (if any)`\
` etc.`

When sending 15 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets (one byte each) is described
below. The length of each line reaches from one end of the screen to the
other end. In case that some lines overlap each other, then lines from
lastmost data sets will overwrite lines from previous data sets.

### SGB Command 06h - ATTR\_DIV

Used to split the screen into two halfes, and to assign separate color
attributes to each half, and to the division line between them.

` Byte  Content`\
` 0     Command*8+Length   (fixed length=1)`\
` 1     Color Palette Numbers and H/V Mode Bit`\
`         Bit 0-1  Palette Number below/right of division line`\
`         Bit 2-3  Palette Number above/left of division line`\
`         Bit 4-5  Palette Number for division line`\
`         Bit 6    H/V Mode Bit  (0=split left/right, 1=split above/below)`\
` 2     X- or Y-Coordinate (depending on H/V bit)`\
` 3-F   Not used (zero)`

### SGB Command 07h - ATTR\_CHR

Used to specify color attributes for separate characters.

` Byte  Content`\
` 0     Command*8+Length (length=1..6)`\
` 1     Beginning X-Coordinate`\
` 2     Beginning Y-Coordinate`\
` 3-4   Number of Data Sets (1-360)`\
` 5     Writing Style   (0=Left to Right, 1=Top to Bottom)`\
` 6     Data Sets 1-4   (Set 1 in MSBs, Set 4 in LSBs)`\
` 7     Data Sets 5-8   (if any)`\
` 8     Data Sets 9-12  (if any)`\
` etc.`

When sending 41 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. Each data set consists of two bits, indicating the palette number
for one character. Depending on the writing style, data sets are written
from left to right, or from top to bottom. In either case the function
wraps to the next row/column when reaching the end of the screen.

### SGB Command 15h - ATTR\_TRN

Used to initialize Attribute Files (ATFs) in SNES RAM. Each ATF consists
of 20x18 color attributes for the gameboy screen. This function does not
directly affect display attributes. Instead, one of the defined ATFs may
be copied to actual display memory at a later time by using ATTR\_SET or
PAL\_SET functions.

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1-F   Not used (zero)`

The ATF data is sent by VRAM-Transfer (4 KBytes).

` 000-FD1  Data for ATF0 through ATF44 (4050 bytes)`\
` FD2-FFF  Not used`

Each ATF consists of 90 bytes, that are 5 bytes (20x2bits) for each of
the 18 character lines of the gameboy window. The two most significant
bits of the first byte define the color attribute (0-3) for the first
character of the first line, the next two bits the next character, and
so on.

### SGB Command 16h - ATTR\_SET

Used to transfer attributes from Attribute File (ATF) to gameboy window.

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1     Attribute File Number (00-2Ch), Bit 6=Cancel Mask`\
` 2-F   Not used (zero)`

When above Bit 6 is set, the gameboy screen becomes re-enabled after the
transfer (in case it has been disabled/frozen by MASK\_EN command).
Note: The same functions may be (optionally) also included in PAL\_SET
commands, as described in the chapter about Color Palette Commands.

SGB Sound Functions
-------------------

### SGB Command 08h - SOUND

Used to start/stop internal sound effect, start/stop sound using
internal tone data.

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1     Sound Effect A (Port 1) Decrescendo 8bit Sound Code`\
` 2     Sound Effect B (Port 2) Sustain     8bit Sound Code`\
` 3     Sound Effect Attributes`\
`         Bit 0-1 - Sound Effect A Pitch  (0..3=Low..High)`\
`         Bit 2-3 - Sound Effect A Volume (0..2=High..Low, 3=Mute on)`\
`         Bit 4-5 - Sound Effect B Pitch  (0..3=Low..High)`\
`         Bit 6-7 - Sound Effect B Volume (0..2=High..Low, 3=Not used)`\
` 4     Music Score Code (must be zero if not used)`\
` 5-F   Not used (zero)`

See Sound Effect Tables below for a list of available pre-defined
effects. \"Notes\" 1) Mute is only active when both bits D2 and D3 are
1. 2) When the volume is set for either Sound Effect A or Sound Effect
B, mute is turned off. 3) When Mute on/off has been executed, the sound
fades out/fades in. 4) Mute on/off operates on the (BGM) which is
reproduced by Sound Effect A, Sound Effect B, and the Super NES APU. A
\"mute off\" flag does not exist by itself. When mute flag is set,
volume and pitch of Sound Effect A (port 1) and Sound Effect B (port 2)
must be set.

### SGB Command 09h - SOU\_TRN

Used to transfer sound code or data to SNES Audio Processing Unit memory
(APU-RAM).

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1-F   Not used (zero)`

The sound code/data is sent by VRAM-Transfer (4 KBytes).

` 000      One (or two ???) 16bit expression(s ???) indicating the`\
`          transfer destination address and transfer length.`\
` ...-...  Transfer Data`\
` ...-FFF  Remaining bytes not used`

Possible destinations in APU-RAM are:

` 0400h-2AFFh  APU-RAM Program Area (9.75KBytes)`\
` 2B00h-4AFFh  APU-RAM Sound Score Area (8Kbytes)`\
` 4DB0h-EEFFh  APU-RAM Sampling Data Area (40.25 Kbytes)`

This function may be used to take control of the SNES sound chip, and/or
to access the SNES MIDI engine. In either case it requires deeper
knowledge of SNES sound programming.

### SGB Sound Effect A/B Tables

Below lists the digital sound effects that are pre-defined in the
SGB/SNES BIOS, and which can be used with the SGB \"SOUND\" Command.
Effect A and B may be simultaneously reproduced. The P-column indicates
the recommended Pitch value, the V-column indicates the numbers of
Voices used. Sound Effect A uses voices 6,7. Sound Effect B uses voices
0,1,4,5. Effects that use less voices will use only the upper voices
(eg. 4,5 for Effect B with only two voices).

### Sound Effect A Flag Table

` Code Description             P V     Code Description             P V`\
` 00  Dummy flag, re-trigger   - 2     18  Fast Jump                3 1`\
` 80  Effect A, stop/silent    - 2     19  Jet (rocket) takeoff     0 1`\
` 01  Nintendo                 3 1     1A  Jet (rocket) landing     0 1`\
` 02  Game Over                3 2     1B  Cup breaking             2 2`\
` 03  Drop                     3 1     1C  Glass breaking           1 2`\
` 04  OK ... A                 3 2     1D  Level UP                 2 2`\
` 05  OK ... B                 3 2     1E  Insert air               1 1`\
` 06  Select...A               3 2     1F  Sword swing              1 1`\
` 07  Select...B               3 1     20  Water falling            2 1`\
` 08  Select...C               2 2     21  Fire                     1 1`\
` 09  Mistake...Buzzer         2 1     22  Wall collapsing          1 2`\
` 0A  Catch Item               2 2     23  Cancel                   1 2`\
` 0B  Gate squeaks 1 time      2 2     24  Walking                  1 2`\
` 0C  Explosion...small        1 2     25  Blocking strike          1 2`\
` 0D  Explosion...medium       1 2     26  Picture floats on & off  3 2`\
` 0E  Explosion...large        1 2     27  Fade in                  0 2`\
` 0F  Attacked...A             3 1     28  Fade out                 0 2`\
` 10  Attacked...B             3 2     29  Window being opened      1 2`\
` 11  Hit (punch)...A          0 2     2A  Window being closed      0 2`\
` 12  Hit (punch)...B          0 2     2B  Big Laser                3 2`\
` 13  Breath in air            3 2     2C  Stone gate closes/opens  0 2`\
` 14  Rocket Projectile...A    3 2     2D  Teleportation            3 1`\
` 15  Rocket Projectile...B    3 2     2E  Lightning                0 2`\
` 16  Escaping Bubble          2 1     2F  Earthquake               0 2`\
` 17  Jump                     3 1     30  Small Laser              2 2`

Sound effect A is used for formanto sounds (percussion sounds).

### Sound Effect B Flag Table

` Code Description             P V     Code Description             P V`\
` 00  Dummy flag, re-trigger   - 4     0D  Waterfall                2 2`\
` 80  Effect B, stop/silent    - 4     0E  Small character running  3 1`\
` 01  Applause...small group   2 1     0F  Horse running            3 1`\
` 02  Applause...medium group  2 2     10  Warning sound            1 1`\
` 03  Applause...large group   2 4     11  Approaching car          0 1`\
` 04  Wind                     1 2     12  Jet flying               1 1`\
` 05  Rain                     1 1     13  UFO flying               2 1`\
` 06  Storm                    1 3     14  Electromagnetic waves    0 1`\
` 07  Storm with wind/thunder  2 4     15  Score UP                 3 1`\
` 08  Lightning                0 2     16  Fire                     2 1`\
` 09  Earthquake               0 2     17  Camera shutter, formanto 3 4`\
` 0A  Avalanche                0 2     18  Write, formanto          0 1`\
` 0B  Wave                     0 1     19  Show up title, formanto  0 1`\
` 0C  River                    3 2`

Sound effect B is mainly used for looping sounds (sustained sounds).

SGB System Control Commands
---------------------------

### SGB Command 17h - MASK\_EN

Used to mask the gameboy window, among others this can be used to freeze
the gameboy screen before transferring data through VRAM (the SNES then
keeps displaying the gameboy screen, even though VRAM doesn\'t contain
meaningful display information during the transfer).

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1     Gameboy Screen Mask (0-3)`\
`         0  Cancel Mask   (Display activated)`\
`         1  Freeze Screen (Keep displaying current picture)`\
`         2  Blank Screen  (Black)`\
`         3  Blank Screen  (Color 0)`\
` 2-F   Not used (zero)`

Freezing works only if the SNES has stored a picture, ie. if necessary
wait one or two frames before freezing (rather than freezing directly
after having displayed the picture). The Cancel Mask function may be
also invoked (optionally) by completion of PAL\_SET and ATTR\_SET
commands.

### SGB Command 0Ch - ATRC\_EN

Used to enable/disable Attraction mode. It is totally unclear what an
attraction mode is ???, but it is enabled by default.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     Attraction Disable  (0=Enable, 1=Disable)`\
` 2-F   Not used (zero)`

### SGB Command 0Dh - TEST\_EN

Used to enable/disable test mode for \"SGB-CPU variable clock speed
function\". This function is disabled by default.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     Test Mode Enable    (0=Disable, 1=Enable)`\
` 2-F   Not used (zero)`

Maybe intended to determine whether SNES operates at 50Hz or 60Hz
display refresh rate ??? Possibly result can be read-out from joypad
register ???

### SGB Command 0Eh - ICON\_EN

Used to enable/disable ICON function. Possibly meant to enable/disable
SGB/SNES popup menues which might otherwise activated during gameboy
game play. By default all functions are enabled (0).

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     Disable Bits`\
`         Bit 0 - Use of SGB-Built-in Color Palettes    (1=Disable)`\
`         Bit 1 - Controller Set-up Screen    (0=Enable, 1=Disable)`\
`         Bit 2 - SGB Register File Transfer (0=Receive, 1=Disable)`\
`         Bit 3-6 - Not used (zero)`\
` 2-F   Not used (zero)`

Above Bit 2 will suppress all further packets/commands when set, this
might be useful when starting a monochrome game from inside of the
SGB-menu of a multi-gamepak which contains a collection of different
games.

### SGB Command 0Fh - DATA\_SND

Used to write one or more bytes directly into SNES Work RAM.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     SNES Destination Address, low`\
` 2     SNES Destination Address, high`\
` 3     SNES Destination Address, bank number`\
` 4     Number of bytes to write (01h-0Bh)`\
` 5     Data Byte #1`\
` 6     Data Byte #2 (if any)`\
` 7     Data Byte #3 (if any)`\
` etc.`

Unused bytes at the end of the packet should be set to zero, this
function is restricted to a single packet, so that not more than 11
bytes can be defined at once. Free Addresses in SNES memory are Bank 0
1800h-1FFFh, Bank 7Fh 0000h-FFFFh.

### SGB Command 10h - DATA\_TRN

Used to transfer binary code or data directly into SNES RAM.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     SNES Destination Address, low`\
` 2     SNES Destination Address, high`\
` 3     SNES Destination Address, bank number`\
` 4-F   Not used (zero)`

The data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Data`

Free Addresses in SNES memory are Bank 0 1800h-1FFFh, Bank 7Fh
0000h-FFFFh. The transfer length is fixed at 4KBytes ???, so that
directly writing to the free 2KBytes at 0:1800h would be a not so good
idea ???

### SGB Command 12h - JUMP

Used to set the SNES program counter to a specified address. Optionally,
it may be used to set a new address for the SNES NMI handler, the NMI
handler remains unchanged if all bytes 4-6 are zero.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     SNES Program Counter, low`\
` 2     SNES Program Counter, high`\
` 3     SNES Program Counter, bank number`\
` 4     SNES NMI Handler, low`\
` 5     SNES NMI Handler, high`\
` 6     SNES NMI Handler, bank number`\
` 7-F   Not used, zero`

Note: The game \"Space Invaders 94\" uses this function when selecting
\"Arcade mode\" to execute SNES program code which has been previously
transferred from the SGB to the SNES. The type of the CPU which is used
in the SNES is unknown ???

SGB Multiplayer Command
-----------------------

### SGB Command 11h - MLT\_REQ

Used to request multiplayer mode (ie. input from more than one joypad).
Because this function provides feedback from the SGB/SNES to the gameboy
program, it is also used to detect SGB hardware.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     Multiplayer Control (0-3) (Bit0=Enable, Bit1=Two/Four Players)`\
`         0 = One player`\
`         1 = Two players`\
`         3 = Four players`\
` 2-F   Not used (zero)`

In one player mode, the second joypad (if any) is used for the SGB
system program. In two player mode, both joypads are used for the game.
Because SNES have only two joypad sockets, four player mode requires an
external \"Multiplayer 5\" adapter.

### Reading Multiple Controllers (Joypads)

When having enabled multiple controllers by MLT\_REQ, data for each
joypad can be read out through JOYPAD register (FF00) as follows: First
set P14 and P15 both HIGH (deselect both Buttons and Cursor keys), you
can now read the lower 4bits of FF00 which indicate the joypad ID for
the following joypad input:

` 0Fh  Joypad 1`\
` 0Eh  Joypad 2`\
` 0Dh  Joypad 3`\
` 0Ch  Joypad 4`

Next, read joypad state as normally. (Actually, just setting P15 LOW is
enough). When completed, set P14 and P15 back HIGH, this automatically
increments the joypad number (or restarts counting once reached the
lastmost joypad). Repeat the procedure until you have read-out states
for all two (or four) joypads.

SGB Border and OBJ Commands
---------------------------

### SGB Command 13h - CHR\_TRN

Used to transfer tile data (characters) to SNES Tile memory in VRAM.
This normally used to define BG tiles for the SGB Border (see PCT\_TRN),
but might be also used to define moveable SNES foreground sprites (see
OBJ\_TRN).

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1     Tile Transfer Destination`\
`         Bit 0   - Tile Numbers   (0=Tiles 00h-7Fh, 1=Tiles 80h-FFh)`\
`         Bit 1   - Tile Type      (0=BG Tiles, 1=OBJ Tiles)`\
`         Bit 2-7 - Not used (zero)`\
` 2-F   Not used (zero)`

The tile data is sent by VRAM-Transfer (4 KBytes).

` 000-FFF  Bitmap data for 128 Tiles`

Each tile occupies 32 bytes (8x8 pixels, 16 colors each). When intending
to transfer more than 128 tiles, call this function twice (once for
tiles 00h-7Fh, and once for tiles 80h-FFh). Note: The BG/OBJ Bit seems
to have no effect and writes to the same VRAM addresses for both BG and
OBJ ???

TODO: explain tile format

### SGB Command 14h - PCT\_TRN

Used to transfer tile map data and palette data to SNES BG Map memory in
VRAM to be used for the SGB border. The actual tiles must be separately
transferred by using the CHR\_TRN function.

` Byte  Content`\
` 0     Command*8+Length    (fixed length=1)`\
` 1-F   Not used (zero)`

The map data is sent by VRAM-Transfer (4 KBytes).

` 000-7FF  BG Map 32x32 Entries of 16bit each (2048 bytes)`\
` 800-87F  BG Palette Data (Palettes 4-7, each 16 colors of 16bits each)`\
` 880-FFF  Not used, don't care`

Each BG Map Entry consists of a 16bit value as such:

` Bit 0-9   - Character Number (use only 00h-FFh, upper 2 bits zero)`\
` Bit 10-12 - Palette Number   (use only 4-7, officially use only 4-6)`\
` Bit 13    - BG Priority      (use only 0)`\
` Bit 14    - X-Flip           (0=Normal, 1=Mirror horizontally)`\
` Bit 15    - Y-Flip           (0=Normal, 1=Mirror vertically)`

Even though 32x32 map entries are transferred, only upper 32x28 are
actually used (256x224 pixels, SNES screen size). The 20x18 entries in
the center of the 32x28 area should be set to 0000h as transparent space
for the gameboy window to be displayed inside. Reportedly,
non-transparent border data will cover the gameboy window.

### SGB Command 18h - OBJ\_TRN

Used to transfer OBJ attributes to SNES OAM memory. Unlike all other
functions with the ending \_TRN, this function does not use the usual
one-shot 4KBytes VRAM transfer method. Instead, when enabled (below
execute bit set), data is permanently (each frame) read out from the
lower character line of the gameboy screen. To suppress garbage on the
display, the lower line is masked, and only the upper 20x17 characters
of the gameboy window are used - the masking method is unknwon - frozen,
black, or recommended to be covered by the SGB border, or else ??? Also,
when the function is enabled, \"system attract mode is not performed\" -
whatever that means ???

` Byte  Content`\
` 0     Command*8+Length (fixed length=1)`\
` 1     Control Bits`\
`         Bit 0   - SNES OBJ Mode enable (0=Cancel, 1=Enable)`\
`         Bit 1   - Change OBJ Color     (0=No, 1=Use definitions below)`\
`         Bit 2-7 - Not used (zero)`\
` 2-3   System Color Palette Number for OBJ Palette 4 (0-511)`\
` 4-5   System Color Palette Number for OBJ Palette 5 (0-511)`\
` 6-7   System Color Palette Number for OBJ Palette 6 (0-511)`\
` 8-9   System Color Palette Number for OBJ Palette 7 (0-511)`\
`         These color entries are ignored if above Control Bit 1 is zero.`\
`         Because each OBJ palette consists of 16 colors, four system`\
`         palette entries (of 4 colors each) are transferred into each`\
`         OBJ palette. The system palette numbers are not required to be`\
`         aligned to a multiple of four, and will wrap to palette number`\
`         0 when exceeding 511. For example, a value of 511 would copy`\
`         system palettes 511, 0, 1, 2 to the SNES OBJ palette.`\
` A-F   Not used (zero)`

The recommended method is to \"display\" gameboy BG tiles F9h..FFh from
left to right as first 7 characters of the bottom-most character line of
the gameboy screen. As for normal 4KByte VRAM transfers, this area
should not be scrolled, should not be overlapped by gameboy OBJs, and
the gameboy BGP palette register should be set up properly. By following
that method, SNES OAM data can be defined in the 70h bytes of the
gameboy BG tile memory at following addresses:

` 8F90-8FEF  SNES OAM, 24 Entries of 4 bytes each (96 bytes)`\
` 8FF0-8FF5  SNES OAM MSBs, 24 Entries of 2 bits each (6 bytes)`\
` 8FF6-8FFF  Not used, don't care (10 bytes)`

The format of SNES OAM Entries is:

` Byte 0  OBJ X-Position (0-511, MSB is separately stored, see below)`\
` Byte 1  OBJ Y-Position (0-255)`\
` Byte 2-3  Attributes (16bit)`\
`   Bit 0-8    Tile Number     (use only 00h-FFh, upper bit zero)`\
`   Bit 9-11   Palette Number  (use only 4-7)`\
`   Bit 12-13  OBJ Priority    (use only 3)`\
`   Bit 14     X-Flip          (0=Normal, 1=Mirror horizontally)`\
`   Bit 15     Y-Flip          (0=Normal, 1=Mirror vertically)`

The format of SNES OAM MSB Entries is:

` Actually, the format is unknown ??? However, 2 bits are used per entry:`\
` One bit is the most significant bit of the OBJ X-Position.`\
` The other bit specifies the OBJ size (8x8 or 16x16 pixels).`

