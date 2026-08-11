import { pgTable, uuid } from "drizzle-orm/pg-core";
import { AUTH_SCHEMA } from "./schemas";

/**
 * Reference to auth.users (managed by Supabase).
 * Only defined here for foreign key purposes — Drizzle won't create or migrate this table.
 *
 * @see https://supabase.com/docs/guides/auth/managing-user-data
 */
const authUsers = AUTH_SCHEMA.table("users", {
    id: uuid("id").primaryKey(),
});

/**
 * Public profiles table — extends auth.users with app-specific data.
 * Linked 1:1 via the user's UUID. Cascade delete ensures cleanup when a user is removed.
 *
 * Requires the following to be applied manually in migrations:
 * - GRANT SELECT ON public.profiles TO anon;
 * - GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO authenticated;
 * - GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO service_role;
 * - ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
 * - Trigger: on_auth_user_created → public.handle_new_user()
 */
export const profiles = pgTable("profiles", {
    id: uuid("id")
        .primaryKey()
        .references(() => authUsers.id, { onDelete: "cascade" }),
});
