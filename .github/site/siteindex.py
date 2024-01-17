#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Created on Mon Jul  6 15:07:00 2020

generate a site index for the github pages site
"""
import sys

header = """
<html>
<head>
<title>xps-gateway</title>
</head>
<body>
<h1>xps-gateway</h1>
"""

footer = """
</body></html>
"""

if __name__ == "__main__":
    package = sys.argv[1:]
    print(header)
    for p in package:
        print("<a href=\"{}/\">{}</a><br>".format(p,p))
    print(footer)
