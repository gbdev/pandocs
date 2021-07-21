import pygal
from pygal.style import Style
import math
from sys import argv, stderr

# ------------------------------------------------------------------------------------------------
# Configuration Constants
# ------------------------------------------------------------------------------------------------

# Rotation of X-Axis labels in degrees
x_label_rotation = 0.01

# Number of labels to be displayed on the X-Axis
x_label_count = 10

# ------------------------------------------------------------------------------------------------
# The first line of the file must contain the X- and Y-Axis labels seperated by commas.
# The following lines are expected to contain the graph data in a comma-separated format
#  and in the same order as the Axis labels.
# ------------------------------------------------------------------------------------------------

def gen_graph(in_path, title):
    custom_style = Style(
        font_family="Inter",
        label_font_size=12,
        major_label_font_size=12,
        title_font_size=16
    )

    # Create Line Chart Object and Open File
    chart = pygal.Line(
        height=450,
        show_dots=False,
        show_legend=False,
        show_minor_x_labels=False,
        x_label_rotation=x_label_rotation,
        style=custom_style
    )
    csv = open(in_path, "r").readlines()

    # Set Chart and Axis Titles
    chart.title = title
    headers = csv.pop(0).split(",")
    chart.x_title = headers[0]
    chart.y_title = headers[1]

    # Generate label spacing variables
    min_x_val = float(csv[0].split(",")[0])
    max_x_val = float(csv[len(csv) - 1].split(",")[0])
    x_mod_val = (max_x_val - min_x_val) / x_label_count

    # Generate graph data arrays
    x_labels = []
    x_labels_major = []
    y_data = []
    last_x = None
    for line in csv:
        # Add data to label arrays
        data = line.split(",")
        x_labels.append(data[0])
        y_data.append(float(data[1]))

        # Check if current X-Label should be Major Label
        xval_float = float(data[0])
        if last_x is not None and ((last_x % x_mod_val) > (xval_float % x_mod_val)):
            x_labels_major.append(math.floor(xval_float))
            x_labels.append(math.floor(xval_float))
        last_x = xval_float

    # Load graph data into chart object and save to file
    chart.x_labels = x_labels
    chart.x_labels_major = x_labels_major

    chart.add("", y_data)
    print(chart.render(is_unicode=True))


if len(argv) != 3:
    print("Usage: python3 graph_render.py <path/to.csv> <graph title>", file=stderr)
    exit(1)

gen_graph(argv[1], argv[2])