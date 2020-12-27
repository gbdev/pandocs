The camera chip
---------------

The [Game Boy Camera](Gameboy_Camera "wikilink") is using the [Mitsubishi
M64282FP](Mitsubishi_M64282FP "wikilink"), a.k.a. the \"Artificial
Retina\" which is a 128\*128 pixel grayscale CMOS image sensor that can
perform image adjustments in realtime. The data sheet is available in
[**/docs/misc/** of the file hub](http://gbdev.gg8.se/files/docs/misc/).

Documentation related to the GB Camera itself (not only the sensor) can
be found in the following github repository:
<https://github.com/AntonioND/gbcam-rev-engineer>

Integrated circuits on the main board
-------------------------------------

+-------------+----------------------+----------------------+
| Component\# | Part\# / inscription | Description          |
+=============+======================+======================+
| U1          | MAC-GBD\             | I/O, memory control? |
|             | Nintendo\            |                      |
|             | 9807 SA              |                      |
+-------------+----------------------+----------------------+
| U2          | GBD-PCAX-0 F\        | 1M X 8 ROM           |
|             | M538011-E -08\       |                      |
|             | 8145507              |                      |
+-------------+----------------------+----------------------+
| U3          | 52CV1000SF85LL\      | 128K X 8 RAM         |
|             | SHARP JAPAN\         |                      |
|             | 9805 5 0A            |                      |
+-------------+----------------------+----------------------+

Pictures of the innards
-----------------------

