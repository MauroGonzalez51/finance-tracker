import { numeric, pgEnum, pgTable, text, timestamp, uuid } from "drizzle-orm/pg-core";
import { accounts } from "./accounts";
import { categories } from "./categories";
import { profiles } from "./profiles";

/**
 * Enum for transaction types.
 *
 * - `INITIAL_BALANCE` — opening balance when account is created
 * - `DEPOSIT` — money coming in to the account
 * - `WITHDRAW` — money going out of the account
 */
export const transactionType = pgEnum("transaction_type", [
    "INITIAL_BALANCE",
    "DEPOSIT",
    "WITHDRAW",
]);

/**
 * Individual financial transactions.
 *
 * Each transaction belongs to exactly one profile, one account, and optionally one category.
 * Amount is stored as a fixed-precision decimal (always positive — type determines direction).
 *
 * ## Relations
 * - `transactions.profile_id` → `profiles.id` (many-to-one)
 * - `transactions.account_id` → `accounts.id` (many-to-one)
 * - `transactions.category_id` → `categories.id` (many-to-one, nullable)
 */
export const transactions = pgTable("transactions", {
    /** Auto-generated UUID primary key */
    id: uuid("id").primaryKey(),

    /** Owner profile — denormalized for RLS efficiency */
    profileId: uuid("profile_id")
        .notNull()
        .references(() => profiles.id, { onDelete: "cascade", onUpdate: "cascade" }),

    /** Target account for this transaction */
    accountId: uuid("account_id")
        .notNull()
        .references(() => accounts.id, { onDelete: "cascade", onUpdate: "cascade" }),

    /** Optional category classification — set null if category is deleted */
    categoryId: uuid("category_id").references(() => categories.id, {
        onDelete: "set null",
        onUpdate: "cascade",
    }),

    /** Transaction direction/type */
    type: transactionType("type").notNull(),

    /** Transaction amount (always positive, precision 18 scale 2) */
    amount: numeric("amount", { precision: 18, scale: 2, mode: "string" }).notNull(),

    /** Optional user notes/description */
    notes: text("notes"),

    /** When the transaction occurred (user-specified) */
    date: timestamp("date", { withTimezone: true }).notNull().defaultNow(),

    /** Record creation timestamp */
    createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow(),
});
