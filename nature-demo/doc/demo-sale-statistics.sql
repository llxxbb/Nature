TRUNCATE TABLE `meta`;
TRUNCATE TABLE `relation`;
TRUNCATE TABLE `instances`;
TRUNCATE TABLE `task`;
TRUNCATE TABLE `task_error`;

-- order to item ---------------------------------------------
INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/order', 'order', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money', 'item money', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/count', 'item count', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('M', 'sale/order/to_item', '', 1, '', '', '{"multi_meta":["B:sale/item/count:1","B:sale/item/money:1"]}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/order:1', 'M:sale/order/to_item:1', '{"executor":{"protocol":"localRust","url":"nature_demo:order_to_item"}}');

-- time range for every item --------------------------------------------------------------------

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money/tag_second', 'time range for second' , 1, '', '', '{"cache_saved":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money:1', 'B:sale/item/money/tag_second:1', '{"target":{"append_para":[0],"dynamic_para":"(item)"},"executor":{"protocol":"builtIn","url":"time_range"}}');

-- second statistics ---------------------------------------------

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/item/money/second', 'second summary of money' , 1, '', '', '');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money/tag_second:1', 'B:sale/item/money/second:1', '{"convert_before":[{"protocol":"builtIn","url":"instance-loader","settings":"{\\"key_gt\\":\\"B:sale/item/money:1|0|(item)/\\",\\"key_lt\\":\\"B:sale/item/money:1|0|(item)0\\",\\"time_part\\":[0,1]}"}],"delay_on_para":[2,1],"executor":{"protocol":"builtIn","url":"merge"}}');

-- top statistics ---------------------------------------------

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/money/second_tag', 'top of money task' , 1, '', '', '{"cache_saved":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/item/money/second:1', 'B:sale/money/second_tag:1', '{"target":{"append_para":[0,1],"dynamic_para":"(time)"}}');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'sale/money/secondTop', 'top of money' , 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('L', 'sale/money/secondTopLooper', 'top looper' , 1, '', '', '{"multi_meta":["B:sale/money/secondTop:1"], "only_one":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:sale/money/second_tag:1', 'L:sale/money/secondTopLooper:1', '{
"convert_before":[
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money:1|0\\",\\"key_lt\\":\\"B:sale/item/money:1|1\\",\\"time_part\\":[0,1]}"},
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money/tag_second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/tag_second:1|0|(time)0\\"}"},
    {"protocol":"builtIn","url":"task-checker","settings":"{\\"key_gt\\":\\"B:sale/item/money/second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/second:1|0|(time)0\\",\\"time_part\\":[0,1]}"},
    {"protocol":"builtIn","url":"instance-loader","settings":"{\\"key_gt\\":\\"B:sale/item/money/second:1|0|(time)/\\",\\"key_lt\\":\\"B:sale/item/money/second:1|0|(time)0\\",\\"page_size\\":1,\\"filters\\":[{\\"protocol\\":\\"builtIn\\",\\"url\\":\\"para_as_key\\",\\"settings\\":\\"{\\\\\\"plain\\\\\\":true,\\\\\\"part\\\\\\":[2]}\\"}]}"}
],"delay_on_para":[2,1],"executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":\\"Content\\",\\"sum_all\\":true,\\"top\\":{\\"MaxTop\\":1}}"}}');


