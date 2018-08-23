-- Your SQL goes here
CREATE TABLE `thing_defines` (
	`key`	VARCHAR ( 255 ) NOT NULL,
	`description`	NVARCHAR ( 255 ),
	`version`	INTEGER NOT NULL,
	`states`	VARCHAR ( 1023 ),
	`fields`	TEXT,
	`create_time`	DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(`key`,`version`)
);

CREATE TABLE `one_step_flow` (
	`from_thing`	TEXT NOT NULL,
	`from_version`	INTEGER NOT NULL,
	`to_thing`	TEXT NOT NULL,
	`to_version`	INTEGER NOT NULL,
	`exe_protocol`	TEXT NOT NULL,
	`exe_url`	TEXT NOT NULL,
	`selector`	TEXT,
	`group`	TEXT,
	`weight`	INTEGER,
	PRIMARY KEY(`from_thing`,`from_version`,`to_thing`,`to_version`)
);

CREATE TABLE `instances` (
	`id`	BLOB NOT NULL,
	`thing`	TEXT NOT NULL,
	`version`	INTEGER NOT NULL,
	`content`	TEXT NOT NULL,
	`context`	TEXT,
	`status`	TEXT,
	`status_version`	INTEGER NOT NULL,
	`from_thing`	TEXT,
	`from_version`	INTEGER,
	`from_status_version`	INTEGER,
	`event_time`	DATETIME NOT NULL,
	`execute_time`	DATETIME NOT NULL,
	`create_time`	DATETIME NOT NULL,
	PRIMARY KEY(`id`,`thing`,`version`,`status_version`)
);

CREATE TABLE `delivery` (
	`id`	BLOB NOT NULL,
	`thing`	TEXT NOT NULL,
	`data_type`	SMALLINT NOT NULL,
	`data`	TEXT NOT NULL,
	`create_time`	DATETIME NOT NULL,
	`execute_time`	DATETIME NOT NULL,
	`retried_times`	SMALLINT NOT NULL,
	PRIMARY KEY(`id`)
);

CREATE TABLE `delivery_error` (
	`id`	BLOB NOT NULL,
	`thing`	TEXT NOT NULL,
	`data_type`	SMALLINT NOT NULL,
	`data`	TEXT NOT NULL,
	`create_time`	DATETIME NOT NULL,
	`msg`	TEXT NOT NULL,
	PRIMARY KEY(`id`)
);

CREATE TABLE `plan` (
	`upstream`	TEXT NOT NULL,
	`to_biz`	TEXT NOT NULL,
	`to_version`	INTEGER NOT NULL,
	`content`	TEXT NOT NULL,
	`create_time`	DATETIME NOT NULL,
	PRIMARY KEY(`upstream`,`to_version`,`to_biz`)
);