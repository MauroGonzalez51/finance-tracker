import { sql } from "drizzle-orm";
import { pgPolicy, pgTable, uuid } from "drizzle-orm/pg-core";
import { AUTH_SCHEMA } from "./auth";

/**
 * Reference to `auth.users` (managed by Supabase Auth).
 * Only defined here for foreign key purposes — Drizzle won't create or migrate this table.
 *
 * @see https://supabase.com/docs/guides/auth/managing-user-data
 */
export const authUsers = AUTH_SCHEMA.table("users", {
    id: uuid("id").primaryKey(),
});

/**
 * Public profiles table — extends `auth.users` with app-specific data.
 *
 * - Linked 1:1 via the user's UUID from Supabase Auth.
 * - Cascade delete ensures cleanup when a user is removed from auth.
 * - Created automatically via `handle_new_user()` trigger on signup.
 *
 * ## RLS & Grants (apply manually or via SQL snippet)
 *
 * ```sql
 * ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
 * GRANT SELECT ON public.profiles TO anon;
 * GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO authenticated;
 * GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO service_role;
 * ```
 *
 * ## Relations
 * - `profiles.id` ← `accounts.profile_id` (one-to-many)
 * - `profiles.id` ← `categories.profile_id` (one-to-many)
 * - `profiles.id` ← `transactions.profile_id` (one-to-many)
 */
export const profiles = pgTable.withRLS(
    "profiles",
    {
        /**
         * UUID matching the user's `auth.users.id`.
         *
         * Acts as both PK and FK — ensures a 1:1 relationship
         * between the profile and the Supabase Auth user.
         */
        id: uuid("id")
            .primaryKey()
            .references(() => authUsers.id, { onDelete: "cascade" }),
    },
    () => [
        pgPolicy("Allow: Read-Only for unauthenticated", {
            as: "permissive",
            for: "select",
            to: "anon",
            using: sql`true`,
        }),

        pgPolicy("Allow: Authenticated users to manage their own profile", {
            as: "permissive",
            for: "all",
            to: "authenticated",
            using: sql`(select auth.uid()) = id`,
            withCheck: sql`(select auth.uid()) = id`,
        }),
    ],
);

// TRIGGER FOR NEW USER

// CREATE FUNCTION public.handle_new_user()
// RETURNS trigger
// LANGUAGE plpgsql
// SECURITY DEFINER SET search_path = ''
// AS $$
// BEGIN
//   INSERT INTO public.profiles (id)
//   VALUES (new.id);
//   RETURN new;
// END;
// $$;

// -- Revoke public execution — this function is only meant to be called by the trigger,
// -- not exposed via PostgREST API (/rest/v1/rpc/handle_new_user).
// REVOKE EXECUTE ON FUNCTION public.handle_new_user() FROM public, anon, authenticated;

// -- Fire the function every time a user is created in auth.users.
// CREATE TRIGGER on_auth_user_created
// AFTER INSERT ON auth.users
// FOR EACH ROW EXECUTE PROCEDURE public.handle_new_user();
