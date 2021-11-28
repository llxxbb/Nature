DROP TABLE `meta`;
DROP TABLE `relation`;
DROP TABLE `instances`;
DROP TABLE `task`;
DROP TABLE `task_error`;

create TABLE `meta` (
    `id`        int(11) NOT NULL AUTO_INCREMENT,
	`meta_type`	VARCHAR ( 10 ) NOT NULL,
	`meta_key`	VARCHAR ( 255 ) NOT NULL,
	`description`	VARCHAR ( 1023 ),
	`version`	INTEGER NOT NULL,
	`states`	VARCHAR ( 1023 ),
	`fields`	VARCHAR ( 1023 ),
	`config`    VARCHAR(2047) DEFAULT '{}' NOT NULL,
	`flag`      INTEGER DEFAULT 1 NOT NULL,
	`create_time`	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(`meta_type`,`meta_key`,`version`),
    UNIQUE KEY `meta_id_IDX` (`id`) USING BTREE,
    KEY `meta_create_time_IDX` (`create_time`) USING BTREE
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

create TABLE `relation` (
    `id`        int(11) NOT NULL AUTO_INCREMENT,
	`from_meta`	VARCHAR ( 255 ) NOT NULL,
	`to_meta`	VARCHAR ( 255 ) NOT NULL,
	`settings`  VARCHAR ( 2047 ) NOT NULL,
	`flag`      INTEGER DEFAULT 1 NOT NULL,
	PRIMARY KEY(`from_meta`,`to_meta`),
    UNIQUE KEY `relation_id_IDX` (`id`) USING BTREE
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE `instances` (
  `meta` varchar(150) NOT NULL,
  `ins_id` bigint(20) unsigned NOT NULL,
  `para` varchar(255) NOT NULL,
  `content` text NOT NULL,
  `context` text DEFAULT NULL,
  `states` text DEFAULT NULL,
  `state_version` int(11) NOT NULL,
  `create_time` datetime NOT NULL,
  `sys_context` text DEFAULT NULL,
  `from_key` varchar(256) NOT NULL COMMENT 'meta|id|para|sta_ver',
  PRIMARY KEY (`meta`,`ins_id`,`para`,`state_version`),
  UNIQUE KEY `instances_un` (`from_key`,`meta`,`ins_id`,`para`),
  KEY `instances_create_time_IDX` (`create_time`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

create TABLE `task` (
	`task_id`	bigint(20) unsigned NOT NULL AUTO_INCREMENT,
	`task_key`	VARCHAR ( 511 ) NOT NULL COMMENT 'meta|id|para|sta_ver',
	`task_type`	TINYINT NOT NULL,
	`task_for`	VARCHAR ( 255 ) NOT NULL,
	`task_state`	TINYINT NOT NULL COMMENT '0: new, 1: down',
	`data`	TEXT NOT NULL,
	`create_time`	DATETIME NOT NULL,
	`execute_time`	DATETIME NOT NULL,
	`retried_times`	SMALLINT NOT NULL,
	UNIQUE KEY `task_un` (`task_key`,`task_type`,`task_for`),
	PRIMARY KEY(`task_id`),
	KEY `task_create_time_IDX` (`create_time`,`task_state`) USING BTREE
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

create TABLE `task_error` (
	`task_id`	bigint(20) unsigned NOT NULL AUTO_INCREMENT,
	`task_key`	VARCHAR ( 511 ) NOT NULL,
	`task_type`	TINYINT NOT NULL,
	`task_for`	VARCHAR ( 255 ) NOT NULL,
	`data`	TEXT NOT NULL,
	`create_time`	DATETIME NOT NULL,
	`msg`	VARCHAR ( 255 ) NOT NULL,
	UNIQUE KEY `task_un` (`task_key`,`task_type`,`task_for`),
    KEY `task_for_IDX` (`task_for`) USING BTREE,
	PRIMARY KEY(`task_id`)
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
