#!/bin/bash

set -e

# $1 = generated constraints file
./solver "$1"
