#![allow(unused)]

mod todo_list; 
mod tdl_driver; 

use std::io; 
use crate::todo_list::TodoListItem; 
use crate::todo_list::TodoList;


fn main() {
    let todo_list_items: Vec<TodoListItem> = create_tdl_item(); 
    let title = String::from("My Super TodoList"); 
    let id = String::from("123456"); 
    let todo_list = TodoList::new(todo_list_items, title, id);
}

fn read_user_input() -> String {

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

