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
    let title = String::from("My Super TodoList"); 
    let id = String::from("123456"); 
    let todo_list = TodoList::new(todo_list_items, title, id);
    let driver = TodoListDriver::new(todo_list);
    println!("{:#?}", driver); 


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

