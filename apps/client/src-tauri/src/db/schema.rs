// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        profile_id -> Text,
        name -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        balance -> Text,
        currency_code -> Text,
    }
}

diesel::table! {
    categories (id) {
        id -> Text,
        profile_id -> Nullable<Text>,
        code -> Nullable<Text>,
        name -> Nullable<Text>,
        parent_id -> Nullable<Text>,
    }
}

diesel::table! {
    payment_method_configs (id) {
        id -> Text,
        profile_id -> Text,
        payment_method_id -> Text,
        credit_limit -> Nullable<Text>,
        interest_rate -> Nullable<Text>,
        billing_cycle_day -> Nullable<Integer>,
    }
}

diesel::table! {
    payment_methods (id) {
        id -> Text,
        account_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        card_number_last4 -> Nullable<Text>,
        card_holder -> Nullable<Text>,
        is_active -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    profiles (id) {
        id -> Text,
    }
}

diesel::table! {
    transaction_configs (id) {
        id -> Text,
        profile_id -> Text,
        transaction_id -> Text,
        payment_method_id -> Nullable<Text>,
        installments -> Integer,
        credit_limit -> Nullable<Text>,
        interest_rate -> Nullable<Text>,
        billing_cycle_day -> Nullable<Integer>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Text,
        profile_id -> Text,
        account_id -> Nullable<Text>,
        payment_method_id -> Nullable<Text>,
        category_id -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Text,
        amount -> Text,
        notes -> Nullable<Text>,
        date -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(accounts -> profiles (profile_id));
diesel::joinable!(categories -> profiles (profile_id));
diesel::joinable!(payment_method_configs -> payment_methods (payment_method_id));
diesel::joinable!(payment_method_configs -> profiles (profile_id));
diesel::joinable!(payment_methods -> accounts (account_id));
diesel::joinable!(transaction_configs -> payment_methods (payment_method_id));
diesel::joinable!(transaction_configs -> profiles (profile_id));
diesel::joinable!(transaction_configs -> transactions (transaction_id));
diesel::joinable!(transactions -> accounts (account_id));
diesel::joinable!(transactions -> categories (category_id));
diesel::joinable!(transactions -> payment_methods (payment_method_id));
diesel::joinable!(transactions -> profiles (profile_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    categories,
    payment_method_configs,
    payment_methods,
    profiles,
    transaction_configs,
    transactions,
);
