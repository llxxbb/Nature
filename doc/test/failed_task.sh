#!/bin/sh

MANAGER_URL='192.168.3.25:8180'
# 用于获取某个目标的错误数
curl http://$MANAGER_URL/failed/num/B:sale/order:1