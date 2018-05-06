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

CREATE UNIQUE INDEX `idx_thing_define` ON `thing_defines` (
	`key`,
	`version`	DESC
);