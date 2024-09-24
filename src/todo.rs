use crate::RusteriaError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    /// a v7 id, which encodes creation-time
    pub id: Uuid,
    pub modified: Option<DateTime<Utc>>,
    pub contents: String,
    pub status: TodoStatus,
}

impl TodoItem {
    pub fn new(contents: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7(),
            modified: None,
            contents,
            status: TodoStatus::Uncompleted,
        }
    }
}

impl Display for TodoItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if core::mem::discriminant(&self.status)
            == core::mem::discriminant(&TodoStatus::Uncompleted)
        {
            " "
        } else {
            "x"
        };
        f.write_fmt(format_args!("[{}] {}", mark, &self.contents))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TodoStatus {
    Uncompleted,
    Completed(DateTime<Utc>),
}

impl PartialEq for TodoStatus {
    /// Our implementation of PartialEq ignored the completion time for Completed items
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Completed(_), Self::Completed(_)) | (Self::Uncompleted, Self::Uncompleted) => {
                true
            }
            _ => false,
        }
    }
}

impl From<bool> for TodoStatus {
    fn from(value: bool) -> Self {
        if value {
            TodoStatus::Completed(Utc::now())
        } else {
            TodoStatus::Uncompleted
        }
    }
}

impl TryFrom<String> for TodoStatus {
    type Error = RusteriaError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "open" {
            Ok(TodoStatus::Uncompleted)
        } else if value == "closed" {
            Ok(TodoStatus::Completed(Utc::now()))
        } else {
            Err(RusteriaError::Unknown("Invalid Status".into()))
        }
    }
}

impl Into<bool> for TodoStatus {
    fn into(self) -> bool {
        match self {
            TodoStatus::Uncompleted => false,
            TodoStatus::Completed(_) => true,
        }
    }
}

pub struct TodoFilter {
    pub status: Option<TodoStatus>,
}
