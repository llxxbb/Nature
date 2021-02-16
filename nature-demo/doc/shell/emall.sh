#!/bin/bash

# This shell to simulate the business system client to communicate with Nature.

path=$(dirname "$0")

# generate order-----------------------------------

instance='{
  "user_id":123,
  "price":1000,
  "items":[
    {
      "item":{
        "id":1,
        "name":"phone",
        "price":800
      },
      "num":1
    },
    {
      "item":{
        "id":2,
        "name":"battery",
        "price":100
      },
      "num":1
    }
  ],
  "address":"a.b.c"
}'

# submit order to Nature
orderID=$("$path"/common/input.sh "B:sale/order:1" "$instance")

# cam be reentrant----------------------------------------
rtn2=$("$path"/common/input.sh "B:sale/order:1" "$instance")

if [ "$orderID" != "$rtn2" ]; then
  echo "should be equal"
  exit 1
fi

# wait order-account instance generated----------------------------
"$path"/common/get_by_id_wait.sh "$orderID" "B:finance/orderAccount:1" 1

# ============================ pay ============================
pay () {
  echo "$1"

  json=$(jq -n \
    --arg order "$1" \
    --arg account "$3" \
    --arg num "$2" \
    --arg time "$4" \
    '{"order":$order,"from_account":$account,"paid":$num|tonumber,"pay_time":$time|tonumber}')
  "$path"/common/input.sh "B:finance/payment:1" "$json"
}

# pay for the first time----------------------------

time=$(date +%s)"000"
payFirst=$(pay "$orderID" 100 "a" "$time")
echo "$payFirst"




