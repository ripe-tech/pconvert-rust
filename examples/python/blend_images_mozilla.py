#!/usr/bin/python
# -*- coding: utf-8 -*-

# Sets of tests related to MDN documentation @ https://developer.mozilla.org/docs/Web/API/Canvas_API/Tutorial/Compositing/Example

import os
import pconvert_rust as pconvert

PATH_TO_ASSETS = os.path.join(os.path.dirname(__file__), "../../assets/mozilla/")

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")

pconvert.blend_images(
    os.path.abspath(f"{PATH_TO_ASSETS}source.png"),
    os.path.abspath(f"{PATH_TO_ASSETS}destination.png"),
    os.path.abspath("result.source_over.mozilla.png"),
    "source_over",
)

pconvert.blend_images(
    os.path.abspath(f"{PATH_TO_ASSETS}source.png"),
    os.path.abspath(f"{PATH_TO_ASSETS}destination.png"),
    os.path.abspath("result.destination_over.mozilla.png"),
    "destination_over",
)
