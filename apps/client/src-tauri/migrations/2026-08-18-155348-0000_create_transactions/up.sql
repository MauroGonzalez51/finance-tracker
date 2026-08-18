CREATE TABLE `transactions`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`profile_id` TEXT NOT NULL,
	`account_id` TEXT,
	`payment_method_id` TEXT,
	`category_id` TEXT,
	`type` TEXT NOT NULL CHECK (`type` IN ('INITIAL_BALANCE', 'DEPOSIT', 'WITHDRAW')),
	`amount` TEXT NOT NULL,
	`notes` TEXT,
	`date` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	`created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`) ON DELETE SET NULL ON UPDATE CASCADE,
	FOREIGN KEY (`payment_method_id`) REFERENCES `payment_methods`(`id`) ON DELETE SET NULL ON UPDATE CASCADE,
	FOREIGN KEY (`category_id`) REFERENCES `categories`(`id`) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE INDEX `transactions_profile_id_index` ON `transactions` (`profile_id`);
CREATE INDEX `transactions_account_id_index` ON `transactions` (`account_id`);
CREATE INDEX `transactions_payment_method_id_index` ON `transactions` (`payment_method_id`);
CREATE INDEX `transactions_category_id_index` ON `transactions` (`category_id`);
