#!/bin/bash
# -*- coding: utf-8 -*-

echo "Uploading to pypi using $PYPI_USERNAME"

echo -e "[pypi]\nusername = $PYPI_USERNAME\npassword = $PYPI_PASSWORD\n" > ~/.pypirc

python3 setup.py sdist bdist_wheel
pip3 install twine
twine upload dist/*
