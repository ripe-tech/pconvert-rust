#!/usr/bin/python
# -*- coding: utf-8 -*-

import sys

try:
    import setuptools
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools"])
    if errno:
        print("Please install setuptools package")
        raise SystemExit(errno)
    else:
        import setuptools

try:
    import setuptools_rust
except ImportError:
    import subprocess
    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        import setuptools_rust

setuptools.setup(
    name = "pconvert-rust",
    version = "0.1.2",
    author = "Platforme International",
    author_email = "development@platforme.com",
    description = "PNG Convert Rust",
    license = "Apache License, Version 2.0",
    keywords = "pconvert rust fast",
    url = "https://www.platforme.com",
    packages = [
        "pconvert_rust",
        "pconvert_rust.test"
    ],
    rust_extensions = [
        setuptools_rust.RustExtension(
            "pconvert_rust.pconvert_rust",
            binding = setuptools_rust.Binding.PyO3
        )
    ],
    install_requires = [],
    setup_requires = ["setuptools-rust>=0.10.1", "wheel"],
    include_package_data = True,
    zip_safe = False,
    classifiers = [
        "Development Status :: 5 - Production/Stable",
        "Topic :: Utilities",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Programming Language :: Python",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7"
    ]
)
