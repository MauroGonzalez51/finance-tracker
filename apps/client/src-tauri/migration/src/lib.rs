pub use sea_orm_migration::prelude::*;

mod macros;

mod m20260829_190325_init;
mod m20260830_152950_create_accounts;
mod m20260901_211119_payment_methods;
mod m20260901_222536_device_profile_sessions;
mod m20260901_233308_create_categories;
mod m20260902_130334_create_transactions;
mod m20260904_212146_create_loan;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260829_190325_init::Migration),
            Box::new(m20260830_152950_create_accounts::Migration),
            Box::new(m20260901_211119_payment_methods::Migration),
            Box::new(m20260901_222536_device_profile_sessions::Migration),
            Box::new(m20260901_233308_create_categories::Migration),
            Box::new(m20260902_130334_create_transactions::Migration),
            Box::new(m20260904_212146_create_loan::Migration),
        ]
    }
}
