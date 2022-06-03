#!/bin/sh

MANAGER_URL='192.168.3.25:8180'
HEADER="Content-Type: application/json"

echo "--- get all failed of Order ----------------------------------"
FUN="failed/num"
DATA='{"task_for":"B:sale/order:1"}'
RES=$(curl -H "$HEADER" -X POST -d $DATA http://$MANAGER_URL/$FUN | jq '.Ok')
echo $RES
if [ "$RES" != 1 ]; then
  echo "*** ERROR ***"
  exit 1
fi

echo "--- get Order failed taskes ----------------------------------"
FUN="failed"
DATA='{"task_for":"B:sale/order:1","limit":100}'
#FILTER='.Ok.[] | length'
RES=$(curl -H "$HEADER" -X POST -d $DATA http://$MANAGER_URL/$FUN | jq '.Ok|length')
echo "should get 1 record, got: $RES"
if [ "$RES" != 1 ]; then
  echo "*** ERROR ***"
  exit 1
fi

echo "--- reset task ----------------------------------"
FUN="failed/reset"
DATA='["1","2","3"]'
#FILTER='.Ok.[] | length'
RES=$(curl -H "$HEADER" -X POST -d $DATA http://$MANAGER_URL/$FUN | jq '.Ok')
echo "should get 3 record, got: $RES"
if [ "$RES" != 3 ]; then
  echo "*** ERROR ***"
  exit 1
fi






## unfinished

#echo "--- delete task ----------------------------------"
FUN="failed/delete"
DATA='["1","2","3"]'
#FILTER='.Ok.[] | length'
RES=$(curl -H "$HEADER" -X POST -d $DATA http://$MANAGER_URL/$FUN | jq '.Ok')
echo "should get 3 record, got: $RES"
if [ "$RES" != 3 ]; then
  echo "*** ERROR ***"
  exit 1
fi

#echo "--- delete task for meta ----------------------------------"
FUN="failed/deleteFor"
DATA='["1","2","3"]'
#FILTER='.Ok.[] | length'
RES=$(curl -H "$HEADER" -X POST -d $DATA http://$MANAGER_URL/$FUN | jq '.Ok')
echo "should get 3 record, got: $RES"
if [ "$RES" != 3 ]; then
  echo "*** ERROR ***"
  exit 1
fi

