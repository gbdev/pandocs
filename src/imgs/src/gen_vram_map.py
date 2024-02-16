#!/usr/bin/env python3
import sys

# Configurable sizes and positions.
TILES_TOP_Y  = 20
TILES_LEFT_X = 100
TILE_SIZE = 12
GAP_BETWEEN_BANKS = 96

# Some (constant) values derived from the above.
TILE_ID_X = TILES_LEFT_X + (TILE_SIZE * 16 + GAP_BETWEEN_BANKS // 2)
TILE_ID_Y = TILES_TOP_Y + TILE_SIZE * 12
TILEMAP_ENTRY_SIZE = TILE_SIZE // 2 # Because each tilemap row contains 32 entries, not 16.
TILEMAP_TOP_Y = TILES_TOP_Y + TILE_SIZE * 24
TILEMAP_ENTRY_COORDS = TILEMAP_TOP_Y + TILEMAP_ENTRY_SIZE * 32

def bank_left_x(bank):
	return TILES_LEFT_X + (TILE_SIZE * 16 + GAP_BETWEEN_BANKS) * bank

with open(sys.argv[1], "wt") as svg:
	def emit(string):
		print(string, file=svg)

	emit(f"""<!-- Do not put any blank lines, or Markdown processing will break! -->
<svg viewBox="0 0 585 840" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">
	<defs>
		<style type="text/css"><![CDATA[
			text {{ fill: var(--fg, #000); dominant-baseline: middle; font-feature-settings: "tnum"; }}
			.centered {{ text-anchor: middle; }}
			.right    {{ text-anchor: end; }}
			rect, polyline, line, circle {{ stroke: var(--fg, #000); fill: var(--bg, #fff); }}
			#arrow-head                  {{ stroke: none;            fill: var(--fg, #000); }}
			.block  {{ stroke-width: 4; }}
			.hover + * {{ display: none; }}  .hover:hover + * {{ display: initial; }}
			rect.hover:hover {{ fill: var(--fg, #000); }}
		]]></style>
		<polygon points="0,0 8,-4 8,4" id="arrow-head"/>
	</defs>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 0}" class="right">0x8000</text>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 1}" class="right">0x8800</text>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 2}" class="right">0x9000</text>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 3}" class="right">0x9800</text>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 5}" class="right">0x9C00</text>
	<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + TILE_SIZE * 8 * 7}" class="right">0x9FFF</text>
	<text x="{TILE_ID_X}" y="{TILE_ID_Y - 10}" class="centered">Tile ID:</text>""")

	for bank in range(2):
		emit(f'	<text x="{bank_left_x(bank) + TILE_SIZE * 8}" y="{TILES_TOP_Y - 10}" class="centered">Bank {bank}</text>')

		for block in range(3):
			emit(f'<rect x="{bank_left_x(bank)}" y="{TILES_TOP_Y + block * 8 * TILE_SIZE}" width="{TILE_SIZE * 16}" height="{TILE_SIZE * 8}" class="block"/>')
			emit('<g class="hover">')
			for block_row in range(8):
				row = block * 8 + block_row
				y = TILES_TOP_Y + row * TILE_SIZE

				emit(f"<!-- ${row:x}x -->")
				for tile_x in range(16):
					x = bank_left_x(bank) + tile_x * TILE_SIZE
					emit(f'<rect x="{x}" y="{y}" width="{TILE_SIZE}" height="{TILE_SIZE}" class="hover"/><text x="{TILE_ID_X}" y="{TILE_ID_Y + 10}" class="centered">${(row * 16 + tile_x) % 256:02X}</text>')
			emit('</g>')
			emit(f'<text x="{TILES_LEFT_X - 10}" y="{TILES_TOP_Y + block * 8 * TILE_SIZE + TILE_SIZE * 4}" class="right">Tile block {block}</text>')

	emit(f"""	<text x="{TILE_ID_X - 2}" y="{TILEMAP_ENTRY_COORDS - 30}" class="right">X:</text>
	<text x="{TILE_ID_X - 2}" y="{TILEMAP_ENTRY_COORDS - 10}" class="right">Y:</text>""")

	for bank in range(2):
		for tmap in range(2):
			emit(f'<rect x="{bank_left_x(bank)}" y="{TILEMAP_TOP_Y + TILEMAP_ENTRY_SIZE * 32 * tmap}" width="{TILEMAP_ENTRY_SIZE * 32}" height="{TILEMAP_ENTRY_SIZE * 32}" class="block"/>')
			emit('<g class="hover">')
			for row in range(32):
				y = TILEMAP_TOP_Y + TILEMAP_ENTRY_SIZE * (row + 32 * tmap)
				emit(f'<!-- Y = {row} -->')
				for entry_x in range(32):
					x = bank_left_x(bank) + entry_x * TILEMAP_ENTRY_SIZE
					emit(f'<rect x="{x}" y="{y}" width="{TILEMAP_ENTRY_SIZE}" height="{TILEMAP_ENTRY_SIZE}" class="hover"/><g><text x="{TILE_ID_X + 2}" y="{TILEMAP_ENTRY_COORDS - 30}">{entry_x}</text><text x="{TILE_ID_X + 2}" y="{TILEMAP_ENTRY_COORDS - 10}">{row}</text><text x="{TILE_ID_X + 2}" y="{TILEMAP_ENTRY_COORDS + 10}" class="centered">${entry_x + (row + tmap * 32) * 32 + 0x9800:4X}</text></g>')
			emit('</g>')
			emit(f'<text x="{TILES_LEFT_X - 10}" y="{TILEMAP_TOP_Y + TILEMAP_ENTRY_SIZE * (16 + 32 * tmap)}" class="right">{"Tilemap" if bank == 0 else "Attrmap"} {tmap}</text>')

	emit('</svg>')
