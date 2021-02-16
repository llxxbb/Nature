TRUNCATE TABLE `meta`;
TRUNCATE TABLE `relation`;
TRUNCATE TABLE `instances`;
TRUNCATE TABLE `task`;
TRUNCATE TABLE `task_error`;
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'delivery', '', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'deliveryState', '', 1, 'new|finished', '', '{"master":"B:delivery:1"}');

-- delivery --> deliveryState
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:delivery:1', 'B:deliveryState:1', '{"target":{"states":{"add":["new"]}, "append_para":[0,1]}}');

-- deliveryState --> delivery
INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:deliveryState:1', 'B:delivery:1', '{"selector":{"state_all":["finished"], "context_all":["mid"]}, "use_upstream_id":true, "executor":{"protocol":"localRust","url":"nature_demo:multi_delivery"}}');