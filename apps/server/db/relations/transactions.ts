import { defineRelations } from "drizzle-orm";
import {
    accounts,
    categories,
    paymentMethods,
    profiles,
    transactionConfig,
    transactions,
} from "../schemas";

/**
 * Relations for the `transactions` table.
 *
 * - Each transaction belongs to one profile.
 * - Each transaction belongs to one account.
 * - Each transaction belongs to one payment method.
 * - Each transaction optionally belongs to one category.
 * - Each transaction has at most one transaction config.
 *
 * Relations for the `transaction_configs` table.
 *
 * - Each transaction config belongs to one transaction (one-to-one).
 * - Each transaction config references one payment method.
 * - Each transaction config belongs to one profile (for RLS).
 */
export const transactionsRelations = defineRelations(
    { transactions, transactionConfig, profiles, accounts, categories, paymentMethods },
    (r) => ({
        transactions: {
            profile: r.one.profiles({
                from: r.transactions.profileId,
                to: r.profiles.id,
            }),
            account: r.one.accounts({
                from: r.transactions.accountId,
                to: r.accounts.id,
            }),
            paymentMethod: r.one.paymentMethods({
                from: r.transactions.paymentMethodId,
                to: r.paymentMethods.id,
            }),
            category: r.one.categories({
                from: r.transactions.categoryId,
                to: r.categories.id,
            }),
            config: r.one.transactionConfig({
                from: r.transactions.id,
                to: r.transactionConfig.transactionId,
            }),
        },
        transactionConfig: {
            transaction: r.one.transactions({
                from: r.transactionConfig.transactionId,
                to: r.transactions.id,
            }),
            paymentMethod: r.one.paymentMethods({
                from: r.transactionConfig.paymentMethodId,
                to: r.paymentMethods.id,
            }),
            profile: r.one.profiles({
                from: r.transactionConfig.profileId,
                to: r.profiles.id,
            }),
        },
    }),
);
