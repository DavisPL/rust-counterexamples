#!/bin/bash

# Check if the user is root
if [ "$(id -u)" != "0" ]; then
   echo "This script must be run as root" 1>&2
   exit 1
fi

# Check if gdb is installed
if ! command -v gdb &> /dev/null
then
    echo "gdb could not be found, please install gdb first"
    exit
fi

# Check for correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <PID> <Number to Search>"
    exit 1
fi

PID=$1
NUMBER=$2

# Use gdb to attach to the process and search memory
gdb -n -q -batch \
    -ex "attach $PID" \
    -ex "set pagination 0" \
    -ex "dump memory /tmp/memory_dump.bin 0xaaaae1270000 0xaaaae12c4000" \
    -ex "detach" \
    -ex "quit"

# Search for the number in the memory dump
grep --binary-files=text --text -a -o $NUMBER /tmp/memory_dump.bin

