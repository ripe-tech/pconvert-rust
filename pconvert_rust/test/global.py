#!/usr/bin/python
# -*- coding: utf-8 -*-

import unittest

import pconvert_rust

class GlobalTest(unittest.TestCase):

    def test_basic(self):
        self.assertEqual(type(pconvert_rust.VERSION), str)
        self.assertEqual(pconvert_rust.VERSION, "0.2.8")
