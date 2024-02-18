#!/bin/bash

set -e

# $1 = lir program file; $2 = json format file; $3 = function to analyze
# ./ctrl_analysis "$2" "$3"
RANDOM_FILENAME=$(date '+%Y%m%d%H%M%S')_$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 5)
# GRAPHML_FILE=$(uuidgen).graphml
GRAPHML_FILE=$RANDOM_FILENAME.graphml
./gene_graphml "$2" "$3" $GRAPHML_FILE
python3 ctrl.py $GRAPHML_FILE
rm -f $GRAPHML_FILE
