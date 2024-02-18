#!/usr/bin/env bash

set -e

# assuming you used a Makefile
make
pip3 install networkx-3.2.1-py3-none-any.whl
