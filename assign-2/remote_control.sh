#!/bin/bash

REMOTE_HOST="csil.cs.ucsb.edu"
REMOTE_USER="zhaohui"
REMOTE_PASSWORD="mimawangle123A"
REMOTE_EXECUTABLE="/cs/faculty/benh/260/control"
LOCAL_PROGRAM=$1 # e.g., "./demos/lir/test1.lir"
REMOTE_PROGRAM="test.control.lir"  # under ~/ directory
FUNCTION=$2 # e.g., FUNCTION="test"
REMOTE_OUTPUT_FILE="control.out.tmp.txt"

# if has $3 argument, set LOCAL_OUTPUT_FILE to $3
if [ -z "$3" ]
then
    LOCAL_OUTPUT_FILE="control.out.tmp.txt"
else
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
