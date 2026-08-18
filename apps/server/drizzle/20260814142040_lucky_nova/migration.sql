CREATE TYPE "account_type" AS ENUM('CHECKING', 'SAVING', 'CASH', 'DIGITAL_WALLET');--> statement-breakpoint
CREATE TYPE "payment_method_type" AS ENUM('DEBIT_CARD', 'CREDIT_CARD', 'TRANSFER', 'CASH_PAYMENT');--> statement-breakpoint
CREATE TYPE "transaction_type" AS ENUM('INITIAL_BALANCE', 'DEPOSIT', 'WITHDRAW');--> statement-breakpoint
CREATE TABLE "accounts" (
	"id" uuid PRIMARY KEY,
	"profile_id" uuid NOT NULL,
	"name" varchar(255),
	"type" "account_type" DEFAULT 'SAVING'::"account_type" NOT NULL,
	"balance" numeric(18,2) DEFAULT '0.00' NOT NULL,
	"currency_code" varchar(3) NOT NULL
);
--> statement-breakpoint
ALTER TABLE "accounts" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE TABLE "payment_method_configs" (
	"id" uuid PRIMARY KEY,
	"profile_id" uuid NOT NULL,
	"payment_method_id" uuid NOT NULL,
	"credit_limit" numeric(18,2),
	"interest_rate" numeric(5,2),
	"billing_cycle_day" integer
);
--> statement-breakpoint
ALTER TABLE "payment_method_configs" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE TABLE "payment_methods" (
	"id" uuid PRIMARY KEY,
	"account_id" uuid NOT NULL,
	"type" "payment_method_type" DEFAULT 'CASH_PAYMENT'::"payment_method_type" NOT NULL,
	"card_number_last4" varchar(4),
	"card_holder" varchar(32),
	"is_active" boolean DEFAULT true NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
ALTER TABLE "payment_methods" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE TABLE "transaction_configs" (
	"id" uuid PRIMARY KEY,
	"profile_id" uuid NOT NULL,
	"transaction_id" uuid NOT NULL,
	"payment_method_id" uuid,
	"installments" integer DEFAULT 1 NOT NULL,
	"credit_limit" numeric(18,2),
	"interest_rate" numeric(5,2),
	"billing_cycle_day" integer
);
--> statement-breakpoint
ALTER TABLE "transaction_configs" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE TABLE "transactions" (
	"id" uuid PRIMARY KEY,
	"profile_id" uuid NOT NULL,
	"account_id" uuid,
	"payment_method_id" uuid,
	"category_id" uuid,
	"type" "transaction_type" NOT NULL,
	"amount" numeric(18,2) NOT NULL,
	"notes" text,
	"date" timestamp with time zone DEFAULT now() NOT NULL,
	"created_at" timestamp with time zone DEFAULT now() NOT NULL
);
--> statement-breakpoint
ALTER TABLE "transactions" ENABLE ROW LEVEL SECURITY;--> statement-breakpoint
CREATE INDEX "accounts_profile_id_index" ON "accounts" ("profile_id");--> statement-breakpoint
CREATE INDEX "payment_method_configs_payment_method_id_index" ON "payment_method_configs" ("payment_method_id");--> statement-breakpoint
CREATE INDEX "payment_methods_account_id_index" ON "payment_methods" ("account_id");--> statement-breakpoint
CREATE INDEX "transaction_configs_transaction_id_index" ON "transaction_configs" ("transaction_id");--> statement-breakpoint
CREATE INDEX "transaction_configs_payment_method_id_index" ON "transaction_configs" ("payment_method_id");--> statement-breakpoint
CREATE INDEX "transactions_profile_id_index" ON "transactions" ("profile_id");--> statement-breakpoint
CREATE INDEX "transactions_account_id_index" ON "transactions" ("account_id");--> statement-breakpoint
CREATE INDEX "transactions_payment_method_id_index" ON "transactions" ("payment_method_id");--> statement-breakpoint
CREATE INDEX "transactions_category_id_index" ON "transactions" ("category_id");--> statement-breakpoint
ALTER TABLE "accounts" ADD CONSTRAINT "accounts_profile_id_profiles_id_fkey" FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "payment_method_configs" ADD CONSTRAINT "payment_method_configs_profile_id_profiles_id_fkey" FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "payment_method_configs" ADD CONSTRAINT "payment_method_configs_AUlyigvJ0Ect_fkey" FOREIGN KEY ("payment_method_id") REFERENCES "payment_methods"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "payment_methods" ADD CONSTRAINT "payment_methods_account_id_accounts_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transaction_configs" ADD CONSTRAINT "transaction_configs_profile_id_profiles_id_fkey" FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transaction_configs" ADD CONSTRAINT "transaction_configs_transaction_id_transactions_id_fkey" FOREIGN KEY ("transaction_id") REFERENCES "transactions"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transaction_configs" ADD CONSTRAINT "transaction_configs_payment_method_id_payment_methods_id_fkey" FOREIGN KEY ("payment_method_id") REFERENCES "payment_methods"("id") ON DELETE SET NULL ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transactions" ADD CONSTRAINT "transactions_profile_id_profiles_id_fkey" FOREIGN KEY ("profile_id") REFERENCES "profiles"("id") ON DELETE CASCADE ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transactions" ADD CONSTRAINT "transactions_account_id_accounts_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE SET NULL ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transactions" ADD CONSTRAINT "transactions_payment_method_id_payment_methods_id_fkey" FOREIGN KEY ("payment_method_id") REFERENCES "payment_methods"("id") ON DELETE SET NULL ON UPDATE CASCADE;--> statement-breakpoint
ALTER TABLE "transactions" ADD CONSTRAINT "transactions_category_id_categories_id_fkey" FOREIGN KEY ("category_id") REFERENCES "categories"("id") ON DELETE SET NULL ON UPDATE CASCADE;--> statement-breakpoint
CREATE POLICY "Allow: Users can manage their own accounts" ON "accounts" AS PERMISSIVE FOR ALL TO "authenticated" USING ((select auth.uid()) = profile_id) WITH CHECK ((select auth.uid()) = profile_id);--> statement-breakpoint
CREATE POLICY "Allow: Users to manage their own payment methods configuration" ON "payment_method_configs" AS PERMISSIVE FOR ALL TO "authenticated" USING (
                (select auth.uid()) = payment_method_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.payment_methods
                    WHERE payment_methods.id = payment_method_configs.payment_method_id
                    AND payment_methods.account_id IN (
                        SELECT id FROM public.accounts WHERE accounts.profile_id = 
                            (select auth.uid())
                    )
                )
            ) WITH CHECK (
                (select auth.uid()) = payment_method_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.payment_methods
                    WHERE payment_methods.id = payment_method_configs.payment_method_id
                    AND payment_methods.account_id IN (
                        SELECT id FROM public.accounts WHERE accounts.profile_id = 
                            (select auth.uid())
                    )
                )
            );--> statement-breakpoint
