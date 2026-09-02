import process from "node:process";
import { cancel, log } from "@clack/prompts";
import { defineNuxtPrepareHandler } from "nuxt-prepare/config";
import { z } from "zod";

const EnvSchema = z.object({
    NODE_ENV: z.enum(["development", "production", "test"]).optional(),
    VITEST: z.boolean().optional(),
});

const SupabaseEnvSchema = EnvSchema.safeExtend({
    NUXT_PUBLIC_SUPABASE_URL: z.url(),
    NUXT_PUBLIC_SUPABASE_KEY: z.string().min(1),
    NUXT_SUPABASE_SECRET_KEY: z.string().min(1),
});

export default defineNuxtPrepareHandler(async () => {
    const env = EnvSchema.safeParse(process.env);
    if (env.data?.NODE_ENV === "test" || env.data?.VITEST) {
        return {};
    }

    const supabase = SupabaseEnvSchema.safeParse(process.env);
    if (!supabase.success) {
        supabase.error.issues
            .filter((issue) => issue.path)
            .forEach((issue) => log.error(`${issue.path}: ${issue.message}`));

        cancel("[Supabase] Missing variables");
        process.exit(1);
    }

    return {};
});
