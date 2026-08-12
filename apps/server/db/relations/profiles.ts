import { defineRelations } from "drizzle-orm";
import { accounts, categories, profiles, transactions } from "../schemas";

/**
 * Relations for the `profiles` table.
 *
 * - Each profile has many accounts.
 * - Each profile has many categories.
 * - Each profile has many transactions.
 */
export const profilesRelations = defineRelations(
    { profiles, accounts, categories, transactions },
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
        },
    }),
);