CREATE POLICY "Allow: users can manage payment methods on their accounts" ON "payment_methods" AS PERMISSIVE FOR ALL TO "authenticated" USING (
                (select auth.uid()) = (
                    SELECT accounts.profile_id
                    FROM accounts
                    WHERE accounts.id = account_id
                )
            ) WITH CHECK (
                (select auth.uid()) = (
                    SELECT accounts.profile_id
                    FROM accounts
                    WHERE accounts.id = account_id
                )
            );--> statement-breakpoint
CREATE POLICY "Allow: Users can manage their own transaction configs" ON "transaction_configs" AS PERMISSIVE FOR ALL TO "authenticated" USING (
                (select auth.uid()) = transaction_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.transactions
                    WHERE transactions.id = transaction_configs.transaction_id
                    AND transactions.profile_id = (select auth.uid())
                )
            ) WITH CHECK (
                (select auth.uid()) = transaction_configs.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.transactions
                    WHERE transactions.id = transaction_configs.transaction_id
                    AND transactions.profile_id = (select auth.uid())
                )
            );--> statement-breakpoint
CREATE POLICY "Allow: Users can manage their own transactions" ON "transactions" AS PERMISSIVE FOR ALL TO "authenticated" USING (
                (select auth.uid()) = transactions.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.accounts
                    WHERE accounts.id = transactions.account_id
                    AND accounts.profile_id = (select auth.uid())
                )
                AND (
                    transactions.category_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM public.categories
                        WHERE categories.id = transactions.category_id
                        AND (categories.profile_id = (select auth.uid()) OR categories.profile_id IS NULL)
                    )
                )) WITH CHECK (
                (select auth.uid()) = transactions.profile_id
                AND EXISTS (
                    SELECT 1 FROM public.accounts
                    WHERE accounts.id = transactions.account_id
                    AND accounts.profile_id = (select auth.uid())
                )
                AND (
                    transactions.category_id IS NULL
                    OR EXISTS (
                        SELECT 1 FROM public.categories
                        WHERE categories.id = transactions.category_id
                        AND (categories.profile_id = (select auth.uid()) OR categories.profile_id IS NULL)
                    )
                ));