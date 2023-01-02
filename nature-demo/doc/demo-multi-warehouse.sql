TRUNCATE TABLE `meta`;
TRUNCATE TABLE `relation`;
TRUNCATE TABLE `instances`;
TRUNCATE TABLE `task`;
TRUNCATE TABLE `task_error`;

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'order', '', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'warehouse/self', '', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'warehouse/third', '', 1, '', '', '');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:order:1', 'B:warehouse/self:1', '{"selector":{"context_all":["self"]},"executor":{"protocol":"localRust","url":"nature_demo:multi_warehouse"}}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:order:1', 'B:warehouse/third:1', '{"selector":{"context_all":["third"]},"executor":{"protocol":"localRust","url":"nature_demo:multi_warehouse"}}');
