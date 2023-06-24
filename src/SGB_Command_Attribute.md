# Color Attribute Commands

## SGB Command $04 — ATTR_BLK

Used to specify color attributes for the inside or outside of one or
more rectangular screen regions.

```
 Byte  Content
 0     Command*8+Length (length=1..7)
 1     Number of Data Sets ($01..$12)
 2-7   Data Set #1
         Byte 0 - Control Code (0-7)
           Bit 0 - Change Colors inside of surrounded area     (1=Yes)
           Bit 1 - Change Colors of surrounding character line (1=Yes)
           Bit 2 - Change Colors outside of surrounded area    (1=Yes)
           Bit 3-7 - Not used (zero)
           Exception: When changing only the Inside or Outside, then the
           Surrounding line becomes automatically changed to same color.
         Byte 1 - Color Palette Designation
           Bit 0-1 - Palette Number for inside of surrounded area
           Bit 2-3 - Palette Number for surrounding character line
           Bit 4-5 - Palette Number for outside of surrounded area
           Bit 6-7 - Not used (zero)
         Data Set Byte 2 - Coordinate X1 (left)
         Data Set Byte 3 - Coordinate Y1 (upper)
         Data Set Byte 4 - Coordinate X2 (right)
         Data Set Byte 5 - Coordinate Y2 (lower)
           Specifies the coordinates of the surrounding rectangle.
 8-D   Data Set #2 (if any)
 E-F   Data Set #3 (continued at 0-3 in next packet) (if any)
```

When sending three or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets is described below.

## SGB Command $05 — ATTR_LIN

Used to specify color attributes of one or more horizontal or vertical
character lines.

```
 Byte  Content
 0     Command*8+Length (length=1..7)
 1     Number of Data Sets ($01..$6E) (one byte each)
 2     Data Set #1
         Bit 0-4 - Line Number    (X- or Y-coordinate, depending on bit 7)
         Bit 5-6 - Palette Number (0-3)
         Bit 7   - H/V Mode Bit   (0=Vertical line, 1=Horizontal Line)
 3     Data Set #2 (if any)
 4     Data Set #3 (if any)
 etc.
```

When sending 15 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. The format of the separate Data Sets (one byte each) is described
below. The length of each line reaches from one end of the screen to the
other end. In case that some lines overlap each other, then lines from
lastmost data sets will overwrite lines from previous data sets.

## SGB Command $06 — ATTR_DIV

Used to split the screen into two halfes, and to assign separate color
attributes to each half, and to the division line between them.

```
 Byte  Content
 0     Command*8+Length   (fixed length=1)
 1     Color Palette Numbers and H/V Mode Bit
         Bit 0-1  Palette Number below/right of division line
         Bit 2-3  Palette Number above/left of division line
         Bit 4-5  Palette Number for division line
         Bit 6    H/V Mode Bit  (0=split left/right, 1=split above/below)
 2     X- or Y-Coordinate (depending on H/V bit)
 3-F   Not used (zero)
```

## SGB Command $07 — ATTR_CHR

Used to specify color attributes for separate characters.

```
 Byte  Content
 0     Command*8+Length (length=1..6)
 1     Beginning X-Coordinate
 2     Beginning Y-Coordinate
 3-4   Number of Data Sets (1-360)
 5     Writing Style   (0=Left to Right, 1=Top to Bottom)
 6     Data Sets 1-4   (Set 1 in MSBs, Set 4 in LSBs)
 7     Data Sets 5-8   (if any)
 8     Data Sets 9-12  (if any)
 etc.
```

When sending 41 or more data sets, data is continued in further
packet(s). Unused bytes at the end of the last packet should be set to
zero. Each data set consists of two bits, indicating the palette number
for one character. Depending on the writing style, data sets are written
from left to right, or from top to bottom. In either case the function
wraps to the next row/column when reaching the end of the screen.

## SGB Command $15 — ATTR_TRN

Used to initialize Attribute Files (ATFs) in SNES RAM. Each ATF consists
of 20×18 color attributes for the Game Boy screen. This function does not
directly affect display attributes. Instead, one of the defined ATFs may
be copied to actual display memory at a later time by using ATTR_SET or
PAL_SET functions.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1-F   Not used (zero)
```

The ATF data is sent by VRAM-Transfer (4 KBytes).

```
 000-FD1  Data for ATF0 through ATF44 (4050 bytes)
 FD2-FFF  Not used
```

Each ATF consists of 90 bytes, that are 5 bytes (20×2 bits) for each of
the 18 character lines of the Game Boy window. The two most significant
bits of the first byte define the color attribute (0-3) for the first
character of the first line, the next two bits the next character, and
so on.

## SGB Command $16 — ATTR_SET

Used to transfer attributes from Attribute File (ATF) to Game Boy window.

```
 Byte  Content
 0     Command*8+Length (fixed length=1)
 1     Attribute File Number ($00-$2C), Bit 6=Cancel Mask
 2-F   Not used (zero)
```

When above Bit 6 is set, the Game Boy screen becomes re-enabled after the
transfer (in case it has been disabled/frozen by MASK_EN command).
Note: The same functions may be (optionally) also included in PAL_SET
commands, as described in the chapter about Color Palette Commands.
