import { defineRelations } from "drizzle-orm";
import { accounts, categories, profiles, transactions } from "../schemas";

/**
 * Relations for the `transactions` table.
 *
 * - Each transaction belongs to one profile.
 * - Each transaction belongs to one account.
 * - Each transaction optionally belongs to one category.
 */
export const transactionsRelations = defineRelations(
    { transactions, profiles, accounts, categories },
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
            category: r.one.categories({
                from: r.transactions.categoryId,
                to: r.categories.id,
            }),
        },
    }),
);
