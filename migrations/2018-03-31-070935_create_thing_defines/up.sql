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