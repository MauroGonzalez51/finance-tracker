import type { AnyPgColumn } from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm";
import { index, pgPolicy, pgTable, uuid, varchar } from "drizzle-orm/pg-core";
import { profiles } from "./profiles";

/**
 * Transaction categories with optional hierarchical nesting.
 *
 * Categories are per-profile and support a parent→child tree structure
 * via `parent_id` self-reference (adjacency list pattern).
 *
 * ## Examples
 * - "Food" (parent) → "Groceries", "Restaurants" (children)
 * - "Transport" (parent) → "Uber", "Gas" (children)
 *
 * ## Relations
 * - `categories.profile_id` → `profiles.id` (many-to-one)
 * - `categories.parent_id` → `categories.id` (self-referencing, many-to-one)
 * - `categories.id` ← `transactions.category_id` (one-to-many)
 */
export const categories = pgTable.withRLS(
    "categories",
    {
        /**
         * Auto-generated UUID primary key.
         */
        id: uuid("id").primaryKey(),

        /**
         * Owner profile reference.
         *
         * Nullable — system-wide default categories have no owner.
         * Cascades on delete/update for user-owned categories.
         */
        profileId: uuid("profile_id").references(() => profiles.id, {
            onDelete: "cascade",
            onUpdate: "cascade",
        }),

        /**
         * Machine-readable code for system categories.
         *
         * Used instead of `name` for internationalization (i18n).
         * Example: "food", "transport", "entertainment".
         */
        code: varchar("code", { length: 255 }),

        /**
         * User-defined display name.
         *
         * Example: "Groceries", "Salary", "Uber".
         */
        name: varchar("name", { length: 255 }),

        /**
         * Parent category for nesting.
         *
         * Null means top-level category. Self-references `categories.id`
         * to build the adjacency list tree.
         */
        parentId: uuid("parent_id").references((): AnyPgColumn => categories.id),
    },
    (table) => [
        index("categories_profile_id_index").on(table.profileId),
        index("categories_parent_id_index").on(table.parentId),

        pgPolicy("Allow: Anon read system categories", {
            as: "permissive",
            for: "select",
            to: "anon",
            using: sql`profile_id IS NULL`,
        }),

        pgPolicy("Allow: Authenticated read system and own categories", {
            as: "permissive",
            for: "select",
            to: "authenticated",
            using: sql`profile_id IS NULL OR profile_id = (select auth.uid())`,
        }),

        pgPolicy("Allow: Authenticated create own categories", {
            as: "permissive",
            for: "insert",
            to: "authenticated",
            withCheck: sql`profile_id = (select auth.uid())`,
        }),

        pgPolicy("Allow: Authenticated update own categories", {
            as: "permissive",
            for: "update",
            to: "authenticated",
            using: sql`profile_id = (select auth.uid())`,
            withCheck: sql`profile_id = (select auth.uid())`,
        }),

        pgPolicy("Allow: Authenticated delete own categories", {
            as: "permissive",
            for: "delete",
            to: "authenticated",
            using: sql`profile_id = (select auth.uid())`,
        }),
    ],
);
