"use strict";

/*
 * Use of `on<event>` properties is normally discouraged, however they *have to* be used here.
 *
 * For the sake of maintainability and the SVG's file size, the tilemaps are made of nested `<use>`
 * elements. However, those elements have a "closed root", thus the "inside" elements are *not*
 * exposed to anything outside of them.
 *
 * I have tried putting `<script>` blocks inside of the "template" element, but they appear to only
 * run on the template, not within any of its clones.
 * The only remaining thing is the `on*` attributes, which *do* appear to be cloned!
 */

const toHex = (num, nbDigits) => num.toString(16).toUpperCase().padStart(nbDigits, '0');
const svg = document.getElementById("vram_map");

// Since the hover events are dispatched to several separate handlers, we need something to
// centralize all of their information.
let tileInfo = { x: null, y: null, baseAddr: null };
const updateTileID = () => {
	// Do nothing if all of the handlers haven't fired at least once yet.
	// Check all three properties to avoid relying on the order in which the events are triggered.
	if (tileInfo.x === null || tileInfo.y === null || tileInfo.baseAddr === null) {
		return;
	}

	const tileID = tileInfo.y * 16 + tileInfo.x + (tileInfo.baseAddr >> 4 & 0x80);
	svg.getElementById("tile_id").textContent = `$${toHex(tileID, 2)}`;
	const tileAddr = tileInfo.baseAddr + (tileInfo.y * 16 + tileInfo.x) * 16;
	svg.getElementById("tile_addr").textContent = `${toHex(tileAddr, 4)}–${toHex(tileAddr + 15, 4)}`;
};

// These are event handlers, called into by `onmouseenter` attributes within `vram_map.svg`.
// Have each hover event attempt to recalc the tile ID; this will cause up to 2 unnecessary recalcs,
// but avoids relying on the order upon which the events are fired.
const tile  = x    => { tileInfo.x = x;           updateTileID(); };
const row   = y    => { tileInfo.y = y;           updateTileID(); };
const block = addr => { tileInfo.baseAddr = addr; updateTileID(); };


// Since the hover events are dispatched to several separate handlers, we need something to
// centralize all of their information.
let mapInfo = { x: null, y: null, baseAddr: null, bank: null };
const updateMapCoords = () => {
	// Do nothing if all of the handlers haven't fired at least once yet.
	// Check all three properties to avoid relying on the order in which the events are triggered.
	// (The bank is always set at the same time as the address.)
	if (mapInfo.x === null || mapInfo.y === null || mapInfo.baseAddr === null) {
		return;
	}

	svg.getElementById("map_what").textContent = mapInfo.bank == 0 ? "Tile ID for" : "Attribute for";
	svg.getElementById("map_x").textContent = mapInfo.x;
	svg.getElementById("map_y").textContent = mapInfo.y;
	const mapAddr = mapInfo.baseAddr + mapInfo.x + mapInfo.y * 32;
	svg.getElementById("map_addr").textContent = `($${toHex(mapAddr, 4)})`;
};

// These are event handlers, called into by `onmouseenter` attributes within `vram_map.svg`.
// Have each hover event attempt to recalc the tile ID; this will cause up to 2 unnecessary recalcs,
// but avoids relying on the order upon which the events are fired.
const mapX = x            => { mapInfo.x = x;                                updateMapCoords(); };
const mapY = y            => { mapInfo.y = y;                                updateMapCoords(); };
const tmap = (addr, bank) => { mapInfo.baseAddr = addr; mapInfo.bank = bank; updateMapCoords(); };
