import { defineRelations } from "drizzle-orm";
import {
    accounts,
    categories,
    paymentMethodConfig,
    profiles,
    transactionConfig,
    transactions,
} from "../schemas";

/**
 * Relations for the `profiles` table.
 *
 * - Each profile has many accounts.
 * - Each profile has many categories.
 * - Each profile has many transactions.
 * - Each profile has many payment method configs (saved templates).
 * - Each profile has many transaction configs.
 */
export const profilesRelations = defineRelations(
    { profiles, accounts, categories, transactions, paymentMethodConfig, transactionConfig },
    (r) => ({
        profiles: {
            accounts: r.many.accounts({
                from: r.profiles.id,
                to: r.accounts.profileId,
            }),
            categories: r.many.categories({
                from: r.profiles.id,
                to: r.categories.profileId,
            }),
            transactions: r.many.transactions({
                from: r.profiles.id,
                to: r.transactions.profileId,
            }),
            paymentMethodConfigs: r.many.paymentMethodConfig({
                from: r.profiles.id,
                to: r.paymentMethodConfig.profileId,
            }),
            transactionConfigs: r.many.transactionConfig({
                from: r.profiles.id,
                to: r.transactionConfig.profileId,
            }),
        },
    }),
);
