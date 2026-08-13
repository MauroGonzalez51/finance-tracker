CREATE TABLE "categories" (
	"id" uuid PRIMARY KEY,
	"profile_id" uuid,
	"code" varchar(255),
	"name" varchar(255),
	"parent_id" uuid
);
--> statement-breakpoint
ALTER TABLE "categories" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE TABLE "profiles" (
	"id" uuid PRIMARY KEY
);
--> statement-breakpoint
ALTER TABLE "profiles" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE INDEX "categories_profile_id_index" ON "categories" ("profile_id");--> statement-breakpoint
CREATE INDEX "categories_parent_id_index" ON "categories" ("parent_id");--> statement-breakpoint
ALTER TABLE "categories" ADD CONSTRAINT "categories_profile_id_profiles_id_fkey" FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "categories" ADD CONSTRAINT "categories_parent_id_categories_id_fkey" FOREIGN KEY ("parent_id") REFERENCES "categories"("id");--> statement-breakpoint
ALTER TABLE "profiles" ADD CONSTRAINT "profiles_id_users_id_fkey" FOREIGN KEY ("id") REFERENCES "auth"."users"("id") ON DELETE CASCADE;--> statement-breakpoint
CREATE POLICY "Allow: Anon read system categories" ON "categories" AS PERMISSIVE FOR SELECT TO "anon" USING (profile_id IS NULL);--> statement-breakpoint
CREATE POLICY "Allow: Authenticated read system and own categories" ON "categories" AS PERMISSIVE FOR SELECT TO "authenticated" USING (profile_id IS NULL OR profile_id = (select auth.uid()));--> statement-breakpoint
CREATE POLICY "Allow: Authenticated create own categories" ON "categories" AS PERMISSIVE FOR INSERT TO "authenticated" WITH CHECK (profile_id = (select auth.uid()));--> statement-breakpoint
CREATE POLICY "Allow: Authenticated update own categories" ON "categories" AS PERMISSIVE FOR UPDATE TO "authenticated" USING (profile_id = (select auth.uid())) WITH CHECK (profile_id = (select auth.uid()));--> statement-breakpoint
CREATE POLICY "Allow: Authenticated delete own categories" ON "categories" AS PERMISSIVE FOR DELETE TO "authenticated" USING (profile_id = (select auth.uid()));--> statement-breakpoint
CREATE POLICY "Allow: Read-Only for unauthenticated" ON "profiles" AS PERMISSIVE FOR SELECT TO "anon" USING (true);--> statement-breakpoint
CREATE POLICY "Allow: Authenticated users to manage their own profile" ON "profiles" AS PERMISSIVE FOR ALL TO "authenticated" USING ((select auth.uid()) = id) WITH CHECK ((select auth.uid()) = id);

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