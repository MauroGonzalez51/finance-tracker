CREATE TABLE `payment_method_configs`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`profile_id` TEXT NOT NULL,
	`payment_method_id` TEXT NOT NULL,
	`credit_limit` TEXT,
	`interest_rate` TEXT,
	`billing_cycle_day` INTEGER,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY (`payment_method_id`) REFERENCES `payment_methods`(`id`) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX `payment_method_configs_payment_method_id_index` ON `payment_method_configs` (`payment_method_id`);
