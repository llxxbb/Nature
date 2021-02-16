#!/bin/bash

# input parameter:
#   id : $1
#   meta: $2
#   state_version: $3

path=$(dirname "$0")

wait=true

rtn=$("$path"/get_by_id.sh "$1" "$2" "$3")
while [ $wait ]; do
  echo "$0-------------$1 $2 $3"
  if [ -n "$rtn" ]&&[ "$rtn" != "null" ]; then
    break
  fi
  sleep 1
  rtn=$("$path"/get_by_id.sh "$1" "$2" "$3")
done
echo "$rtn"
