from sys import argv, stderr

import lxml
import matplotlib.pyplot as plt
import pandas as pd
from bs4 import BeautifulSoup


def gen_graph(in_path, title):

    ## Let's draw the plot

    plt.rcParams["figure.figsize"] = [7.50, 3.50]
    plt.rcParams["figure.autolayout"] = True
    plt.rcParams["font.family"] = "Inter"
    # Assume fonts are installed on the machine where the SVG will be viewed
    #  (we load Inter with the webpage so it should be there)
    plt.rcParams["svg.fonttype"] = "none"

    # Those are just used to "fingerprint" the resulting elements in the SVG export,
    #  they will be replaced by CSS variables
    COLOR_BASE = "#FFCD01".lower()
    COLOR_LINE = "#FFCD02".lower()

    # Set everything to the base color
    plt.rcParams["text.color"] = COLOR_BASE
    plt.rcParams["axes.labelcolor"] = COLOR_BASE
    plt.rcParams["xtick.color"] = COLOR_BASE
    plt.rcParams["ytick.color"] = COLOR_BASE

    # Read the values to plot from the input CSV
    df = pd.read_csv(in_path)

    # Set the color of the actual plot line to the secondary color
    plot = df.set_index("Time (ms)").plot(
        legend=None, gid="fitted_curve", color=COLOR_LINE
    )

    # Add grid lines on the y values
    plot.yaxis.grid(True)

    # Set the color of the plot box to the base color too
    plt.setp(plot.spines.values(), color=COLOR_BASE)

    # Add title at the top
    plt.title(title)
    plt.ylabel(df.columns[1])
    plt.savefig("test.svg", transparent=True)

    ## Manipulate the SVG render of the plot to replace colors with CSS variables
    with open("test.svg", "r") as f:
        contents = f.read()
        # It's an SVG, so let's use the XML parser
        soup = BeautifulSoup(contents, "xml")

        replace_style_property(soup, "path", "stroke", COLOR_BASE, "var(--fg)")

        replace_style_property(soup, "path", "stroke", COLOR_LINE, "var(--inline-code-color)")

        replace_style_property(soup, "text", "fill", COLOR_BASE, "var(--fg)")

        # Write the altered SVG file
        with open("output.svg", "wb") as f_output:
            f_output.write(soup.prettify("utf-8"))
            print(soup)


def replace_style_property(
    soup, element_name, css_property, value_to_replace, new_value
):
    """
    Given a `Soup`, a CSS `property` applied inline, replace the a `specific value`
    this property can assume with `another` one in all the elements with the specified `name`

    E.g. the style of all the "path" elements whith a CSS property "fill" of
    "#ffcd01" will change to "var(--fg):

    `fill: #ffcd01; font: 12px 'Inter'; text-anchor: middle`
    to
    `fill: var(--fg); font: 12px 'Inter'; text-anchor: middle`

    Soup and soup ResultSet are modified in-place
    """
    found_elements = soup.find_all(
        element_name,
        style=lambda value: value and f"{css_property}: {value_to_replace}" in value,
    )
    # Replace the color magic value with the CSS variable
    for element in found_elements:
        element["style"] = element["style"].replace(
            f"{css_property}: {value_to_replace}", f"{css_property}: {new_value}"
        )

    return


if len(argv) != 3:
    print("Usage: python3 graph_render.py <path/to.csv> <graph title>", file=stderr)
    exit(1)

gen_graph(argv[1], argv[2])
