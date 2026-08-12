import { pgSchema } from "drizzle-orm/pg-core";

/**
 * Reference to Supabase's internal `auth` schema.
 * Used only to declare foreign key targets — Drizzle will not create or migrate this schema.
 */
export const AUTH_SCHEMA = pgSchema("auth");
