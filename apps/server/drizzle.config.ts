import assert from "node:assert";
import process from "node:process";
import { defineConfig } from "drizzle-kit";

// eslint-disable-next-line dot-notation
const DATABASE_URL = process.env["DATABASE_URL"];
assert(DATABASE_URL !== undefined, "DATABASE_URL must be defined");

export default defineConfig({
    out: "./drizzle",
    schema: "./db/index.ts",
    dialect: "postgresql",
    dbCredentials: {
        url: DATABASE_URL,
    },
});
