TRUNCATE TABLE `meta`;
TRUNCATE TABLE `relation`;
TRUNCATE TABLE `instances`;
TRUNCATE TABLE `task`;
TRUNCATE TABLE `task_error`;

-- all class's all subjects score to personal subject score ---------------------------------------------

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/table', 'store original score data', 1, '', '', '');

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/trainee/subject', 'person original score', 1, '', '', '');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:score/table:1', 'B:score/trainee/subject:1', '{"executor":{"protocol":"builtIn","url":"scatter"}, "convert_after":[{"protocol":"http","url":"http://127.0.0.1:8082/add_score"},{"protocol":"localRust","url":"nature_demo:name_to_id"}]}');

-- sum for personal subject ---------------------------------------------

INSERT INTO meta
(meta_type, meta_key, description, version, states, fields, config)
VALUES('B', 'score/trainee/all-subject', 'all subject\'s score for a person', 1, '', '', '{"is_state":true}');

INSERT INTO relation
(from_meta, to_meta, settings)
VALUES('B:score/trainee/subject:1', 'B:score/trainee/all-subject:1', '{"target":{"append_para":[0]},"executor":{"protocol":"builtIn","url":"merge","settings":"{\\"key\\":{\\"Para\\":[1]},\\"when_same\\":\\"Old\\",\\"sum_all\\":true}"}}');
