#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import pconvert_rust as pconvert

def read_file_data(path):
    file = open(path, "rb")
    try: data = file.read()
    finally: file.close()
    return data

PATH_TO_ASSETS = os.path.join(os.path.dirname(__file__), "../../assets/demo/")

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")

data = pconvert.blend_images(
    read_file_data(os.path.abspath(f"{PATH_TO_ASSETS}sole.png")),
    read_file_data(os.path.abspath(f"{PATH_TO_ASSETS}back.png"))
)

print(data)
print(type(data))
print(len(data))

file = open(os.path.abspath("result.png"), "wb")
try: file.write(data)
finally: file.close()

# data = read_file_data(os.path.abspath(f"{PATH_TO_ASSETS}sole.png"))
# print(data)
# print(type(data))
# print(len(data))

# pconvert.blend_images(
#     read_file_data(os.path.abspath("result.png")),
#     read_file_data(os.path.abspath(f"{PATH_TO_ASSETS}front.png")),
#     os.path.abspath("result.png")
# )
