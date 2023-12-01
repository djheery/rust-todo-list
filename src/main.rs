#![allow(unused)]

#[derive(Debug)]
struct TodoListItem {
  id: String, 
  item_title: String, 
  item_description: String,
  item_completed: bool, 
}

#[derive(Debug)]
enum ItemUpdateFields {
    TITLE,
    DESCRIPTION,
}

impl TodoListItem {

    fn new(id: String, item_title: String, item_description: String, item_completed: bool) -> TodoListItem {
      TodoListItem {
        id, 
        item_title, 
        item_description, 
        item_completed
      }
    }

    fn print_item_title(&self) {
        println!("> Title: {}", self.item_title); 
    }

    fn print_item_description(&self) {
        println!("> Descripton: {}", self.item_description); 
    }

    fn print_item_completion_status(&self) {
        println!("> Is Completed: {}", self.item_completed.to_string());
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
struct TodoList { 
   id: String, 
   todo_list_title: String, 
   todo_list_items: Vec<TodoListItem>
}

struct Driver {
    
}

impl TodoList {
    fn new(todo_list_items: Vec<TodoListItem>, title: String, id: String) -> TodoList {
        TodoList {
           id, 
           todo_list_title: title, 
           todo_list_items, 
     }
    }

    fn get_item_at_index(&self, index: usize) {
        let item = self.todo_list_items.get(index);  
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

fn main() {
    let todo_list_items: Vec<TodoListItem> = create_tdl_item(); 
    let title = String::from("My Super TodoList"); 
    let id = String::from("123456"); 
    let todo_list = TodoList::new(todo_list_items, title, id);

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
