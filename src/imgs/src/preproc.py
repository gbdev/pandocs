#!/usr/bin/env python3
import json
import pathlib
import sys

from graph_render import gen_graph

if len(sys.argv) == 3 and sys.argv[1] == "supports":
	sys.exit(sys.argv[2] == "not-supported")

# Copy the book object from standard input to standard output.
context,book = json.JSONDecoder().decode(sys.stdin.read())
sys.stdout.write(json.JSONEncoder().encode(book))
sys.stdout.close() # Ensure that nothing else gets written.

pathlib.Path("./generated").mkdir(exist_ok=True)
gen_graph("src/imgs/src/MBC5_Rumble_Mild.csv", "Mild Rumble", "./generated/MBC5_Rumble_Mild.svg")
gen_graph("src/imgs/src/MBC5_Rumble_Strong.csv", "Strong Rumble", "./generated/MBC5_Rumble_Strong.svg")
