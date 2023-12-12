use crate::todo_list::{ TodoListItem, TodoList };

#[derive(Debug)]
pub struct TodoListUI {
   command_log: Vec<String>, 
}

impl TodoListUI {
    pub fn new() -> TodoListUI {
       let command_log: Vec<String> = vec![]; 
       return TodoListUI {
          command_log,
       }
    }

    pub fn print_menu() {
        
    }
    
    pub fn print_tdl_item(&mut self, todo_list_item: &TodoListItem) {
        println!(" "); 
        println!("** Showing Item **"); 
        println!("> Item Name: {} ", todo_list_item.get_title()); 
        println!("> Item Description: {}", todo_list_item.get_description()); 
        println!("> Item Completed: {}", todo_list_item.get_completion_status()); 
        println!(" "); 

        self.command_log.push("Printed Item".to_string());
    }

    pub fn print_full_tdl(todo_list: &TodoList) {
        println!("{:#?}", todo_list);
    }

    pub fn print_tdl_update(item_name: &String) {
      println!("Item Updated: {}", item_name);   
    }

    pub fn print_tdl_item_deletion(item_name: &String) {
       println!("Item Deleted: {}", item_name);  
    }

    pub fn print_tdl_item_addition(item_name: &String) {
       println!("Item Added: {}", item_name);  
    }

    pub fn print_tdl_item_completion(item_name: &String) {
       println!("Item Completed: {}", item_name);  
    }
    
}

