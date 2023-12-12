use crate::todo_list::TodoList;
use crate::cli_ui; 

#[derive(Debug)]
pub struct TodoListDriver {
   todo_list: Vec<TodoList>, 
   actions: Vec<String>,
   ui_type: String, 
   ui: cli_ui::TodoListUI,
}

impl TodoListDriver {
    pub fn new(todo_list: Vec<TodoList>) -> TodoListDriver {
        let actions: Vec<String> = vec![
            "Select Todo List".to_string(), 
            "Print Todo List".to_string(), 
            "Eat Shit".to_string(), 
            "Fuck Off".to_string()
        ];

        return TodoListDriver {
            todo_list, 
            actions,
            ui_type: "CLI".to_string(), 
            ui: cli_ui::TodoListUI::new(),
        }; 
    }

    pub fn init(&self) {
        let tdl_titles: Vec<&String> = self.todo_list.iter().map(|x| *&x.get_title()).collect();
        self.ui.print_splash();
        self.ui.main_menu(tdl_titles)
    }
}
