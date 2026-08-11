-- Migration: 0000
--
-- Creates the public.profiles table linked 1:1 with auth.users (managed by Supabase).
-- This table extends user data with app-specific fields.
-- The auth schema reference is only for FK purposes — Supabase owns auth.users.
--
-- @see https://supabase.com/docs/guides/auth/managing-user-data

CREATE SCHEMA IF NOT EXISTS "auth";
--> statement-breakpoint
CREATE TABLE "profiles" (
	"id" uuid PRIMARY KEY NOT NULL
);
--> statement-breakpoint
ALTER TABLE "profiles" ADD CONSTRAINT "profiles_id_users_id_fk" FOREIGN KEY ("id") REFERENCES "auth"."users"("id") ON DELETE cascade ON UPDATE no action;

-- ---------------------------------------------------------------------------
-- Permissions & RLS
-- Grants the minimum privileges each Supabase role needs on public.profiles.
-- - anon: read-only (unauthenticated users can view profiles)
-- - authenticated: full CRUD (logged-in users manage their own profile via RLS)
-- - service_role: full CRUD (server-side admin access, bypasses RLS)
--
-- @see https://supabase.com/docs/guides/auth/managing-user-data#accessing-user-data-via-api
-- ---------------------------------------------------------------------------
GRANT SELECT ON public.profiles TO anon;
GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO authenticated;
GRANT SELECT, INSERT, UPDATE, DELETE ON public.profiles TO service_role;

-- Enable RLS so row-level policies control access instead of table-level grants alone.
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;

-- ---------------------------------------------------------------------------
-- Trigger: auto-create profile on user signup
-- Fires after every new row in auth.users and copies the user's UUID into
-- public.profiles. This ensures every authenticated user has a profile row.
--
-- @see https://supabase.com/docs/guides/auth/managing-user-data
-- ---------------------------------------------------------------------------
CREATE FUNCTION public.handle_new_user()
RETURNS trigger
LANGUAGE plpgsql
SECURITY DEFINER SET search_path = ''
AS $$
BEGIN
  INSERT INTO public.profiles (id)
  VALUES (new.id);
  RETURN new;
END;
$$;

-- Revoke public execution — this function is only meant to be called by the trigger,
-- not exposed via PostgREST API (/rest/v1/rpc/handle_new_user).
REVOKE EXECUTE ON FUNCTION public.handle_new_user() FROM public, anon, authenticated;

-- Fire the function every time a user is created in auth.users.
CREATE TRIGGER on_auth_user_created
AFTER INSERT ON auth.users
FOR EACH ROW EXECUTE PROCEDURE public.handle_new_user();
