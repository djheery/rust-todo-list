use crate::todo_list::TodoList;

#[derive(Debug)]
pub struct TodoListDriver {
   todo_list: TodoList, 
   actions: Vec<String>,
}

impl TodoListDriver {
    pub fn new(todo_list: TodoList) -> TodoListDriver {
        let actions: Vec<String> = vec![
            "create".to_string(), 
            "read".to_string(), 
            "update".to_string(), 
            "delete".to_string()
        ];

        return TodoListDriver {
            todo_list, 
            actions
        }; 
    }
}
