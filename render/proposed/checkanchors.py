#!/usr/bin/env python3
"""
checkanchors, an internal link checker

Copyright 2021 Damian Yerrick

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""
import os
import sys
import argparse
import html5lib
import xml.etree.ElementTree as ET

helpText="Checks an HTML file for missing internal anchors."
versionText = """checkanchors v0.01
Copyright 2021 Damian Yerrick
This is free software under the MIT License (Expat variant)."""

def parse_argv(argv):
    p = argparse.ArgumentParser(description=helpText)
    p.add_argument('--version', action='version', version=versionText)
    p.add_argument("input", default="-",
                   help="name of HTML file to check (or - for standard input)")
    p.add_argument("-o", "--output", default="-",
                   help="where to write the diagnostic (default: standard output)")
    return p.parse_args(argv[1:])

def main(argv=None):
    args = parse_argv(argv or sys.argv)
    if args.input == "-":
        body = sys.stdin.read()
    else:
        with open(args.input, "r", encoding="utf-8") as infp:
            body = infp.read()
    doc = html5lib.parse(body, treebuilder="etree", namespaceHTMLElements=False)
    out = []

    # Look for multiple IDs
    els = doc.findall(".//*[@id]")
    seen_ids, errs = {}, []
    for el in els:
        idvalue = el.get("id")
        textcontent = " ".join(ET.tostring(el, "unicode", "text").split())
        try:
            oldtext = seen_ids[idvalue]
        except KeyError:
            seen_ids[idvalue], oldtext = textcontent, None
        else:
            errs.append('id="%s"\n    %s\n    %s\n'
                        % (idvalue, oldtext, textcontent))
    if errs:
        out.append("IDs seen multiple times\n")
        out.extend(errs)
        out.append("\n")
        
    seen_ids, errs = set(seen_ids), []

    # Look for missing anchors
    els = doc.findall(".//a[@href]")
    for el in els:
        hrefvalue = el.get("href")
        if not hrefvalue.startswith("#"): continue
        desired_id = hrefvalue[1:]
        if desired_id not in seen_ids:
            outerxml = " ".join(ET.tostring(el, "unicode", "xml").split())
            errs.append(outerxml)
    if errs:
        out.append("Dangling internal links (whose fragment matches no id= value)\n")
        out.extend("    %s\n" % x for x in errs)
        out.append("\n")
    errs = []

    # Look for consecutive headings with no text between them
    for el in doc.iter():
        prevel, prevtag, prevtail = None, "", ""
        for child in el:
            tag, tail = child.tag, (child.tail or "").strip()
            if tag is ET.Comment or tag == "hr":
                prevtail = prevtail + tail
                continue
            if (prevtag.startswith("h") and tag.startswith("h")
                and tail == ''
                and prevtag[1:].isdigit() and tag[1:].isdigit()):
                prevhlevel, hlevel = int(prevtag[1:]), int(tag[1:])
                if prevhlevel >= hlevel:
                    errs.append(ET.tostring(prevel, "unicode", "text"))
            prevel, prevtag, prevtail = child, child.tag, child.tail
    if errs:
        out.append("Empty sections\n")
        out.extend("    %s\n" % x for x in errs)
        out.append("\n")
    errs = []

    # Write the report.  Will be empty if no errors.
    if args.output == '-':
        sys.stdout.writelines(out)
    else:
        with open(args.output, "w", encoding="utf-8") as outfp:
            outfp.writelines(out)

if __name__=='__main__':
    if 'idlelib' in sys.modules:
        main(["./checkanchors.py", "index.html"])
    else:
        main()
