#!/usr/bin/python
# -*- coding: utf-8 -*-

import os
import unittest
import pconvert_rust

TEST_ASSETS = os.path.join(os.path.dirname(__file__), "../../assets/test/")

class BlendingTest(unittest.TestCase):

    def test_blend_images(self):
        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}sole.png"),
            os.path.abspath(f"{TEST_ASSETS}back.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png")
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}front.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}shoelace.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}background_alpha.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
        )

    def test_blend_images_multithread(self):
        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}sole.png"),
            os.path.abspath(f"{TEST_ASSETS}back.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            options = {
                "num_threads": 5
            }
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}front.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            options = {
                "num_threads": 5
            }
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}shoelace.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            options = {
                "num_threads": 5
            }
        )

        pconvert_rust.blend_images(
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            os.path.abspath(f"{TEST_ASSETS}background_alpha.png"),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            options = {
                "num_threads": 5
            }
        )

    def test_blend_multiple(self):
        pconvert_rust.blend_multiple(
            (
                os.path.abspath(f"{TEST_ASSETS}sole.png"),
                os.path.abspath(f"{TEST_ASSETS}back.png"),
                os.path.abspath(f"{TEST_ASSETS}front.png"),
                os.path.abspath(f"{TEST_ASSETS}shoelace.png"),
                os.path.abspath(f"{TEST_ASSETS}background_alpha.png")
            ),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png")
        )

    def test_blend_multiple_multithread(self):
        pconvert_rust.blend_multiple(
            (
                os.path.abspath(f"{TEST_ASSETS}sole.png"),
                os.path.abspath(f"{TEST_ASSETS}back.png"),
                os.path.abspath(f"{TEST_ASSETS}front.png"),
                os.path.abspath(f"{TEST_ASSETS}shoelace.png"),
                os.path.abspath(f"{TEST_ASSETS}background_alpha.png")
            ),
            os.path.abspath(f"{TEST_ASSETS}result_alpha_alpha_Fast_NoFilter.png"),
            options = {
                "num_threads": 5
            }
        )
