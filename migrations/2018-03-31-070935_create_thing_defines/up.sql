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
	`id`	BLOB NOT NULL UNIQUE,
	`thing`	TEXT NOT NULL,
	`version`	INTEGER NOT NULL,
	`content`	TEXT NOT NULL,
	`context`	TEXT,
	`status`	TEXT,
	`status_version`	INTEGER NOT NULL,
	`from_thing`	TEXT,
	`from_version`	INTEGER,
	`from_status_version`	INTEGER,
	`execute_time`	DATETIME NOT NULL,
	`create_time`	DATETIME NOT NULL,
	PRIMARY KEY(`id`)
);