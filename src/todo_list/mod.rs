#![allow(unused)]

use std::io; 

#[derive(Debug)]
pub struct TodoListItem {
  id: String, 
  item_title: String, 
  item_description: String,
  item_completed: bool, 
}

#[derive(Debug)]
pub enum ItemUpdateFields {
    TITLE,
    DESCRIPTION,
}

impl TodoListItem {

    pub fn new(id: String, item_title: String, item_description: String, item_completed: bool) -> TodoListItem {
      TodoListItem {
        id, 
        item_title, 
        item_description, 
        item_completed
      }
    }

    pub fn get_title(&self) -> &String {
        return &self.item_title;
    }

    pub fn get_description(&self) -> &String {
        return &self.item_description; 
    }

    pub fn get_completion_status(&self) -> bool {
        return self.item_completed; 
    }

    fn set_item_details(&mut self, field: ItemUpdateFields, update: String) {
       match field {
         ItemUpdateFields::TITLE => self.item_title = update, 
         ItemUpdateFields::DESCRIPTION => self.item_description = update, 
       }
    }

    fn set_item_completion_status(&mut self, update: bool) {
       self.item_completed = update;  
    }
}

#[derive(Debug)]
pub struct TodoList { 
   id: String, 
   todo_list_title: String, 
   todo_list_items: Vec<TodoListItem>
}


impl TodoList {
    pub fn new(todo_list_items: Vec<TodoListItem>, title: String, id: String) -> TodoList {
        TodoList {
           id, 
           todo_list_title: title, 
           todo_list_items, 
     }
    }

    pub fn get_title(&self) -> &String {
       return &self.todo_list_title; 
    }

    fn get_item_at_index(&self, index: usize) -> Result<&TodoListItem, &'static str> {
        let item = self.todo_list_items.get(index);  

        match item {
          Some(i) => return Ok(&item.unwrap()),
          None => return Err("ITEM_NOT_FOUND") 
        }
    }

    fn add_new_tdl_item(&mut self, item: TodoListItem) {
        self.todo_list_items.push(item);
    }
    
    fn remove_item_at_index(&mut self, index: usize) {
        
    }

    fn update_item_at_index(&mut self, index: usize) {
        
    }

    fn add_new_item(&mut self, new_item: TodoListItem) {
        self.todo_list_items.push(new_item); 
    }
}
