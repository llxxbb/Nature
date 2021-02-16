#!/bin/bash

# input parameter:
#   meta : $1
#   content: $2

JSON_STRING=$( jq -n \
                  --arg meta "$1" \
                  --arg content "$2" \
                  '{"data":{"meta": $meta, "content": $content}}' )

#echo "$JSON_STRING"

# sed -e 's/^"//' -e 's/"$//' used to remove " at that surround with the value
curl -H "Content-type: application/json" -X POST \
     -d"$JSON_STRING" http://localhost:8080/input | jq '.Ok' | sed -e 's/^"//' -e 's/"$//'
