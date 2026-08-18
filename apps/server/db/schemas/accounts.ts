import { sql } from "drizzle-orm";
import { index, numeric, pgEnum, pgPolicy, snakeCase, uuid, varchar } from "drizzle-orm/pg-core";
import { profiles } from "./profiles";

/**
 * Enum for supported account types.
 *
 * - `CHECKING` — checking/current account
 * - `SAVING` — savings account
 * - `CASH` — physical cash
 * - `DIGITAL_WALLET` — digital wallet
 */
export const accountType = pgEnum("account_type", ["CHECKING", "SAVING", "CASH", "DIGITAL_WALLET"]);

/**
 * Financial accounts belonging to a profile.
 *
 * Each user can have multiple accounts (bank, cash, credit card, etc.).
 * Balance is stored as a fixed-precision decimal string to avoid floating-point issues.
 *
 * Users must create at least one account before recording transactions.
 *
 * ## Relations
 * - `accounts.profile_id` → `profiles.id` (many-to-one)
 * - `accounts.id` ← `transactions.account_id` (one-to-many)
 */
export const accounts = snakeCase.table.withRLS(
    "accounts",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid().primaryKey(),

        /**
         * Owner profile reference.
         *
         * Cascades on both delete and update to maintain
         * referential integrity with `profiles`.
         */
        profileId: uuid()
            .notNull()
            .references(() => profiles.id, {
                onDelete: "cascade",
                onUpdate: "cascade",
            }),

        /**
         * User-defined display name.
         *
         * Example: "Bancolombia Ahorros", "Nequi", "Efectivo".
         */
        name: varchar({ length: 255 }),

        /**
         * Account classification.
         *
         * Defaults to `SAVING`. See `accountType` enum for all options.
         */
        type: accountType().notNull().default("SAVING"),

        /**
         * Current balance — precision 18, scale 2.
         *
         * Stored as string to avoid floating-point rounding.
         * Example: "1234567890123456.78".
         */
        balance: numeric({ precision: 18, scale: 2, mode: "string" }).notNull().default("0.00"),

        /**
         * ISO 4217 currency code.
         *
         * Examples: "COP", "USD", "EUR".
         */
        currencyCode: varchar({ length: 3 }).notNull(),
    },
    (table) => [
        index().on(table.profileId),
        pgPolicy("Allow: Users can manage their own accounts", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`(select auth.uid()) = profile_id`,
            withCheck: sql`(select auth.uid()) = profile_id`,
        }),
    ],
);
