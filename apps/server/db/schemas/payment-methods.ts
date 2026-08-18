import { sql } from "drizzle-orm";
import {
    boolean,
    index,
    integer,
    numeric,
    pgEnum,
    pgPolicy,
    snakeCase,
    timestamp,
    uuid,
    varchar,
} from "drizzle-orm/pg-core";
import { accounts } from "./accounts";
import { profiles } from "./profiles";

/**
 * Enum for supported payment method types.
 *
 * - `DEBIT_CARD` — linked debit card
 * - `CREDIT_CARD` — credit card with revolving credit
 * - `TRANSFER` — bank transfer / wire
 * - `CASH_PAYMENT` — physical cash payment
 */
export const paymentMethodType = pgEnum("payment_method_type", [
    "DEBIT_CARD",
    "CREDIT_CARD",
    "TRANSFER",
    "CASH_PAYMENT",
]);

/**
 * Payment methods linked to a specific account.
 *
 * Each account can have multiple payment methods (e.g., a debit card and a
 * credit card both tied to the same bank account). Payment methods are used
 * to classify how a transaction was made.
 *
 * Configuration (credit limit, interest rate, etc.) is **lazy-loaded** — it is
 * NOT populated when the payment method is created. Instead, users fill in the
 * config via `payment_method_configs` when they choose to.
 *
 * ## Relations
 * - `payment_methods.account_id` → `accounts.id` (many-to-one)
 * - `payment_methods.id` ← `payment_method_configs.payment_method_id` (one-to-one, lazy)
 * - `payment_methods.id` ← `transactions.payment_method_id` (one-to-many)
 * - `payment_methods.id` ← `transaction_configs.payment_method_id` (one-to-many)
 */
export const paymentMethods = snakeCase.table.withRLS(
    "payment_methods",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid().primaryKey(),

        /**
         * Parent account reference.
         *
         * The account this payment method belongs to.
         * Cascades on delete/update for referential integrity.
         */
        accountId: uuid()
            .notNull()
            .references(() => accounts.id, { onDelete: "cascade", onUpdate: "cascade" }),

        /**
         * Payment method classification.
         *
         * Defaults to `CASH_PAYMENT`. See `paymentMethodType` enum for all options.
         */
        type: paymentMethodType().notNull().default("CASH_PAYMENT"),

        /**
         * Last 4 digits of the card number (if applicable).
         *
         * Only relevant for `DEBIT_CARD` and `CREDIT_CARD` types.
         * Example: "4532".
         */
        cardNumberLast4: varchar({ length: 4 }),

        /**
         * Cardholder display name (if applicable).
         *
         * Example: "JUAN P GONZALEZ".
         */
        cardHolder: varchar({ length: 32 }),

        /**
         * Whether this payment method is currently active.
         *
         * Inactive methods are hidden from new transaction forms
         * but preserved for historical records.
         */
        isActive: boolean().notNull().default(true),

        /**
         * Record creation timestamp.
         */
        createdAt: timestamp({ withTimezone: true }).notNull().defaultNow(),
    },
    (table) => [
        index().on(table.accountId),
        pgPolicy("Allow: users can manage payment methods on their accounts", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`
                (select auth.uid()) = (
                    SELECT accounts.profile_id
                    FROM accounts
                    WHERE accounts.id = account_id
                )
            `,
            withCheck: sql`
                (select auth.uid()) = (
                    SELECT accounts.profile_id
                    FROM accounts
                    WHERE accounts.id = account_id
                )
            `,
        }),
    ],
);

/**
 * Saved configuration for a payment method (lazy — user-initiated).
 *
 * This table stores **reusable** credit/debit configuration that applies
 * to all future transactions using the associated payment method.
 * It is NOT created automatically — the user explicitly saves a config
 * when they want it to persist as a template.
 *
 * When a user creates a transaction and configures installments/interest,
 * they can choose to "save" that config here so it auto-fills next time.
 *
 * ## Relations
 * - `payment_method_configs.payment_method_id` → `payment_methods.id` (one-to-one)
 * - `payment_method_configs.profile_id` → `profiles.id` (many-to-one, for RLS)
 */
export const paymentMethodConfig = snakeCase.table.withRLS(
    "payment_method_configs",
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
         * The payment method this config belongs to.
         *
         * One-to-one relationship — each payment method has at most one saved config.
         */
        paymentMethodId: uuid()
            .notNull()
            .references(() => paymentMethods.id, {
                onDelete: "cascade",
                onUpdate: "cascade",
            }),

        /**
         * Maximum credit limit for credit cards.
         *
         * Precision 18, scale 2. Example: "5000000.00" (5M COP).
         */
        creditLimit: numeric({ precision: 18, scale: 2, mode: "string" }),

        /**
         * Monthly interest rate percentage.
         *
         * Precision 5, scale 2. Example: "2.10" (2.10% monthly).
         */
        interestRate: numeric({ precision: 5, scale: 2, mode: "string" }),

        /**
         * Day of the month the billing cycle closes.
         *
         * Example: 15 means the cycle closes on the 15th.
         */
        billingCycleDay: integer(),
    },
    (table) => [
        index().on(table.paymentMethodId),
        pgPolicy("Allow: Users to manage their own payment methods configuration", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`
                (select auth.uid()) = payment_method_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.payment_methods
                    WHERE payment_methods.id = payment_method_configs.payment_method_id
                    AND payment_methods.account_id IN (
                        SELECT id FROM public.accounts WHERE accounts.profile_id = 
                            (select auth.uid())
                    )
                )
            `,
            withCheck: sql`
                (select auth.uid()) = payment_method_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.payment_methods
                    WHERE payment_methods.id = payment_method_configs.payment_method_id
                    AND payment_methods.account_id IN (
                        SELECT id FROM public.accounts WHERE accounts.profile_id = 
                            (select auth.uid())
                    )
                )
            `,
        }),
    ],
);
