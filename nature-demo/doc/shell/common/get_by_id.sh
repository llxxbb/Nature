#!/bin/bash

# input parameter:
#   id : $1
#   meta: $2
#   state_version: $3

JSON_STRING=$( jq -n \
                  --arg a "$1" \
                  --arg b "$2" \
                  --argjson sta_ver "$3" \
                  '{"id":$a, "meta":$b ,"state_version":$sta_ver}' )

echo "$JSON_STRING"

curl -H "Content-type: application/json" -X POST \
     -d"$JSON_STRING" http://localhost:8080/get/byId | jq '.Ok'
