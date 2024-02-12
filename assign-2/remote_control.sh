#!/bin/bash

REMOTE_HOST="csil.cs.ucsb.edu"
REMOTE_USER="zhaohui"
REMOTE_PASSWORD="mimawangle123A"
REMOTE_EXECUTABLE="/cs/faculty/benh/260/control"
LOCAL_PROGRAM=$1 # e.g., "./demos/lir/test1.lir"
REMOTE_PROGRAM="test.lir"  # under ~/ directory
FUNCTION=$2 # e.g., FUNCTION="test"

# if has $3 argument, set REMOTE_OUTPUT_FILE to $3, esle set REMOTE_OUTPUT_FILE to "output.txt"
if [ -z "$3" ]
then
    REMOTE_OUTPUT_FILE="output.txt"
    LOCAL_OUTPUT_FILE="output.txt"
else
    REMOTE_OUTPUT_FILE=$3
    LOCAL_OUTPUT_FILE=$3
fi

sshpass -p "$REMOTE_PASSWORD" scp $LOCAL_PROGRAM $REMOTE_USER@$REMOTE_HOST:$REMOTE_PROGRAM
echo "transfered $LOCAL_PROGRAM to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PROGRAM"

expect -c "
spawn ssh $REMOTE_USER@$REMOTE_HOST $REMOTE_EXECUTABLE $REMOTE_PROGRAM $FUNCTION > $REMOTE_OUTPUT_FILE
expect \"password:\"
send \"$REMOTE_PASSWORD\r\"
expect eof
"

sshpass -p "$REMOTE_PASSWORD" scp $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE $LOCAL_OUTPUT_FILE
echo "transfered $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE to $LOCAL_OUTPUT_FILE"
