import { defineRelations } from "drizzle-orm";
import { categories, profiles, transactions } from "../schemas";

/**
 * Relations for the `categories` table.
 *
 * - Each category optionally belongs to one profile.
 * - Self-referencing: each category can have a parent and many children.
 * - Each category can be referenced by many transactions.
 */
export const categoriesRelations = defineRelations({ categories, profiles, transactions }, (r) => ({
    categories: {
        profile: r.one.profiles({
            from: r.categories.profileId,
            to: r.profiles.id,
        }),
        parent: r.one.categories({
            alias: "parent",
            from: r.categories.parentId,
            to: r.categories.id,
        }),
        children: r.many.categories({
            alias: "children",
            from: r.categories.id,
            to: r.categories.parentId,
        }),
        transactions: r.many.transactions({
            from: r.categories.id,
            to: r.transactions.categoryId,
        }),
    },
}));
