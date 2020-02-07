HuC1
----

HuC1 is an MBC used by some Game Boy games which besides doing the usual
MBC stuff, also provides IR comms. A lot of sources on the internet said
that HuC1 was \"similar to MBC1\", but they didn\'t provide any detail.
I took a look, and it turns out that HuC1 differs from MBC1 quite a lot.

Memory Map
----------

`   0000-1FFF   IR select`
`   2000-3FFF   ROM bank select`
`   4000-5FFF   RAM bank select`
`   6000-7FFF   Nothing?`
`   --`
`   A000-BFFF   Cart RAM or IR register`

0000-1FFF IR Select (Write Only)
--------------------------------

Most MBCs let you disable the cartridge RAM to prevent accidental
writes. HuC1 doesn\'t do this, instead you use this register to switch
the A000-BFFF region between \"RAM mode\" and \"IR mode\" (described
below). Write 0x0E to switch to IR mode, and anything else to switch to
RAM mode. Nevertheless some HuC1 games attempt to write 0x0A and 0x00 to
this region as if it would enable/disable cart RAM.

2000-3FFF ROM Bank Number (Write Only)
--------------------------------------

HuC1 can accept a bank number of at least 6 bits here.

4000-5FFF RAM Bank Select (Write Only)
--------------------------------------

HuC1 can accept a bank number of at least 2 bits here.

6000-7FFF Nothing? (Write Only)
-------------------------------

Writes to this region seem to have no effect. Even so, some games do
write to this region, as if it had the same effect as on MBC1. You may
safely ignore these writes.

A000-BFFF Cart RAM or IR register (Read/Write)
----------------------------------------------

When in \"IR mode\" (wrote 0x0E to 0x0000), the IR register is visible
here. Write to this region to control the IR transmitter. 0x01 turns it
on, 0x00 turns it off. Read from this region to see either 0xC1 (saw
light) or 0xC0 (did not see light). When in \"RAM mode\" (wrote
something other than 0x0E to 0x000) this region behaves like normal cart
RAM.

External links
--------------

-   [Source on jrra.zone](http://jrra.zone/blog/huc1.html)

