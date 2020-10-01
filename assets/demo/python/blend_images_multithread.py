#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import time
import threading
import pconvert_rust as pconvert

PATH_TO_ASSETS = "../"

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")


def print_pool_status():
    while True:
        print(pconvert.get_thread_pool_status())
        time.sleep(0.1)

x = threading.Thread(target=print_pool_status)

x.start()

for _ in range(100000):
    print("Blending")
    pconvert.blend_multiple(
        (
            os.path.abspath(f"{PATH_TO_ASSETS}sole.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
        ),
        os.path.abspath("result.png"),
        options = {
            "num_threads": 5
        }
    )
