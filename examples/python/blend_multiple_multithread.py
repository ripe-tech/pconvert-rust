#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import time
import threading
import pconvert_rust as pconvert

PATH_TO_ASSETS = os.path.join(os.path.dirname(__file__), "../../assets/demo/")

print(f"VERSION: {pconvert.VERSION}")
print(f"COMPILED ON: {pconvert.COMPILATION_DATE}, {pconvert.COMPILATION_TIME}")


def print_pool_status():
    while True:
        print(pconvert.get_thread_pool_status())
        time.sleep(0.1)


def blend(i):
    pconvert.blend_multiple(
        (
            os.path.abspath(f"{PATH_TO_ASSETS}sole.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}front.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}shoelace.png"),
            os.path.abspath(f"{PATH_TO_ASSETS}background_alpha.png"),
        ),
        os.path.abspath(f"result{i}.png"),
        options={"num_threads": 20},
    )


pool_status_thread = threading.Thread(target=print_pool_status)
pool_status_thread.daemon = True
pool_status_thread.start()

blending_threads = [threading.Thread(target=blend, args=(x,)) for x in range(1000)]

for blending_thread in blending_threads:
    blending_thread.start()

for blending_thread in blending_threads:
    blending_thread.join()
