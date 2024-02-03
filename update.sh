#!/bin/bash

# Check if the correct number of arguments was provided
if [ "$#" -ne 3 ]; then
    echo "Usage: $0 <PID> <Memory Address> <Number>"
    exit 1
fi

PID=$1
MEMORY_ADDRESS=$2
NUMBER=$3

# Create a gdb command file
GDB_COMMANDS=$(mktemp)
echo "set *((int *)0x$MEMORY_ADDRESS) = $NUMBER" > $GDB_COMMANDS
echo "detach" >> $GDB_COMMANDS
echo "quit" >> $GDB_COMMANDS

# Use gdb to attach to the process and run the commad
gdb -p $PID -x $GDB_COMMANDS

# Cleanup
rm $GDB_COMMANDS
