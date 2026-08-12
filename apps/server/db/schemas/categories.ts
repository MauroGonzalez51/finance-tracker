import type { AnyPgColumn } from "drizzle-orm/pg-core";
import { index, pgTable, uuid, varchar } from "drizzle-orm/pg-core";
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
export const categories = pgTable(
    "categories",
    {
        /** Auto-generated UUID primary key */
        id: uuid("id").primaryKey(),

        /** Owner profile — nullable for system-wide default categories */
        profileId: uuid("profile_id").references(() => profiles.id, {
            onDelete: "cascade",
            onUpdate: "cascade",
        }),

        /** User-defined display name (e.g. "Groceries", "Salary") */
        name: varchar("name", { length: 255 }),

        /** Parent category for nesting — null means top-level */
        parentId: uuid("parent_id").references((): AnyPgColumn => categories.id),
    },
    (table) => [index("categories_parent_id_index").on(table.parentId)],
);
