#!/bin/bash

set -e

# $1 = lir program file; $2 = json format file; $3 = function to analyze
# ./ctrl_analysis "$2" "$3"

GRAPHML_FILE=$(uuidgen).graphml
./gene_graphml "$2" "$3" $GRAPHML_FILE
python ctrl.py $GRAPHML_FILE
rm -f $GRAPHML_FILE
