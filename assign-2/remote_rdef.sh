#!/bin/bash

source ../config.ini
REMOTE_HOST=$CSIL_HOST
REMOTE_USER=$CSIL_USER
REMOTE_PASSWORD=$CSIL_PASSWORD
REMOTE_EXECUTABLE="/cs/faculty/benh/260/rdef"
LOCAL_PROGRAM=$1 # e.g., "./demos/lir/test1.lir"
REMOTE_PROGRAM="test.rdef.lir"  # under ~/ directory
FUNCTION=$2 # e.g., FUNCTION="test"
REMOTE_OUTPUT_FILE="rdef.out.tmp.txt"

# if has $3 argument, set LOCAL_OUTPUT_FILE to $3
if [ -z "$3" ]
then
    LOCAL_OUTPUT_FILE="rdef.out.tmp.txt"
else
    LOCAL_OUTPUT_FILE=$3
fi

sshpass -p "$REMOTE_PASSWORD" scp $LOCAL_PROGRAM $REMOTE_USER@$REMOTE_HOST:$REMOTE_PROGRAM
echo "transferred $LOCAL_PROGRAM to $REMOTE_USER@$REMOTE_HOST:$REMOTE_PROGRAM"

expect -c "
spawn ssh $REMOTE_USER@$REMOTE_HOST $REMOTE_EXECUTABLE $REMOTE_PROGRAM $FUNCTION > $REMOTE_OUTPUT_FILE
expect \"password:\"
send \"$REMOTE_PASSWORD\r\"
expect eof
"

sshpass -p "$REMOTE_PASSWORD" scp $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE $LOCAL_OUTPUT_FILE
echo "transferred $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE to $LOCAL_OUTPUT_FILE"
