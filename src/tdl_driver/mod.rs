use crate::todo_list::TodoList;
use crate::cli_ui; 

#[derive(Debug)]
pub struct TodoListDriver {
   todo_list: TodoList, 
   actions: Vec<String>,
   ui_type: String, 
   pub ui: cli_ui::TodoListUI,
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
            actions,
            ui_type: "CLI".to_string(), 
            ui: cli_ui::TodoListUI::new(),
        }; 
    }

    pub fn init(self) {
        self.ui.menu();
    }
}
