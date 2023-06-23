# OAM Corruption Bug

There is a flaw in the Game Boy hardware that causes rubbish data to be written
to object attribute memory (OAM) if the following instructions are used while their 16-bit content
(before the operation) is in the range $FE00&ndash;$FEFF and the PPU is in mode 2:

```rgbasm
 inc rr         dec rr       ; rr = bc, de, or hl
 ld a, [hli]    ld a, [hld]
 ld [hli], a    ld [hld], a
```

Objects 0 and 1 ($FE00 & $FE04) are not affected by this bug.

Game Boy Color and Advance are not affected by this bug, even when
running monochrome software.

## Accurate Description

The OAM Corruption Bug (or OAM Bug) actually consists of two different bugs:

- Attempting to read or write from OAM (Including the $FEA0-$FEFF
  region) while the PPU is in mode 2 (OAM scan) will corrupt it.
- Performing an increase or decrease operation on any 16-bit register
  (BC, DE, HL, SP or PC) while that register is in the OAM range
  ($FE00–$FEFF) will trigger an access to OAM, causing a corruption.
  This happens because the CPU's increment and decrement unit (IDU)
  for 16-bit numbers is directly tied to the address bus.
  During IDU operation, the value is output as an address,
  even if a read or write is not asserted.

## Affected Operations

The following operations are affected by this bug:

- Any memory access instruction, if it accesses OAM
- `inc rr`, `dec rr` - if `rr` is a 16-bit register pointing to OAM,
  it will trigger a write and corrupt OAM
- `ld [hli], a`, `ld [hld], a`, `ld a, [hli]`, `ld a, [hld]`- these
  will trigger a corruption twice if `hl` points to OAM; once for the
  usual memory access, and once for the extra write triggered by the
  `inc`/`dec`
- `pop rr`, the `ret` family - For some reason, `pop` will trigger the
  bug only 3 times (instead of the expected 4 times); one read, one
  glitched write, and another read without a glitched write. This also
  applies to the `ret` instructions.
- `push rr`, the `call` family, `rst xx` and interrupt handling -
  Pushing to the stack will trigger the bug 4 times; two usual writes
  and two glitched writes caused by the implied `dec sp`. However, since one
  glitched write occurs in the same cycle as a actual write, this will
  effectively behave like 3 writes.
- Executing code from OAM - If PC is inside OAM (reading $FF,
  that is, `rst $38`) the bug will trigger twice, once for increasing PC
  inside OAM (triggering a write), and once for reading from OAM. If a
  multi-byte opcode is executed from $FDFF or $FDFE, and bug will
  similarly trigger twice for every read from OAM.

## Corruption Patterns

The OAM is split into 20 rows of 8 bytes each, and during mode 2 the PPU
reads those rows consecutively; one every 1 M-cycle. The operations
patterns rely on type of operation (read/write/both) used on OAM during
that M-cycle, as well as the row currently accessed by the PPU. The
actual read/write address used, or the written value have no effect.
Additionally, keep in mind that OAM uses a 16-bit data bus, so all
operations are on 16-bit words.

### Write Corruption

A "write corruption" corrupts the currently access row in the following
manner, as long as it's not the first row (containing the first two
objects):

- The first word in the row is replaced with this bitwise expression:
  `((a ^ c) & (b ^ c)) ^ c`, where `a` is the original value of that
  word, `b` is the first word in the preceding row, and `c` is the
  third word in the preceding row.
- The last three words are copied from the last three words in the
  preceding row.

### Read Corruption

A "read corruption" works similarly to a write corruption, except the
bitwise expression is `b | (a & c)`.

### Write During Increase/Decrease

If a register is increased or decreased in the same M-cycle of a write,
this will effectively trigger two writes in a single M-cycle. However,
this case behaves just like a single write.

### Read During Increase/Decrease

If a register is increased or decreased in the same M-cycle of a write,
this will effectively trigger both a read **and** a write in a single
M-cycle, resulting in a more complex corruption pattern:

- This corruption will not happen if the accessed row is one of the
  first four, as well as if it's the last row:
  - The first word in the row preceding the currently accessed row
    is replaced with the following bitwise expression:
    `(b & (a | c | d)) | (a & c & d)` where `a` is the first word
    two rows before the currently accessed row, `b` is the first
    word in the preceding row (the word being corrupted), `c` is the
    first word in the currently accessed row, and `d` is the third
    word in the preceding row.
  - The contents of the preceding row is copied (after the
    corruption of the first word in it) both to the currently
    accessed row and to two rows before the currently accessed row
- Regardless of whether the previous corruption occurred or not, a
  normal read corruption is then applied.
