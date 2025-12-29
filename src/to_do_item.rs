// Structure of the item
pub struct ToDoItem {
    pub title: String,
    pub completed: bool
}

// Implement methods for items
impl ToDoItem {
    // A constructor for item
    pub fn new(title: String) -> Self {
        ToDoItem {
            title,
            completed: false
        }
    }

    // Load an existing item
    pub fn load(title: String, completed: bool) -> Self {
        ToDoItem {
            title,
            completed
        }
    }
}