#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import pconvert_rust as pconvert

PATH_TO_ASSETS = os.path.join(os.path.dirname(__file__), "../")

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")

pconvert.blend_multiple(
    (
        os.path.abspath(f"{PATH_TO_ASSETS}sole.png"),
        os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
        os.path.abspath(f"{PATH_TO_ASSETS}front.png")
    ),
    os.path.abspath("result.basic.png")
)
