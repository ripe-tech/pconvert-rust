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

def blend(i):
    pconvert.blend_images(
        os.path.abspath(f"{PATH_TO_ASSETS}sole.png"),
        os.path.abspath(f"{PATH_TO_ASSETS}back.png"),
        os.path.abspath(f"result{i}.png"),
        options = {
            "num_threads": 5
        }
    )

pool_status_thread = threading.Thread(target=print_pool_status)
pool_status_thread.daemon = True
pool_status_thread.start()

blending_threads = [threading.Thread(target=blend, args=(x,)) for x in range(10)]

for blending_thread in blending_threads:
    blending_thread.start()
    time.sleep(0.2)

for blending_thread in blending_threads:
    blending_thread.join()
