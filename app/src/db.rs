use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};

/// Sets up the database
pub async fn db_setup() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<RocksDb>("data/{{ project-name }}.db").await?;

    db.use_ns("{{ project-name }}")
        .use_db("{{ project-name }}")
        .await?;

    db.query(
        r#"
        // Todo
        DEFINE TABLE IF NOT EXISTS todo SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS title ON TABLE todo TYPE string;
        "#,
    )
    .await?;

    // This can be imported into Surrealist and is useful when building/debugging
    if cfg!(debug_assertions) {
        db.export("backup.sql").await?;
    }

    Ok(db)
}
