import { defineRelations } from "drizzle-orm";
import { accounts, profiles, transactions } from "../schemas";

/**
 * Relations for the `accounts` table.
 *
 * - Each account belongs to one profile.
 * - Each account has many transactions.
 */
export const accountsRelations = defineRelations(
    { accounts, profiles, transactions },
    (r) => ({
        accounts: {
            profile: r.one.profiles({
                from: r.accounts.profileId,
                to: r.profiles.id,
            }),
            transactions: r.many.transactions({
                from: r.accounts.id,
                to: r.transactions.accountId,
            }),
        },
    }),
);
