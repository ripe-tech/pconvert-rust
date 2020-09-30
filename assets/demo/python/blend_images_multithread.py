#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import pconvert_rust as pconvert

PATH_TO_ASSETS = "../"

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")

pconvert.blend_images(
    os.path.abspath(f"{PATH_TO_ASSETS}sole.png"),
    os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
    os.path.abspath("result.png"),
    options = {
        "num_threads": 5
    }
)
