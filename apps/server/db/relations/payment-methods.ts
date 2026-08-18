import { defineRelations } from "drizzle-orm";
import {
    accounts,
    paymentMethodConfig,
    paymentMethods,
    transactionConfig,
    transactions,
} from "../schemas";

/**
 * Relations for the `payment_methods` table.
 *
 * - Each payment method belongs to one account.
 * - Each payment method has at most one saved config (lazy, user-initiated).
 * - Each payment method has many transactions.
 * - Each payment method has many transaction configs.
 *
 * Relations for the `payment_method_configs` table.
 *
 * - Each config belongs to one payment method (one-to-one).
 */
export const paymentMethodsRelations = defineRelations(
    { paymentMethods, paymentMethodConfig, accounts, transactions, transactionConfig },
    (r) => ({
        paymentMethods: {
            account: r.one.accounts({
                from: r.paymentMethods.accountId,
                to: r.accounts.id,
            }),
            config: r.one.paymentMethodConfig({
                from: r.paymentMethods.id,
                to: r.paymentMethodConfig.paymentMethodId,
            }),
            transactions: r.many.transactions({
                from: r.paymentMethods.id,
                to: r.transactions.paymentMethodId,
            }),
            transactionConfigs: r.many.transactionConfig({
                from: r.paymentMethods.id,
                to: r.transactionConfig.paymentMethodId,
            }),
        },
        paymentMethodConfig: {
            paymentMethod: r.one.paymentMethods({
                from: r.paymentMethodConfig.paymentMethodId,
                to: r.paymentMethods.id,
            }),
        },
    }),
);
