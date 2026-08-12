import { numeric, pgEnum, pgTable, uuid, varchar } from "drizzle-orm/pg-core";
import { profiles } from "./profiles";

/**
 * Enum for supported account types.
 *
 * - `DEFAULT` — default account -> also created when a new user is created
 * - `CHECKING` — checking/current account
 * - `SAVING` — savings account
 * - `CREDIT_CARD` — credit card
 * - `CASH` — physical cash
 * - `INVESTMENT` — investment/brokerage account
 */
export const accountType = pgEnum("account_type", [
    "DEFAULT",
    "CHECKING",
    "SAVING",
    "CREDIT_CARD",
    "CASH",
    "INVESTMENT",
]);

/**
 * Financial accounts belonging to a profile.
 *
 * Each user can have multiple accounts (bank, cash, credit card, etc.).
 * Balance is stored as a fixed-precision decimal string to avoid floating-point issues.
 *
 * ## Relations
 * - `accounts.profile_id` → `profiles.id` (many-to-one)
 * - `accounts.id` ← `transactions.account_id` (one-to-many)
 */
export const accounts = pgTable("accounts", {
    /** Auto-generated UUID primary key */
    id: uuid("id").primaryKey(),

    /** Owner profile reference — cascades on delete/update */
    profileId: uuid("profile_id")
        .notNull()
        .references(() => profiles.id, {
            onDelete: "cascade",
            onUpdate: "cascade",
        }),

    /** User-defined display name (e.g. "Bancolombia Ahorros") */
    name: varchar("name", { length: 255 }),

    /** Account classification */
    type: accountType("type").notNull().default("SAVING"),

    /** Current balance — precision 18, scale 2 (e.g. "1234567890123456.78") */
    balance: numeric("balance", { precision: 18, scale: 2, mode: "string" })
        .notNull()
        .default("0.00"),

    /** ISO 4217 currency code (e.g. "COP", "USD") */
    currencyCode: varchar("currency_code", { length: 3 }).notNull(),
});
