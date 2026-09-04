#[macro_export]
macro_rules! create_indexes {
    ($manager:expr, $table:expr, [ $( $col:expr => $name:expr ),* $(,)? ]) => {
        {
            $(
                $manager
                    .create_index(
                        sea_orm_migration::sea_query::Index::create()
                            .name($name.to_string())
                            .if_not_exists()
                            .table($table)
                            .col($col)
                            .to_owned(),
                    )
                    .await?;
            )*
        }
    };
}

#[macro_export]
macro_rules! create_unique_indexes {
    ($manager:expr, $table:expr, [ $( $col:expr => $name:expr ),* $(,)? ]) => {
        {
            $(
                $manager
                    .create_index(
                        sea_orm_migration::sea_query::Index::create()
                            .name($name.to_string())
                            .if_not_exists()
                            .table($table)
                            .col($col)
                            .unique()
                            .to_owned(),
                    )
                    .await?;
            )*
        }
    };
}
