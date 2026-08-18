CREATE TABLE `payment_methods`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`account_id` TEXT NOT NULL,
	`type` TEXT NOT NULL DEFAULT 'CASH_PAYMENT' CHECK (`type` IN ('DEBIT_CARD', 'CREDIT_CARD', 'TRANSFER', 'CASH_PAYMENT')),
	`card_number_last4` TEXT,
	`card_holder` TEXT,
	`is_active` BOOL NOT NULL DEFAULT 1,
	`created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	FOREIGN KEY (`account_id`) REFERENCES `accounts`(`id`) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE INDEX `payment_methods_account_id_index` ON `payment_methods` (`account_id`);
