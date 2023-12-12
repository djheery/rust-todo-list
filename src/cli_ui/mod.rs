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

    pub fn print_splash(&self) {
        println!(" ");
        println!("====================================================");
        println!("|  ---------  /-------\\   ------\\    /-------\\     |"); 
        println!("|  |__   __|  |  ---  |   | ---  |   |  ---  |     |"); 
        println!("|    |   |    |  | |  |   | |  \\ |   |  | |  |     |"); 
        println!("|    |   |    |  | |  |   | |  | |   |  | |  |     |"); 
        println!("|    |   |    |  | |  |   | |  / |   |  | |  |     |"); 
        println!("|    |   |    |  ---  |   | ---  |   |  ---  |     |");
        println!("|    |___|    \\_______/   |______/    \\______/     |");
        println!("|                                                  |");
        println!("====================================================");
        println!(""); 
        println!("This is my cli todo application written in Rust."); 
        println!("It is simply a test application to be expanded upon to try programming in a new language"); 
        println!(""); 
        println!("Code By: Daniel Heery"); 
        println!("I hope you appreciate the shitty splash title. Yes I know.... it was a complete waste of time"); 
        println!(""); 
        println!("Application Start..."); 
        println!(""); 
    }

    pub fn menu(&self, actions: &Vec<String>) {
       println!("***********************");  
       println!("* Menu                *");
       println!("***********************");  
       
       for i in 0..actions.len() {
        println!("[{}] {}", i + 1, actions[i]); 
       }

       println!(" ");
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

    pub fn print_full_tdl(self, todo_list: &TodoList) {
        println!("{:#?}", todo_list);
    }

    pub fn print_tdl_update(self, item_name: &String) {
      println!("Item Updated: {}", item_name);   
    }

    pub fn print_tdl_item_deletion(self, item_name: &String) {
       println!("Item Deleted: {}", item_name);  
    }

    pub fn print_tdl_item_addition(self, item_name: &String) {
       println!("Item Added: {}", item_name);  
    }

    pub fn print_tdl_item_completion(self, item_name: &String) {
       println!("Item Completed: {}", item_name);  
    }
    
}

