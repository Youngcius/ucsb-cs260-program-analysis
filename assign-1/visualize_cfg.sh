#!/bin/bash

# DOT_FILE="test.dot"

# if $1 has value, set DOT_FILE to $1; otherwise, DOT_FILE is "test.dot"
if [ -z "$1" ]
then
    DOT_FILE="test.dot"
else
    DOT_FILE=$1
fi

if [ -z "$2" ]
then
    OUTPUT_FILE="test.png"
else
    OUTPUT_FILE=$2
fi

dot -Tpng $DOT_FILE -o $OUTPUT_FILE
