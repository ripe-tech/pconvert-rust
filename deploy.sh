#!/bin/bash
# -*- coding: utf-8 -*-

set -ex

echo "Uploading to pypi using $PYPI_USERNAME"

echo -e "[pypi]\nusername = $PYPI_USERNAME\npassword = $PYPI_PASSWORD\n" > ~/.pypirc

python3 setup.py build sdist
pip3 install --upgrade twine
python3 -m twine upload dist/*
