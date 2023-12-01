mod todo_list; 

use crate::todo_list::TodoList;

pub struct TodoListDriver {
   todo_list: TodoList, 
   actions: Vec<String>,
}

