export var metaDefined = [
  {
    "id": 1,
    "meta_type": "B",
    "meta_key": "sale/order",
    "description": "order",
    "version": 1,
    "states": "",
    "fields": "",
    "config": "",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  },
  {
    "id": 2,
    "meta_type": "B",
    "meta_key": "sale/orderState",
    "description": "order state",
    "version": 1,
    "states": "new|paid|package|outbound|dispatching|signed|canceling|canceled",
    "fields": "",
    "config": "{\"master\":\"B:sale/order:1\"}",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  },
  {
    "id": 3,
    "meta_type": "B",
    "meta_key": "finance/payment",
    "description": "order payment",
    "version": 1,
    "states": "",
    "fields": "",
    "config": "",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  },
  {
    "id": 4,
    "meta_type": "B",
    "meta_key": "finance/orderAccount",
    "description": "order account",
    "version": 1,
    "states": "unpaid|partial|paid",
    "fields": "",
    "config": "{\"master\":\"B:sale/order:1\"}",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  },
  {
    "id": 5,
    "meta_type": "B",
    "meta_key": "third/waybill",
    "description": "waybill",
    "version": 1,
    "states": "",
    "fields": "",
    "config": "",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  },
  {
    "id": 6,
    "meta_type": "B",
    "meta_key": "sale/orderSign",
    "description": "order finished",
    "version": 1,
    "states": "",
    "fields": "",
    "config": "",
    "flag": 1,
    "create_time": "2021-01-10T10:16:10"
  }
]
export var relationDefined = [
  {
    "id": 1,
    "from_meta": "B:sale/order:1",
    "to_meta": "B:sale/orderState:1",
    "settings": "{\"target\":{\"state_add\":[\"new\"]}}",
    "flag": 1
  },
  {
    "id": 2,
    "from_meta": "B:sale/order:1",
    "to_meta": "B:finance/orderAccount:1",
    "settings": "{\"executor\":{\"protocol\":\"localRust\",\"url\":\"nature_demo:order_receivable\"},\"target\":{\"state_add\":[\"unpaid\"]}}",
    "flag": 1
  },
  {
    "id": 3,
    "from_meta": "B:finance/payment:1",
    "to_meta": "B:finance/orderAccount:1",
    "settings": "{\"executor\":{\"protocol\":\"localRust\",\"url\":\"nature_demo:pay_count\"}}",
    "flag": 1
  },
  {
    "id": 4,
    "from_meta": "B:finance/orderAccount:1",
    "to_meta": "B:sale/orderState:1",
    "settings": "{\"selector\":{\"state_all\":[\"paid\"]},\"target\":{\"state_add\":[\"paid\"]}}",
    "flag": 1
  },
  {
    "id": 5,
    "from_meta": "B:sale/orderState:1",
    "to_meta": "N:warehouse/outApplication:1",
    "settings": "{\"selector\":{\"state_all\":[\"paid\"]},\"executor\":{\"protocol\":\"localRust\",\"url\":\"nature_demo:stock_out_application\"}}",
    "flag": 1
  },
  {
    "id": 6,
    "from_meta": "B:sale/orderState:1",
    "to_meta": "B:sale/orderState:1",
    "settings": "{\"selector\":{\"state_all\":[\"paid\"]},\"executor\":{\"protocol\":\"http\",\"url\":\"http://localhost:8082/send_to_warehouse\"},\"target\":{\"state_add\":[\"package\"]}}",
    "flag": 1
  },
  {
    "id": 7,
    "from_meta": "B:sale/orderState:1",
    "to_meta": "B:third/waybill:1",
    "settings": "{\"id_bridge\":true, \"selector\":{\"state_all\":[\"outbound\"]}, \"executor\":{\"protocol\":\"localRust\",\"url\":\"nature_demo:go_express\"}}",
    "flag": 1
  },
  {
    "id": 8,
    "from_meta": "B:third/waybill:1",
    "to_meta": "B:sale/orderState:1",
    "settings": "{\"target\":{\"state_add\":[\"dispatching\"]}}",
    "flag": 1
  },
  {
    "id": 9,
    "from_meta": "B:sale/orderState:1",
    "to_meta": "B:sale/orderSign:1",
    "settings": "{\"delay\":1, \"id_bridge\":true, \"selector\":{\"state_all\":[\"dispatching\"]}, \"executor\":{\"protocol\":\"localRust\",\"url\":\"nature_demo:auto_sign\"}}",
    "flag": 1
  },
  {
    "id": 10,
    "from_meta": "B:sale/orderSign:1",
    "to_meta": "B:sale/orderState:1",
    "settings": "{\"target\":{\"state_add\":[\"signed\"]}}",
    "flag": 1
  }
]