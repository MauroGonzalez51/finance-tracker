import { sql } from "drizzle-orm";
import { index, numeric, pgEnum, pgPolicy, pgTable, uuid, varchar } from "drizzle-orm/pg-core";
import { profiles } from "./profiles";

/**
 * Enum for supported account types.
 *
 * - `CHECKING` — checking/current account
 * - `SAVING` — savings account
 * - `CREDIT_CARD` — credit card
 * - `CASH` — physical cash
 * - `WALLET` — digital wallet (Nequi, Daviplata, PayPal, etc.)
 * - `INVESTMENT` — investment/brokerage account
 * - `LOAN` — loan or debt (balance starts negative and decreases over time)
 */
export const accountType = pgEnum("account_type", [
    "CHECKING",
    "SAVING",
    "CREDIT_CARD",
    "CASH",
    "WALLET",
    "INVESTMENT",
    "LOAN",
]);

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
export const accounts = pgTable.withRLS(
    "accounts",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid("id").primaryKey(),

        /**
         * Owner profile reference.
         *
         * Cascades on both delete and update to maintain
         * referential integrity with `profiles`.
         */
        profileId: uuid("profile_id")
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
        name: varchar("name", { length: 255 }),

        /**
         * Account classification.
         *
         * Defaults to `SAVING`. See `accountType` enum for all options.
         */
        type: accountType("type").notNull().default("SAVING"),

        /**
         * Current balance — precision 18, scale 2.
         *
         * Stored as string to avoid floating-point rounding.
         * Example: "1234567890123456.78".
         */
        balance: numeric("balance", { precision: 18, scale: 2, mode: "string" })
            .notNull()
            .default("0.00"),

        /**
         * ISO 4217 currency code.
         *
         * Examples: "COP", "USD", "EUR".
         */
        currencyCode: varchar("currency_code", { length: 3 }).notNull(),
    },
    (table) => [
        index("accounts_profile_id_index").on(table.profileId),

        pgPolicy("Allow: Users can manage their own accounts", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`(select auth.uid()) = profile_id`,
            withCheck: sql`(select auth.uid()) = profile_id`,
        }),
    ],
);
