#!/bin/bash

source ../config.ini
REMOTE_HOST=$CSIL_HOST
REMOTE_USER=$CSIL_USER
REMOTE_PASSWORD=$CSIL_PASSWORD
REMOTE_EXECUTABLE="/cs/faculty/benh/260/constraint-solver"
LOCAL_CONSTRAINTS_GENERATED=$1 # e.g., "./generated.out.tmp.txt"
REMOTE_CONSTRAINTS_GENERATED="test.generated.txt"  # under ~/ directory
REMOTE_OUTPUT_FILE="solved.out.tmp.txt"

# if has $3 argument, set LOCAL_OUTPUT_FILE to $3
if [ -z "$3" ]
then
    LOCAL_OUTPUT_FILE="solved.out.tmp.txt"
else
    LOCAL_OUTPUT_FILE=$3
fi

sshpass -p "$REMOTE_PASSWORD" scp $LOCAL_CONSTRAINTS_GENERATED $REMOTE_USER@$REMOTE_HOST:$REMOTE_CONSTRAINTS_GENERATED
echo "transferred $LOCAL_CONSTRAINTS_GENERATED to $REMOTE_USER@$REMOTE_HOST:$REMOTE_CONSTRAINTS_GENERATED"

expect -c "
spawn ssh $REMOTE_USER@$REMOTE_HOST $REMOTE_EXECUTABLE $REMOTE_CONSTRAINTS_GENERATED > $REMOTE_OUTPUT_FILE
expect \"password:\"
send \"$REMOTE_PASSWORD\r\"
expect eof
"

sshpass -p "$REMOTE_PASSWORD" scp $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE $LOCAL_OUTPUT_FILE
echo "transferred $REMOTE_USER@$REMOTE_HOST:$REMOTE_OUTPUT_FILE to $LOCAL_OUTPUT_FILE"
