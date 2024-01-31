#!/bin/bash

set -e

# $1 = lir program file; $2 = json format file; $3 = function to analyze
./constants_analysis "$2" "$3"
