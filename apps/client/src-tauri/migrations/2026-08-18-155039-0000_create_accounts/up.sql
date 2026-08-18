CREATE TABLE `accounts`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`profile_id` TEXT NOT NULL,
	`name` TEXT,
	`type` TEXT NOT NULL DEFAULT 'SAVING' CHECK (`type` IN ('CHECKING', 'SAVING', 'CASH', 'DIGITAL_WALLET')),
	`balance` TEXT NOT NULL DEFAULT '0.00',
	`currency_code` TEXT NOT NULL,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX `accounts_profile_id_index` ON `accounts` (`profile_id`);
