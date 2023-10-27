use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

use super::database::TodoModel;
use super::request::CreateTodoRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: Ulid,
    pub subject: String,
    pub description: String,
    pub is_done: bool,
    pub due_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TodoModel> for Todo {
    fn from(value: TodoModel) -> Self {
        Self {
            id: Ulid::from_string(&value.id.id.to_string()).expect("Error parsing ULID"),
            subject: value.subject,
            description: value.description,
            is_done: value.is_done,
            due_date: value.due_date,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl Todo {
    pub fn from_request(data: CreateTodoRequest) -> Self {
        let creation_date = Utc::now();

        Self {
            id: Ulid::new(),
            subject: data.subject,
            description: data.description,
            is_done: false,
            due_date: data.due_date,
            created_at: creation_date,
            updated_at: creation_date,
        }
    }
}
