use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use ulid::Ulid;
use validator::Validate;

use crate::common::ApplicationError;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoModel {
    id: Thing,
    subject: String,
    description: String,
    is_done: bool,
    due_date: String,
    created_at: String,
    updated_at: String,
}

impl From<TodoModel> for Todo {
    fn from(value: TodoModel) -> Self {
        Self {
            id: Ulid::from_string(&value.id.id.to_string())
                .map_err(|err| {
                    ApplicationError::ServerError(vec![
                        String::from("Error parsing model ID"),
                        err.to_string(),
                    ])
                })
                .unwrap(),
            subject: value.subject,
            description: value.description,
            is_done: value.is_done,
            due_date: value
                .due_date
                .parse()
                .map_err(|_| {
                    ApplicationError::ServerError(vec![
                        String::from("Error parsing model ID"),
                        value.due_date,
                    ])
                })
                .unwrap(),
            created_at: value
                .created_at
                .parse()
                .map_err(|_| {
                    ApplicationError::ServerError(vec![
                        String::from("Error parsing model ID"),
                        value.created_at,
                    ])
                })
                .unwrap(),
            updated_at: value
                .updated_at
                .parse()
                .map_err(|_| {
                    ApplicationError::ServerError(vec![
                        String::from("Error parsing model ID"),
                        value.updated_at,
                    ])
                })
                .unwrap(),
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1))]
    subject: String,
    #[validate(length(min = 1))]
    description: String,
    due_date: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct UpdateTodoRequest {
    pub subject: Option<String>,
    pub description: Option<String>,
    pub is_done: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct TodoResponse {
    id: String,
    subject: String,
    description: String,
    is_done: bool,
    due_date: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Deserialize, Validate)]
pub struct SearchTodo {
    #[validate(length(min = 1))]
    pub q: String,
}

impl IntoResponse for TodoResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<Todo> for TodoResponse {
    fn from(value: Todo) -> Self {
        Self {
            id: value.id.to_string(),
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

impl UpdateTodoRequest {
    pub fn merge(mut self, original: Todo) -> Self {
        self.subject = self.subject.or(Some(original.subject));
        self.description = self.description.or(Some(original.description));
        self.due_date = self.due_date.or(Some(original.due_date));
        self.is_done = self.is_done.or(Some(original.is_done));

        self
    }
}
