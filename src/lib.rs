mod error;

use chrono::{DateTime, Utc};
use error::RusteriaError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use uuid::Uuid;

pub struct TodoFilter {
    pub status: Option<TodoStatus>,
}

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
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Completed(_), Self::Completed(_)) | (Self::Uncompleted, Self::Uncompleted) => {
                true
            }
            _ => false,
        }
    }
}

impl Deref for TodoStatus {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        match self {
            TodoStatus::Uncompleted => &false,
            TodoStatus::Completed(_) => &true,
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

/// Adds a new Todo item to the list
///
///
/// ## Example
/// ```
/// use rustasteria::*;
/// let mut v: Vec<TodoItem> = Vec::new();
/// let new_item = add_todo("Buy Groceries".to_owned(), &mut v);
/// assert_eq!(&new_item.contents, "Buy Groceries");
/// assert_eq!(v.len(), 1);
/// ```
pub fn add_todo<'a>(contents: String, todos: &'a mut Vec<TodoItem>) -> &'a TodoItem {
    let item = TodoItem::new(contents);
    todos.push(item);
    todos.last().unwrap()
}

/// Deletes a ToDo item from the list, returning a RusteriaError::NoSuchTodoItem if no matching id is found
///
///
/// ## Example
/// ### Successfully deleting an item
/// ```
/// use rustasteria::*;
/// let mut v: Vec<TodoItem> = vec![TodoItem::new("Buy Groceries".to_owned())];
/// let id = v.first().unwrap().id;
/// let res = delete_todo(id, &mut v);
/// assert!(res.is_ok());
/// assert_eq!(v.len(), 0);
/// ```
///
/// ### Item not found
/// ```
/// use rustasteria::*;
/// use uuid::*;
/// let mut v: Vec<TodoItem> = Vec::new();
/// let id = uuid::Uuid::now_v7();
/// let res = delete_todo(id, &mut v);
/// assert!(res.is_err());
/// ```
pub fn delete_todo(id: Uuid, todos: &mut Vec<TodoItem>) -> Result<(), RusteriaError> {
    let idx = todos
        .iter()
        .position(|todo| todo.id == id)
        .ok_or(RusteriaError::NoSuchTodo)?;
    todos.remove(idx);
    Ok(())
}

/// Updates a ToDo with the given status. Returns RusteriaError::NoSuchTodo if no matching id is found
pub fn update_todo<'a>(
    id: Uuid,
    status: TodoStatus,
    todos: &'a mut Vec<TodoItem>,
) -> Result<&'a TodoItem, RusteriaError> {
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.modified = Some(match status {
                TodoStatus::Uncompleted => Utc::now(),
                TodoStatus::Completed(date_time) => date_time,
            });
            todo.status = status;
            return Ok(todo);
        }
    }
    Err(RusteriaError::NoSuchTodo)
}

pub fn list_todos(filter: TodoFilter, todos: impl IntoIterator<Item = TodoItem>) -> Vec<TodoItem> {
    todos
        .into_iter()
        .filter(|x| {
            if let Some(filtered_status) = &filter.status {
                &x.status == filtered_status
            } else {
                true
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t() {
        println!("do it");
    }

    #[test]
    fn status_derefs_into_bool() {
        let uncompleted = TodoStatus::Uncompleted;
        let completed = TodoStatus::Completed(DateTime::UNIX_EPOCH);

        assert_eq!(
            &(*uncompleted),
            &false,
            "Status::Uncompleted should have deref'd into `false`"
        );

        assert_eq!(
            &(*completed),
            &true,
            "Status::Completed should have deref'd into `true`"
        );
    }
}
