# Palette Commands

## SGB Command $00 — PAL01

Transmit color data for SGB palette 0, color 0-3, and for SGB palette 1,
color 1-3 (without separate color 0).

{{#bits 16 <
        "" 0:"Header"
           1-2:"Pals 0 & 1 color #0"
            3-4:"Pal 0 color #1"   5-6:"Pal 0 color #2"   7-8:"Pal 0 color #3"
           9-10:"Pal 1 color #1" 11-12:"Pal 1 color #2" 13-14:"Pal 1 color #3"
}}

The **header** byte is `$00 << 3 | $01` = $01.

## SGB Command $01 — PAL23

Same as `PAL01` above, but for Palettes 2 and 3 respectively.
The **header** byte is thus $09.

## SGB Command $02 — PAL03

Same as `PAL01` above, but for Palettes 0 and 3 respectively.
The **header** byte is thus $11.

## SGB Command $03 — PAL12

Same as `PAL01` above, but for Palettes 1 and 2 respectively.
The **header** byte is thus $19.

## SGB Command $0A — PAL_SET

Used to copy pre-defined palette data from SGB system color palettes to
actual SNES palettes.

:::warning

Before using this feature, System Palette data should be initialized by
[`PAL_TRN`](<#SGB Command $0B — PAL_TRN>) command, and (when used) Attribute File (ATF) data should be
initialized by [`ATTR_TRN`](<#SGB Command $15 — ATTR_TRN>).

:::

{{#bits 16 <
        "" 0:"Header"
           1-2:"Palette #0's ID"
           3-4:"Palette #1's ID"
           5-6:"Palette #2's ID"
           7-8:"Palette #3's ID"
           9:"Flags"
}}

The **header** byte is `$0A << 3 | $01` = $51.
All **palette ID**s are little-endian.

{{#bits 8 >
        "Flags" 7:"Apply ATF" 6:"Cancel MASK_EN" 5-0:"ATF number"
}}

- **Apply ATF**: If and only if this is set, then the ATF whose ID is specified by bits 0–5 is applied as if by [`ATTR_SET`](<#SGB Command $16 — ATTR_SET>).
- **Cancel `MASK_EN`**: If this bit is set, then any current [`MASK_EN`](<#SGB Command $17 — MASK_EN>) "screen freeze" is cancelled.
- **ATF number**: Index of the ATF to transfer. Values greater than $2C are invalid.

## SGB Command $0B — PAL_TRN

Used to initialize SGB system color palettes in SNES RAM. System color
palette memory contains 512 pre-defined palettes, these palettes do not
directly affect the display, however, the `PAL_SET` command may be later
used to transfer four of these "logical" palettes to actual visible
"physical" SGB palettes. Also, the `OBJ_TRN` feature will use groups
of 4 System Color Palettes (4\*4 colors) for SNES OBJ palettes (16
colors).

{{#bits 16 <
        "" 0:"Header"
}}

The **header** byte must be $59.

The palette data is sent by [VRAM Transfer](<#VRAM Transfers>).

<div class=table-wrapper><table class=bit-descrs><thead><tr>
        <th></th><th>…0</th><th>…1</th><th>…2</th><th>…3</th><th>…4</th><th>…5</th><th>…6</th><th>…7</th><th>…8</th><th>…9</th><th>…A</th><th>…B</th><th>…C</th><th>…D</th><th>…E</th><th>…F</th>
</tr></thead><tbody><tr>
        <td>$800…</td><td colspan=2>Pal #0 color #0</td><td colspan=2>Pal #0 color #1</td><td colspan=2>Pal #0 color #2</td><td colspan=2>Pal #0 color #3</td><td colspan=2>Pal #1 color #0</td><td colspan=2>Pal #1 color #1</td><td colspan=2>Pal #1 color #2</td><td colspan=2>Pal #1 color #3</td>
</tr><tr>
        <td>$801…</td><td colspan=2>Pal #2 color #0</td><td colspan=2>Pal #2 color #1</td><td colspan=2>Pal #2 color #2</td><td colspan=2>Pal #2 color #3</td><td colspan=2>Pal #3 color #0</td><td colspan=2>Pal #3 color #1</td><td colspan=2>Pal #3 color #2</td><td colspan=2>Pal #3 color #3</td>
</tr><tr>
        <td>…</td><td colspan=16>…</td>
</tr><tr>
        <td>$8FF…</td><td colspan=2>Pal #510 color #0</td><td colspan=2>Pal #510 color #1</td><td colspan=2>Pal #510 color #2</td><td colspan=2>Pal #510 color #3</td><td colspan=2>Pal #511 color #0</td><td colspan=2>Pal #511 color #1</td><td colspan=2>Pal #511 color #2</td><td colspan=2>Pal #511 color #3</td>
</tr></tbody></table></div>

:::tip

The data is stored at 3000-3FFF in SNES memory.

:::

## SGB Command $19 — PAL_PRI

If the player overrides the active palette set (a pre-defined or the custom one), it stays in effect until the smiley face is selected again, or the player presses the X button on their SNES controller.

However, if `PAL_PRI` is enabled, then changing the palette set (via any `PAL_*` command besides `PAL_TRN`) will switch back to the game's newly-modified palette set, if it wasn't already active.

_Donkey Kong_ (1994) is one known game that appears to use this.

{{#bits 16 <
        "" 0:"Header" 1:"Priority"
}}

The **header** must be $C9.

Bit 0 of the **priority** byte enables (`1`) or disables (`0`) `PAL_PRI`.
