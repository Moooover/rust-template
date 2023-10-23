use config::Config;
use database::DatabaseDriver;

mod config;
mod database;

#[tokio::main]
async fn main() {
    let config = Config::new();
    let database_driver = DatabaseDriver::init(&config)
        .await
        .expect(&format!("Unable to connect to DB!"));

    if let Err(err) = create_todo_table(&database_driver).await {
        println!("{}", err);
    }

    if let Err(err) = create_todo_index(&database_driver).await {
        println!("{}", err);
    }
}

async fn create_todo_table(database_driver: &DatabaseDriver) -> Result<(), String> {
    let todo_table = r#"
    DEFINE TABLE todo SCHEMAFULL;

    DEFINE FIELD subject ON todo TYPE string;
    DEFINE FIELD description ON todo TYPE string;
    DEFINE FIELD due_date ON todo TYPE datetime;
    DEFINE FIELD is_done ON todo TYPE bool;
    DEFINE FIELD created_at ON todo TYPE datetime;
    DEFINE FIELD updated_at ON todo TYPE datetime;
    "#;

    database_driver
        .client
        .query(todo_table)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn create_todo_index(database_driver: &DatabaseDriver) -> Result<(), String> {
    let todo_table = r#"
    DEFINE INDEX todoSearchIndex ON TABLE todo COLUMNS subject SEARCH ANALYZER ascii BM25;
    DEFINE ANALYZER english TOKENIZERS class FILTERS snowball(english);
    "#;

    database_driver
        .client
        .query(todo_table)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
