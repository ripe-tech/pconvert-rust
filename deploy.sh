#!/bin/bash
# -*- coding: utf-8 -*-

set -ex

echo "Uploading to pypi using $PYPI_USERNAME"

echo -e "[pypi]\nusername = $PYPI_USERNAME\npassword = $PYPI_PASSWORD\n" > ~/.pypirc

PYTHON3=${PYTHON3-python3}
PIP3=${PIP3-pip3}

$PYTHON3 setup.py build sdist
$PIP3 install --upgrade "twine<2"
$PYTHON3 -m twine upload dist/*
