#![allow(unused)]

// this stuff comment
mod todo_list; 
mod tdl_driver; 
mod cli_ui;

use std::io; 
use crate::todo_list::TodoListItem; 
use crate::todo_list::TodoList;
use crate::tdl_driver::TodoListDriver;


fn main() {
    let todo_list_items: Vec<TodoListItem> = create_tdl_item(); 
    let todo_list_items_2: Vec<TodoListItem> = create_tdl_item(); 
    let title = String::from("My Super TodoList"); 
    let title2 = String::from("My Shitty TodoList"); 
    let id = String::from("123456"); 
    let id2 = String::from("1234543"); 
    let todo_list = TodoList::new(todo_list_items, title, id);
    let todo_list_2 = TodoList::new(todo_list_items_2, title2, id2); 
    let lists = vec![todo_list, todo_list_2];
    let driver = TodoListDriver::new(lists);
    driver.init();
}

fn read_user_input() -> String {
  // TODO: Write the logic for a reusable user input.; 
    return String::from("Hello World"); 
}



fn create_tdl_item() -> Vec<TodoListItem> {
   let mut vec = Vec::new(); 
   let mut number: u32 = 5; 

   while number != 0 {
     let str_number = number.to_string(); 
     let id = String::from(str_number.clone()); 
     let title = format!("Todo List Item {}", str_number.clone());
     let description = format!("This is my description for Todo List Item {}", str_number.clone());
     let completed = false; 
     let tdl_item = TodoListItem::new(id, title, description, completed);

     vec.insert(0, tdl_item); 
     number -= 1; 
   }; 

   return vec; 
}

