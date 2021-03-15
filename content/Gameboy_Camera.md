# Game Boy Camera

::: tip SOURCE
This section was originally compiled by Antonio Niño Díaz during his work on reverse engineering the Game Boy Camera. The upstream source can be found [here](https://github.com/AntonioND/gbcam-rev-engineer)
:::

## Camera Cartridge

The Game Boy Camera cartridge contains 4 ICs: the usual ROM and RAM ICs, a big controller IC (like a MBC) and a sensor (M64282FP "retina" chip).

The main board contains all ICs except from the sensor.

| Component# | Part#/inscription                    | Description          |
|------------|--------------------------------------|----------------------|
| U1         | MAC-GBD Nintendo 9807 SA             | I/O, memory control. |
| U2         | GBD-PCAX-0 F M538011-E - 08 8145507  | 1MB ROM              |
| U3         | 52CV1000SF85LL SHARP JAPAN 9805 5 0A | 128KB RAM            |

The U1 is the only one connected to the GB cartridge pins (besides some of the address pins of the ROM IC). The U2 and U3 (ROM and RAM) are connected to U1. The M64282FP "retina" chip is in a separate PCB, and is connected to the U1.
The M64282FP handles most of the configuration of the capturing process. The U1 transforms the commands from the Game Boy CPU into the correct signals needed for the M64282FP. The detailed timings are described below.
It is a good idea to have the datasheet of the M64282FP, but it is very poorly explained, so this document will try to explain everything about it (except from limits like voltage or signal timings). There are datasheets of similar sensors (M64283FP and M64285FP) that can be very useful to understand some things about the sensor of the GB Camera.

## Game Boy Camera MBC

The Game Boy Camera controller works pretty much the same as a MBC3.

#### 0000-3FFF - ROM Bank 00 (Read Only)

First 16 KB of the ROM.

#### 4000-7FFF - ROM Bank 01-3F (Read Only)

This area may contain any ROM bank (0 included). The initial mapped bank is 01.

<!-- #### A000-BFFF - RAM Bank 00-0F (Read/Write) -->

#### A000-BFFF - CAM Registers (Read/Write)

Depending on the current RAM Bank Number, this memory space is used to access the cartridge RAM or the CAM registers. RAM can only be read if the capture unit is not working, it returns 00h otherwise.

#### 0000-1FFF - RAM Enable (Write Only)

A value of 0Ah will enable writing to RAM, 00h will disable it. Reading from RAM or registers is always enabled. Writing to registers is always enabled. Disabled on reset.

#### 2000-3FFF - ROM Bank Number (Write Only)

Writing a value of 00-3Fh selects the corresponding ROM Bank for area 4000-7FFF.

#### 4000-5FFF - RAM Bank Number/CAM Registers Select (Write Only)

Writing a value in range for 00h-0Fh maps the corresponding external RAM Bank to memory at A000-BFFF. Writing any value with bit 5 set to '1' will select CAM registers. Usually bank 10h is used to select the registers. All registers are mirrored every 80h bytes. RAM bank 0 selected on reset.

::: tip NOTE
Unlike most games, the GB Camera RAM can only be written when PHI pin = '1'. It's an enable signal for the RAM chip. Most cartridge readers and writers can't handle PHI pin so they can't restore a saved backup. It isn't needed to change ROM banks.
:::

## I/O Registers

The Game Boy Camera I/O registers are mapped to all banks with bit 4 set to '1'. The GB Camera ROM usually changes to bank 16 to use the registers.

There are 3 groups of registers:
- The first group is composed by the trigger register A000. This register starts the capture process and returns the current status (working/capture finished).
- The second group is composed by registers A001-A005, used to configure most parameters of the M64282FP sensor.
- The third group is composed by 48 registers that form a 4×4 matrix. Each element of the matrix is formed by 3 bytes. This matrix is used by the controller for contrast and dithering.

All registers are write-only, except the register A000. The others return 00h when read. The initial values of all registers on reset is 00h.

### Register A000

The lower 3 bits of this register can be read and write. The other bits return '0'. Writing any value with bit 0 set to '1' will start the capturing process. Any write with bit 0 set to '0' is a normal write and won't trigger the capture. The value of bits 1 and 2 affects the value written to registers 4, 5 and 6 of the M64282FP, which are used in 1-D filtering mode (effects described in following chapters).
Bit 0 of this register is also used to verify if the capturing process is finished. It returns '1' when the hardware is working and '0' if the capturing process is over.
When the capture process is active all RAM banks will return 00h when read (and writes are ignored), but the register A000 can still be read to know when the transfer is finished.
The capturing process can be stopped by writing a '0' to bit 0. When a '1' is written again it will continue the previous capture process with the old capture parameters, even if the registers are changed in between. If the process is stopped RAM can be read again.

### Register A001

This register is mapped to register 1 of M64282FP. It controls the output gain and the edge operation mode.

### Register A002, A003

This registers are mapped to registers 2 and 3 of M64282FP. They control the exposure time. Register 2 is the MSB, register 3 is the LSB.

```
u16 exposure_steps = [A003] | ([A002]<<8);
```

### Register A004

This register is mapped to register 7 of M64282FP. It sets the output voltage reference, the edge enhancement ratio and it can invert the image.

### Register A005

This register is mapped to register 0 of M64282FP. It sets the output reference voltage and enables the zero point calibration.

### Register A006-A035

Those registers form a 4×4 matrix with 3 bytes per element. They handle dithering and contrast, and they are sorted by rows:

<table class="tg">
<thead>
  <tr>
    <th class="tg-0pky"></th>
    <th class="tg-0pky" colspan="4">X</th>
  </tr>
</thead>
<tbody>
  <tr>
    <td class="tg-0pky" rowspan="4">Y</td>
    <td class="tg-0pky">00</td>
    <td class="tg-0pky">10</td>
    <td class="tg-0lax">20</td>
    <td class="tg-0lax">30</td>
  </tr>
  <tr>
    <td class="tg-0pky">01</td>
    <td class="tg-0pky">11</td>
    <td class="tg-0lax">21</td>
    <td class="tg-0lax">31</td>
  </tr>
  <tr>
    <td class="tg-0pky">02</td>
    <td class="tg-0pky">12</td>
    <td class="tg-0lax">23</td>
    <td class="tg-0lax">33</td>
  </tr>
  <tr>
    <td class="tg-0lax">03</td>
    <td class="tg-0lax">13</td>
    <td class="tg-0lax">23</td>
    <td class="tg-0lax">33</td>
  </tr>
</tbody>
</table>
