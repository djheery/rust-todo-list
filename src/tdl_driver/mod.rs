mod todo_list; 

use crate::todo_list::TodoList;

pub struct TodoListDriver {
   todo_list: TodoList, 
   actions: Vec<String>,
}

impl TodoListDriver {
    pub fn new(todo_list: TodoList) -> TodoListDriver {
        let actions = vec!["create", "read", "update", "delete"];
    }
}
