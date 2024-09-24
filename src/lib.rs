pub mod error;
pub mod todo;

use chrono::Utc;
pub use error::RusteriaError;
pub use todo::{TodoFilter, TodoItem, TodoStatus};
pub use uuid::Uuid;

/// Adds a new Todo item to the list
///
///
/// ## Example
/// ```
/// use rusteria::*;
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
/// use rusteria::*;
/// let mut v: Vec<TodoItem> = vec![TodoItem::new("Buy Groceries".to_owned())];
/// let id = v.first().unwrap().id;
/// let res = delete_todo(id, &mut v);
/// assert!(res.is_ok());
/// assert_eq!(v.len(), 0);
/// ```
///
/// ### Item not found
/// ```
/// use rusteria::*;
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
    fn status_derefs_into_bool() {}
}
