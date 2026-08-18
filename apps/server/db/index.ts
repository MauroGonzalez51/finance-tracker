/**
 * @module db
 *
 * Database schema definitions and relations for the finance-tracker.
 * Uses Drizzle ORM with PostgreSQL (Supabase).
 *
 * Structure:
 * - `schemas/` — table definitions, enums, and column docs
 * - `relations/` — Drizzle relational query definitions
 */
export * from "./relations";
export * from "./schemas";
