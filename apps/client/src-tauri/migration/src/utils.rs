use sea_orm_migration::prelude::*;

pub trait ToIdentifier {
    fn to_identifier(&self) -> String;
}

impl<T: Iden> ToIdentifier for T {
    fn to_identifier(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct IndexConfig {
    pub column: String,
    pub name: String,
    pub unique: bool,
}

impl IndexConfig {
    pub fn new(column: impl ToIdentifier, name: impl ToIdentifier) -> Self {
        Self {
            column: column.to_identifier(),
            name: name.to_identifier(),
            unique: false,
        }
    }

    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }
}

pub async fn create_indexes(
    manager: &SchemaManager<'_>,
    table: impl ToIdentifier,
    indexes: &[IndexConfig],
) -> Result<(), DbErr> {
    for config in indexes {
        let mut index = Index::create()
            .name(config.name.clone())
            .if_not_exists()
            .table(table.to_identifier())
            .col(config.column.clone())
            .to_owned();

        if config.unique {
            index = index.unique().to_owned();
        }

        manager.create_index(index).await?;
    }

    Ok(())
}
