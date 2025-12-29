use std::fs;
use std::io::Write;
use crate::to_do_item::ToDoItem;

// Structure of the list
pub struct ToDoList {
    items: Vec<ToDoItem>
}

impl ToDoList {
    // A constructor for the list
    pub fn new() -> Self {
        ToDoList {
            items: Vec::new()
        }
    }

    // Add an item
    pub fn add(&mut self, title: String) -> String {
        // Create a new item structure
        let item = ToDoItem::new( title.clone() );

        // Add the created structure to the list
        self.items.push(item);

        // Display a message
        format!("[ADD]: Task '{}' has been created.", title)
    }

    // Delete an item
    pub fn delete(&mut self, id: i32) -> Result<(), ()> {
        // Check if the range is valid
        if id as usize <= self.items.len() && id - 1 >= 0 {
            // Remove the index
            self.items.remove(id as usize - 1);
            Ok(())
        } else {
            Err(())
        }
    }

    // Search task by id
    pub fn search(&self, id: i32) -> Result<String, ()> {
        // Check if the range is valid
        if id as usize <= self.items.len() && id - 1 >= 0 {
            // Return the searched result
            Ok(self.items[id as usize - 1].title.clone())
        } else {
            Err(())
        }
    }

    // Display the items
    pub fn list(&self) {
        // Checks if the list is empty
        if self.items.len() == 0 {
            println!("[LIST]: No items found.");
            return;
        }

        // List header
        println!("\t====================: TASK LIST :====================");

        // If the list is not empty, then iterate the list
        for (index, item) in self.items.iter().enumerate() {
            // Separate the completed items to add a mark
            if item.completed {
                println!("\t{}: [x] - {}", index + 1, item.title);
            } else {
                println!("\t{}: [ ] - {}", index + 1, item.title);
            }
        }

        // List footer
        println!("\t=====================================================");
    }

    // Complete a task
    pub fn complete(&mut self, id: i32) -> Result<String, String> {
        // Check if the range is valid
        if id as usize <= self.items.len() && id - 1 >= 0 {
            let id = id as usize - 1;

            // Checks if the current item is not completed
            if self.items[id].completed == false {
                // If true, then mark as complete
                self.items[id].completed = true;
                Ok(format!("[COMPLETE]: Task '{}' is complete.", self.items[id].title))
            }
            // Otherwise, display that the task is already completed
            else {
                Ok(format!("[COMPLETE]: Task '{}' has already been completed.", self.items[id].title))
            }
        }
        // Return error
        else {
            Err(format!("[ERROR]: Task [{}] is out of range.", id + 1))
        }
    }

    // Save the file
    pub fn save(&self) -> Result<String, String> {
        // Create a file is the file doesn't exist
        let mut file = fs::File::create("tasks.csv").expect("[CRITICAL]: Could not create file.");

        // Checks if there is a list to iterate
        if self.items.len() as i32 <= 0 {
            // Write to the file
            match writeln!(file, "") {
                Ok(_) => (),
                Err(_) => return Err("[CRITICAL]: Could not write to file.".to_string())
            }

            return Ok("[SAVE]: Saving an empty list.".to_string());
        }

        // Iterate the list
        for item in &self.items {
            // Write to the file
            match writeln!(file, "{},{}", item.title, item.completed) {
                Ok(_) => (),
                Err(_) => return Err("[CRITICAL]: Could not write to file.".to_string())
            }
        }

        // List is saved without errors
        Ok("[SAVE]: List is successfully saved.".to_string())
    }

    // Load the file
    pub fn load(&mut self) {
        // Read file contents
        match fs::read_to_string("tasks.csv") {
            // If success
            Ok(contents) => {
                // Split lines for iterations of items
                let items = contents.split("\n").collect::<Vec<&str>>();

                // Iterate through the items
                for item in items {
                    // Make sure the item is not empty
                    if !item.is_empty() {
                        // Split each item
                        let parts = item.split(",").collect::<Vec<&str>>();

                        // Verify if it has exactly two parts
                        if parts.len() == 2 {
                            // Push the item to the list
                            self.items.push(
                                // Structure the item and parse the parts properly
                                ToDoItem::load(
                                    parts[0].to_string(),
                                    parts[1].parse::<bool>().unwrap()
                                )
                            )
                        }
                    }
                }
            },
            // If error
            Err(error_message) => println!("{error_message}")
        }
    }
}