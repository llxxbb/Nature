-- Your SQL goes here
CREATE TABLE `thing_defines` (
	`key`	VARCHAR ( 255 ) NOT NULL UNIQUE,
	`description`	NVARCHAR ( 255 ),
	`version`	INTEGER NOT NULL,
	`have_states`	INTEGER NOT NULL DEFAULT 0 UNIQUE,
	`states`	VARCHAR ( 1023 ),
	`fields`	TEXT,
	`create_time`	DATETIME DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY(`key`)
);