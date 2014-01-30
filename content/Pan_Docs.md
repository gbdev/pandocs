Table of Contents
-----------------

### Overview

[Game Boy Technical Data](Game_Boy_Technical_Data "wikilink") - [Memory
Map](Memory_Map "wikilink")

### I/O Ports

[Video Display](Video_Display "wikilink") - [Sound
Controller](Sound_Controller "wikilink") - [Joypad
Input](Joypad_Input "wikilink") - [Serial Data Transfer (Link
Cable)](Serial_Data_Transfer_(Link_Cable) "wikilink") - [Timer and
Divider Registers](Timer_and_Divider_Registers "wikilink") -
[Interrupts](Interrupts "wikilink") - [CGB
Registers](CGB_Registers "wikilink") - [SGB
Functions](SGB_Functions "wikilink")

### CPU Specifications

[CPU Registers and Flags](CPU_Registers_and_Flags "wikilink") - [CPU
Instruction Set](CPU_Instruction_Set "wikilink") - [CPU Comparision with
Z80](CPU_Comparision_with_Z80 "wikilink")

### Cartridges

[The Cartridge Header](The_Cartridge_Header "wikilink") - [Memory Bank
Controllers](Memory_Bank_Controllers "wikilink") - [Gamegenie/Shark
Cheats](Gamegenie/Shark_Cheats "wikilink")

### Memory Bank Controllers

[MBC1](MBC1 "wikilink") - [MBC2](MBC2 "wikilink") -
[MBC3](MBC3 "wikilink") - [MBC5](MBC5 "wikilink")

### Other

[Power Up Sequence](Power_Up_Sequence "wikilink") - [Reducing Power
Consumption](Reducing_Power_Consumption "wikilink") - [Sprite RAM
Bug](Sprite_RAM_Bug "wikilink") - [External
Connectors](External_Connectors "wikilink")

Excerpt from Martin\'s document
-------------------------------

     =================================================================
           Everything You Always Wanted To Know About GAMEBOY *
     =================================================================

                         * but were afraid to ask

            Pan of -ATX- Document Updated by contributions from:
         Marat Fayzullin, Pascal Felber, Paul Robson, Martin Korth
                 CPU, SGB, CGB, AUX specs by Martin Korth

                      Last updated 10/2001 by nocash
                   Previously updated 4-Mar-98 by kOOPa

### Foreword

The following was typed up for informational purposes regarding the
inner workings on the hand-held game machine known as GameBoy,
manufactured and designed by Nintendo Co., LTD. This info is presented
to inform a user on how their Game Boy works and what makes it \"tick\".
GameBoy is copyrighted by Nintendo Co., LTD. Any reference to
copyrighted material is not presented for monetary gain, but for
educational purposes and higher learning.

### Available Document Formats

The present version of this document is available at
<http://nocash.emubase.de/pandocs.htm>

Also, a copy of this document is included in the manual of newer
versions of the [no\$gmb](no$gmb "wikilink") debugger, because of recent
piracy attacks (many thanks and best wishes go to hell) I have currently
no intention to publish any such or further no\$gmb updates though.

Pan Docs Wikification
---------------------

### Mission

Pan Docs is the single most comprehensive technical reference to Gameboy
that is available to the public. It was originally written by Pan of
Anthrox, and was later maintained by Martin Korth, also known for
[no\$gmb](no$gmb "wikilink"). As Pandocs is an important resource, it is
a priority to add it to the wiki.

The addition of information from the document is done under the
following premises:

-   It is thought that the information in the document was meant to be
    public domain by its authors.
-   The document is no longer actively maintained by its original
    authors.
-   Adding the information to a wiki allows people to correct or add
    information if needed.
-   That the information can be presented in such a way that it is more
    appealing.

Update as of 1/30/2014 at 3:41 P.M.

I have provided a link to Pan\'s old webpage using the wayback machine.
Here you can download some of his old tools that are so hard to find. I
might provide a tutorial or two after I get enough time.

<http://web.archive.org/web/19990209030645/http://www.anthrox.com/>

Please click on free software to download the tools. We hold no
liability for the tools that you download.

### Ways to help

-   Add information from Pan Docs. (To do: Formatting guidelines)
-   Convert ASCII art into images or tables, or other formatting as
    appropriate.
-   Condense information! Create a parallel article where the most
    important parts of a Pandocs article are condensed.

