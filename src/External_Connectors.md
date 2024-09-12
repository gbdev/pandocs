# External Connectors

## Cartridge Slot

 Pin  | Name   | Explanation
------|--------|--------------
  1   | VDD    | Power Supply +5V DC
  2   | PHI    | System Clock
  3   | /WR    | Write
  4   | /RD    | Read
  5   | /CS    | Chip Select
 6-21 | A0-A15 | Address Lines
22-29 | D0-D7  | Data Lines
  30  | /RES   | Reset signal
  31  | VIN    | External Sound Input
  32  | GND    | Ground

## Link Port

Pin numbers are arranged as 2,4,6 in upper row, 1,3,5 in lower row;
outside view of Game Boy socket; flat side of socket upside. Colors as
used in most or all standard link cables, because SIN and SOUT are
crossed, colors Red and Orange are exchanged at one cable end.

Pin | Name | Color  | Explanation
----|------|--------|-------------
  1 | VCC  | -      | +5V DC
  2 | SOUT | red    | Data Out
  3 | SIN  | orange | Data In
  4 | P14  | -      | Not used
  5 | SCK  | green  | Shift Clock
  6 | GND  | blue   | Ground

Note: The original Game Boy used larger plugs than Game Boy Pocket and
newer. Linking between older/newer Game Boy systems is possible by using cables
with one large and one small plug though.

## Stereo Sound Connector (3.5mm, female)

 Pin    | Explanation
--------|-----------
 Tip    | Sound Left
 Middle | Sound Right
 Base   | Ground

## External Power Supply

The Game Boy can be powered by an external supply through a coaxial (or *barrel*) power connector.
The appropriate connector and supply depends on the model.
The table below describes the physical connector (barrel outer and inner diameter) and the electrical requirements for each model:

Model | Outer Dia. | Inner Dia. | Voltage |  Load | Centre Polarity
------|------------|------------|---------|-------|-------------
  DMG |     3.5 mm |    1.35 mm |  6 V DC | 0.7 W | Negative (-)
  MGB |    2.35 mm |     0.7 mm |  3 V DC | 0.7 W | Positive (+)
  CGB |    2.35 mm |     0.7 mm |  3 V DC | 0.6 W | Positive (+)

- Polarity is listed as the sign of the inner/centre contact. The outer contact is always the opposite sign.
- The CGB and DMG specify the voltage and polarity on the case, near the connector. The MGB lists the voltage but not the polarity.
- The dimensions of the MGB/CGB connector match a Japanese standard connector (JEITA RC-5320A JSAP1, formerly EIAJ-01).
- The listed load in watts is taken from the back label of each model.
