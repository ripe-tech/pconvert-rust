#!/usr/bin/python
# -*- coding: utf-8 -*-

import sys

try:
    from setuptools import setup
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools"])
    if errno:
        print("Please install setuptools package")
        raise SystemExit(errno)
    else:
        from setuptools import setup

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

setup(
    name = "pconvert-rust",
    version = "0.1.0",
    author = "Platforme International",
    author_email = "development@platforme.com",
    description = "PNG Convert Rust",
    license = "Apache License, Version 2.0",
    keywords = "pconvert rust fast",
    url = "https://www.platforme.com",
    packages = ["pconvert_rust"],
    rust_extensions = [RustExtension("pconvert_rust.pconvert_rust", binding = Binding.PyO3)],
    install_requires = install_requires,
    setup_requires = setup_requires,
    include_package_data = True,
    zip_safe = False
)