CREATE TABLE `categories`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`profile_id` TEXT,
	`code` TEXT,
	`name` TEXT,
	`parent_id` TEXT,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY (`parent_id`) REFERENCES `categories`(`id`)
);

CREATE INDEX `categories_profile_id_index` ON `categories` (`profile_id`);
CREATE INDEX `categories_parent_id_index` ON `categories` (`parent_id`);
