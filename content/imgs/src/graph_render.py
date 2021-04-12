import pygal
from pygal.style import Style
import math

# ------------------------------------------------------------------------------------------------
# Configuration Constants
# ------------------------------------------------------------------------------------------------

# Rotation of X-Axis labels in degrees
x_label_rotation = 45

# Number of labels to be displayed on the X-Axis
x_label_count = 10

# ------------------------------------------------------------------------------------------------
# The script will automatically generate SVG line-graphs with the same file name
# (replacing the .csv extension with .svg) within the /content/imgs directory
#
# The first line of the file must contain the X- and Y-Axis labels seperated by commas.
# The following lines are expected to contain the graph data in a comma-separated format
#  and in the same order as the Axis labels.
# ------------------------------------------------------------------------------------------------
graph_files = [
    {"filename": "MBC5_Rumble_Mild.csv", "title": "Mild Rumble"},
    {"filename": "MBC5_Rumble_Strong.csv", "title": "Strong Rumble"},
]
# ================================================================================================


def gen_graph(g_file):

    custom_style = Style(
        font_family="Inter",
        label_font_size=12,
        major_label_font_size=12,
        title_font_size=16
    )

    # Create Line Chart Object and Open File
    chart = pygal.Line(
        show_dots=False,
        show_legend=False,
        show_minor_x_labels=False,
        x_label_rotation=x_label_rotation,
        style=custom_style
    )
    csv = open(g_file["filename"], "r").readlines()

    # Set Chart and Axis Titles
    chart.title = g_file["title"]
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
    chart.render_to_file("../" + g_file["filename"].replace(".csv", ".svg"))


# Call gen_graph() for each file in the graph_files array
for g_file in graph_files:
    print(f"Generating {g_file['filename'].replace('.csv', '.svg')} ..")
    gen_graph(g_file)
