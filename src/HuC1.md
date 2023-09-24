# HuC1

HuC1 is an MBC developed by Hudson Soft. It implements ROM and RAM
banking, and also provides infrared communication.
Despite many sources on the internet claiming that HuC1 is “similar to MBC1”, it actually differs from MBC1 significantly.

## Memory Map

Address range | Feature
--------------|------------------------------------
  $0000–1FFF  | RAM/IR select (when writing only)
  $2000–3FFF  | ROM bank select (when writing only)
  $4000–5FFF  | RAM bank select (when writing only)
  $6000–7FFF  | Nothing?
  $A000–BFFF  | Cart RAM or IR register

### 0000–1FFF — IR Select \[write-only\]

Most MBCs can disable the cartridge RAM to prevent accidental writes.
HuC1 doesn’t do this. Instead, this register swtiches the $A000–BFFF
region between “RAM mode” and “IR mode” (described below). Write $0E to
switch to IR mode, or anything else to switch to RAM mode.

Some HuC1 games still write $0A and $00 to this region as if it would
enable/disable cart RAM.

### 2000–3FFF — ROM Bank Number \[write-only\]

HuC1 can accept a bank number of at least 6 bits here.

### 4000–5FFF — RAM Bank Select \[write-only\]

HuC1 can accept a bank number of at least 2 bits here.

### 6000–7FFF — Nothing? \[write-only\]

Writes to this region seem to have no effect. Even so, some games do
write to this region, as if it had the same effect as on MBC1. You may
safely ignore these writes.

### A000–BFFF — Cart RAM or IR register \[read/write\]

When in “IR mode” (wrote $0E to $0000), the IR register is visible
here. Write to this region to control the IR transmitter. $01 turns it
on, $00 turns it off. Read from this region to see either $C1 (saw
light) or $C0 (did not see light).

When in “RAM mode” (wrote something other than $0E to $0000) this region
behaves like normal cart RAM.

## External links

- [Source on jrra.zone](http://jrra.zone/blog/huc1.html)
