
#[derive(Debug)]
struct TodoListItem {
  id: String, 
  item_title: String, 
  item_description: String,
  item_completed: bool, 
}

fn main() {
    let todo_list: Vec<TodoListItem> = create_tdl_item(); 
    print_menu(&todo_list); 
    show_full_item_at_index(&todo_list, 2) 
}

fn print_menu(todo_list: &Vec<TodoListItem>) {
    println!("==============================="); 
    println!("|| Welcome to your TodoListi ||");
    println!("==============================="); 
    println!(""); 
    println!("** Please Select an action from ");
    println!("** the list below. ");
    println!(""); 
    println!(">>> 1 View Todo List            "); 
    println!(">>> 2 Add Todo List Item        "); 
    println!(">>> 3 Update Todo List Item     "); 
    println!(">>> 4 Delete Todo List Item     ");
    println!(">>> 5 See full information for an individual item "); 
    println!(""); 
    for ele in todo_list {
        println!("[{}] => {}", ele.id, ele.item_title); 
    }

    println!(""); 
}

fn show_full_item_at_index(todo_list: &Vec<TodoListItem>, index: usize) {
   let item: &TodoListItem = &todo_list[index]; 

   println!("*******************************"); 
   println!("|| Showing TDL Item {}       ||", index + 1);
   println!("*******************************"); 

   println!("> Title: {}", item.item_title); 
   println!("> Description: {}", item.item_description); 
   println!("> Has Been Completed: {}", item.item_completed); 
}

fn create_tdl_item() -> Vec<TodoListItem> {
   let mut vec = Vec::new(); 
   let mut number: u32 = 5; 

   while number != 0 {
     let str_number = number.to_string(); 
     let tdl_item = TodoListItem {
       id: String::from(str_number.clone()), 
       item_title:  format!("Todo List Item {}", str_number.clone()), 
       item_description: format!("This is my description for Todo List Item {}", str_number.clone()),
       item_completed: false, 
     };

     vec.insert(0, tdl_item); 
     number -= 1; 
   }; 

   return vec; 
}
