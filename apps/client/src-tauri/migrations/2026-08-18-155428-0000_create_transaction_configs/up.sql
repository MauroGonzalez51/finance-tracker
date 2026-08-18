CREATE TABLE `transaction_configs`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`profile_id` TEXT NOT NULL,
	`transaction_id` TEXT NOT NULL,
	`payment_method_id` TEXT,
	`installments` INTEGER NOT NULL DEFAULT 1,
	`credit_limit` TEXT,
	`interest_rate` TEXT,
	`billing_cycle_day` INTEGER,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY (`transaction_id`) REFERENCES `transactions`(`id`) ON DELETE CASCADE ON UPDATE CASCADE,
	FOREIGN KEY (`payment_method_id`) REFERENCES `payment_methods`(`id`) ON DELETE SET NULL ON UPDATE CASCADE
);

CREATE INDEX `transaction_configs_transaction_id_index` ON `transaction_configs` (`transaction_id`);
CREATE INDEX `transaction_configs_payment_method_id_index` ON `transaction_configs` (`payment_method_id`);
