import { sql } from "drizzle-orm";
import {
    index,
    numeric,
    pgEnum,
    pgPolicy,
    pgTable,
    text,
    timestamp,
    uuid,
} from "drizzle-orm/pg-core";
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
export const transactions = pgTable.withRLS(
    "transactions",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid("id").primaryKey(),

        /**
         * Owner profile reference.
         *
         * Denormalized for RLS efficiency — allows row-level policies
         * to check ownership without joining `accounts`.
         */
        profileId: uuid("profile_id")
            .notNull()
            .references(() => profiles.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * Target account for this transaction.
         *
         * Cascades on delete/update to keep consistency
         * when an account is removed.
         */
        accountId: uuid("account_id")
            .notNull()
            .references(() => accounts.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * Optional category classification.
         *
         * Set to null if the referenced category is deleted (`onDelete: "set null"`).
         * Allows unclassified transactions.
         */
        categoryId: uuid("category_id").references(() => categories.id, {
            onDelete: "set null",
            onUpdate: "cascade",
        }),

        /**
         * Transaction direction/type.
         *
         * Determines whether the amount is added to or
         * subtracted from the account balance.
         */
        type: transactionType("type").notNull(),

        /**
         * Transaction amount — precision 18, scale 2.
         *
         * Always positive. The `type` field determines
         * whether it's income or expense.
         */
        amount: numeric("amount", { precision: 18, scale: 2, mode: "string" }).notNull(),

        /**
         * Optional user notes or description.
         *
         * Free-text field for additional context
         * (e.g. "Almuerzo en Crepes & Waffles").
         */
        notes: text("notes"),

        /**
         * When the transaction occurred.
         *
         * User-specified date with timezone.
         * Defaults to the current timestamp if not provided.
         */
        date: timestamp("date", { withTimezone: true }).notNull().defaultNow(),

        /**
         * Record creation timestamp.
         *
         * Automatically set on insert. Useful for auditing
         * and sorting by insertion order.
         */
        createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow(),
    },
    (table) => [
        index("transactions_profile_id_index").on(table.profileId),
        index("transactions_account_id_index").on(table.accountId),
        index("transactions_category_id_index").on(table.categoryId),

        pgPolicy("Allow: Users can manage their own transactions", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`
                (select auth.uid()) = transactions.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.accounts
                    WHERE accounts.id = transactions.account_id
                    AND accounts.profile_id = (select auth.uid())
                )
                AND (
                    transactions.category_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM public.categories
                        WHERE categories.id = transactions.category_id
                        AND (categories.profile_id = (select auth.uid()) OR categories.profile_id IS NULL)
                    )
                )`,
            withCheck: sql`
                (select auth.uid()) = transactions.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.accounts
                    WHERE accounts.id = transactions.account_id
                    AND accounts.profile_id = (select auth.uid())
                )
                AND (
                    transactions.category_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM public.categories
                        WHERE categories.id = transactions.category_id
                        AND (categories.profile_id = (select auth.uid()) OR categories.profile_id IS NULL)
                    )
                )`,
        }),
    ],
);
