MBC2 (max 256KByte ROM and 512x4 bits RAM)
------------------------------------------

### 0000-3FFF - ROM Bank 00 (Read Only)

Same as for MBC1.

### 4000-7FFF - ROM Bank 01-0F (Read Only)

Same as for MBC1, but only a total of 16 ROM banks is supported.

### A000-A1FF - 512x4bits RAM, built-in into the MBC2 chip (Read/Write)

The MBC2 doesn\'t support external RAM, instead it includes 512x4 bits
of built-in RAM (in the MBC2 chip itself). It still requires an external
battery to save data during power-off though. As the data consists of
4bit values, only the lower 4 bits of the \"bytes\" in this memory area
are used.

### 0000-1FFF - RAM Enable (Write Only)

The least significant bit of the upper address byte must be zero to
enable/disable cart RAM. For example the following addresses can be used
to enable/disable cart RAM: 0000-00FF, 0200-02FF, 0400-04FF, \...,
1E00-1EFF. The suggested address range to use for MBC2 ram
enable/disable is 0000-00FF.

### 2000-3FFF - ROM Bank Number (Write Only)

Writing a value (XXXXBBBB - X = Don\'t cares, B = bank select bits) into
2000-3FFF area will select an appropriate ROM bank at 4000-7FFF.

The least significant bit of the upper address byte must be one to
select a ROM bank. For example the following addresses can be used to
select a ROM bank: 2100-21FF, 2300-23FF, 2500-25FF, \..., 3F00-3FFF. The
suggested address range to use for MBC2 rom bank selection is 2100-21FF.

