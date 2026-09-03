import { sql } from "drizzle-orm";
import {
    index,
    integer,
    numeric,
    pgEnum,
    pgPolicy,
    snakeCase,
    text,
    timestamp,
    uuid,
} from "drizzle-orm/pg-core";
import { accounts } from "./accounts";
import { categories } from "./categories";
import { paymentMethods } from "./payment-methods";
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
 * Each transaction belongs to exactly one profile, one account, one payment method,
 * and optionally one category. Amount is stored as a fixed-precision decimal
 * (always positive — type determines direction).
 *
 * The `profile_id` is denormalized for RLS efficiency so row-level policies
 * can check ownership without joining `accounts`.
 *
 * ## Relations
 * - `transactions.profile_id` → `profiles.id` (many-to-one)
 * - `transactions.account_id` → `accounts.id` (many-to-one)
 * - `transactions.payment_method_id` → `payment_methods.id` (many-to-one)
 * - `transactions.category_id` → `categories.id` (many-to-one, nullable)
 * - `transactions.id` ← `transaction_configs.transaction_id` (one-to-one)
 */
export const transactions = snakeCase.table.withRLS(
    "transactions",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid().primaryKey(),

        /**
         * Owner profile reference.
         *
         * Denormalized for RLS efficiency — allows row-level policies
         * to check ownership without joining `accounts`.
         */
        profileId: uuid()
            .notNull()
            .references(() => profiles.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * Target account for this transaction.
         *
         * Set to null if the referenced account is deleted (`onDelete: "set null"`).
         */
        accountId: uuid().references(() => accounts.id, {
            onDelete: "set null",
            onUpdate: "cascade",
        }),

        /**
         * Payment method used for this transaction.
         *
         * Set to null if the referenced payment method is deleted.
         */
        paymentMethodId: uuid().references(() => paymentMethods.id, {
            onDelete: "set null",
            onUpdate: "cascade",
        }),

        /**
         * Optional category classification.
         *
         * Set to null if the referenced category is deleted (`onDelete: "set null"`).
         * Allows unclassified transactions.
         */
        categoryId: uuid().references(() => categories.id, {
            onDelete: "set null",
            onUpdate: "cascade",
        }),

        /**
         * Transaction direction/type.
         *
         * Determines whether the amount is added to or
         * subtracted from the account balance.
         */
        type: transactionType().notNull(),

        /**
         * Transaction amount — precision 18, scale 2.
         *
         * Always positive. The `type` field determines
         * whether it's income or expense.
         */
        amount: numeric({ precision: 18, scale: 2, mode: "string" }).notNull(),

        /**
         * Optional user notes or description.
         *
         * Free-text field for additional context
         * (e.g. "Almuerzo en Crepes & Waffles").
         */
        notes: text(),

        /**
         * When the transaction occurred.
         *
         * User-specified date with timezone.
         * Defaults to the current timestamp if not provided.
         */
        date: timestamp({ withTimezone: true }).notNull().defaultNow(),

        /**
         * Record creation timestamp.
         *
         * Automatically set on insert. Useful for auditing
         * and sorting by insertion order.
         */
        createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow(),
    },
    (table) => [
        index().on(table.profileId),
        index().on(table.accountId),
        index().on(table.paymentMethodId),
        index().on(table.categoryId),
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

/**
 * Per-transaction payment configuration (installments, interest, billing).
 *
 * This table stores the **specific** configuration for a single transaction,
 * particularly relevant for credit card payments. It mirrors the structure of
 * `payment_method_configs` but is scoped to one transaction.
 *
 * ## Workflow
 * 1. User creates a transaction with a credit card payment method.
 * 2. User configures installments, interest rate, and billing cycle for that transaction.
 * 3. This config is stored here regardless of whether the user "saves" it.
 * 4. If the user chooses to **save** the config for reuse, a copy is written
 *    to `payment_method_configs` as the default for that payment method.
 *
 * ## Key Difference from `payment_method_configs`
 * - `transaction_configs` — per-transaction, always created, includes `installments`
 * - `payment_method_configs` — per-payment-method template, user-initiated save, no installments
 *
 * ## Relations
 * - `transaction_configs.transaction_id` → `transactions.id` (one-to-one)
 * - `transaction_configs.payment_method_id` → `payment_methods.id` (many-to-one)
 * - `transaction_configs.profile_id` → `profiles.id` (many-to-one, for RLS)
 */
export const transactionConfig = snakeCase.table.withRLS(
    "transaction_configs",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid().primaryKey(),

        /**
         * Owner profile reference (denormalized for RLS efficiency).
         */
        profileId: uuid()
            .notNull()
            .references(() => profiles.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * The transaction this config belongs to.
         *
         * One-to-one relationship — each transaction has at most one config.
         */
        transactionId: uuid()
            .notNull()
            .unique()
            .references(() => transactions.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * The payment method this config relates to.
         *
         * Stored for quick lookups and to maintain context even if
         * the transaction's payment method reference changes.
         */
        paymentMethodId: uuid().references(() => paymentMethods.id, {
            onDelete: "set null",
            onUpdate: "cascade",
        }),

        /**
         * Number of installments for this transaction.
         *
         * Defaults to 1 (single payment). For credit card purchases,
         * users can split into multiple installments (e.g., 12, 24, 36 cuotas).
         */
        installments: integer().notNull().default(1),

        /**
         * Credit limit snapshot at time of transaction.
         *
         * Precision 18, scale 2. Captures the credit limit that was
         * applicable when this transaction was made.
         */
        creditLimit: numeric({ precision: 18, scale: 2, mode: "string" }),

        /**
         * Interest rate applicable to this transaction.
         *
         * Precision 5, scale 2. May differ from the payment method's
         * default rate for promotional or negotiated rates.
         */
        interestRate: numeric({ precision: 5, scale: 2, mode: "string" }),

        /**
         * Billing cycle day applicable to this transaction.
         *
         * Day of the month when this installment is billed.
         */
        billingCycleDay: integer(),
    },
    (table) => [
        index().on(table.transactionId),
        index().on(table.paymentMethodId),
        pgPolicy("Allow: Users can manage their own transaction configs", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`
                (select auth.uid()) = transaction_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.transactions
                    WHERE transactions.id = transaction_configs.transaction_id
                    AND transactions.profile_id = (select auth.uid())
                )
            `,
            withCheck: sql`
                (select auth.uid()) = transaction_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.transactions
                    WHERE transactions.id = transaction_configs.transaction_id
                    AND transactions.profile_id = (select auth.uid())
                )
            `,
        }),
    ],
);
