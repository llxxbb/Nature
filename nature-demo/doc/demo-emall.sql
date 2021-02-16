TRUNCATE TABLE `meta`;
TRUNCATE TABLE `relation`;
TRUNCATE TABLE `instances`;
TRUNCATE TABLE `task`;
TRUNCATE TABLE `task_error`;

-- generate order ---------------------------------------------
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/orderState', 'order state', 1, 'new|paid|package|outbound|dispatching|signed|canceling|canceled', '', '{"master":"B:sale/order:1"}');

-- order --> orderState
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:sale/orderState:1', '{"target":{"state_add":["new"]}}');

-- pay for the bill  ---------------------------------------------
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'finance/payment', 'order payment', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'finance/orderAccount', 'order account', 1, 'unpaid|partial|paid', '', '{"master":"B:sale/order:1"}');

-- order --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_receivable"},"target":{"state_add":["unpaid"]}}');

-- payment --> orderAccount
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:finance/payment:1', 'B:finance/orderAccount:1', '{"executor":{"protocol":"localRust","url":"nature_demo:pay_count"}}');

-- orderAccount --> orderState
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:finance/orderAccount:1', 'B:sale/orderState:1', '{"selector":{"state_all":["paid"]},"target":{"state_add":["paid"]}}');

-- stock out  ---------------------------------------------

-- orderState:paid --> stockOutApplication
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'N:warehouse/outApplication:1', '{"selector":{"state_all":["paid"]},"executor":{"protocol":"localRust","url":"nature_demo:stock_out_application"}}');

-- orderState:paid --> orderState:package
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderState:1', '{"selector":{"state_all":["paid"]},"executor":{"protocol":"http","url":"http://localhost:8082/send_to_warehouse"},"target":{"state_add":["package"]}}');

-- delivery  ---------------------------------------------
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'third/waybill', 'waybill', 1, '', '', '');

-- orderState:outbound --> waybill
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:third/waybill:1', '{"id_bridge":true, "selector":{"state_all":["outbound"]}, "executor":{"protocol":"localRust","url":"nature_demo:go_express"}}');

-- waybill --> orderState:dispatching
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:third/waybill:1', 'B:sale/orderState:1', '{"target":{"state_add":["dispatching"]}}');

-- signed  ---------------------------------------------
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/orderSign', 'order finished', 1, '', '', '');

-- orderState:dispatching --> orderSign
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderState:1', 'B:sale/orderSign:1', '{"delay":1, "id_bridge":true, "selector":{"state_all":["dispatching"]}, "executor":{"protocol":"localRust","url":"nature_demo:auto_sign"}}');

-- orderSign --> orderState:signed
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/orderSign:1', 'B:sale/orderState:1', '{"target":{"state_add":["signed"]}}');

