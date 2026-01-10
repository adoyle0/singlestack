use leptos::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use surrealdb::{Surreal, engine::local::Db};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
}

// Endpoints need to be specified to ensure they match across ssr/csr builds
#[server(endpoint = "add_todo")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let db = expect_context::<Surreal<Db>>();

    db.query(
        r#"
        CREATE todo CONTENT {
            title: $title,
        }
        "#,
    )
    .bind(("title", title))
    .await?
    .check()?;

    Ok(())
}

#[server(endpoint = "delete_todo")]
pub async fn delete_todo(id: String) -> Result<(), ServerFnError> {
    let db = expect_context::<Surreal<Db>>();

    db.query(
        r#"
        DELETE type::thing("todo", $id)
        "#,
    )
    .bind(("id", id))
    .await?
    .check()?;

    Ok(())
}

#[server(endpoint = "get_todos")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let db = expect_context::<Surreal<Db>>();

    let mut response: surrealdb::Response = db
        .query(
            r#"
            SELECT *, <string> record::id(id) AS id
                FROM todo
            "#,
        )
        .await?
        .check()?;

    let todos: Vec<Todo> = response.take(0)?;

    Ok(todos)
}
