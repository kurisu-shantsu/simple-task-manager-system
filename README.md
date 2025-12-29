# Simple Task Manager System

This project is a high-performance, command-line platform designed to streamline task organization through modern asynchronous programming. It transforms the standard "to-do list" into a responsive engine capable of handling concurrent background tasks and persistent data storage, ensuring that your workflow remains uninterrupted and your data is always secure.

* **Non-Blocking Concurrency**: Leverages the `Tokio` runtime to manage background reminders and system tasks without freezing the user interface, allowing for a seamless interactive experience.
* **Robust Persistence**: Utilizes structured serialization to securely store and retrieve task data, moving away from brittle manual parsing to industry-standard data handling.
* **Optimized Performance**: Engineered with Rust's memory safety and efficiency, ensuring that file I/O and background operations are both fast and reliable.
* **Background Reminders**: Spawns independent, lightweight "green threads" to alert users of due tasks in real-time.
* **Structured Data Vault**: Implements a reliable storage system that protects task integrity and prevents data loss.
* **Scalable Architecture**: Designed with a modular codebase that separates data logic from the execution loop for easy maintenance and expansion.

Built with [Rust](https://www.rust-lang.org/), [Tokio](https://tokio.rs/), and [Serde](https://serde.rs/).

# Commands Usage

| Command    | Argument(s)      | Description                                              |
|:-----------|:-----------------|:---------------------------------------------------------|
| `add`      | `<task_name>`    | Creates a new task and adds it to the list.              |
| `delete`   | `<task_number>`  | Removes a task from the list by its index.               |
| `remind`   | `<id> <seconds>` | Spawns a background task to alert you after `x` seconds. |
| `complete` | `<task_number>`  | Marks a task as completed with an `[x]` mark.            |
| `list`     | `none`           | Displays all tasks, sorted by entry.                     |
| `help`     | `none`           | Shows available commands and syntax.                     |
| `save`     | `none`           | Manually saves the current list to disk.                 |
| `quit`     | `none`           | Saves progress and terminates the application.           |

# Installation & Setup

- **Prerequisites**: Ensure you have Rust and Cargo installed.

- **Clone the Project**:

    ```terminaloutput
    git clone https://github.com/kurisu-shantsu/simple-task-manager-system
    ```

- **Change Directory**:

    ```terminaloutput
    cd simple-task-manager-system
    ```

- **Run the Application**:

    ```terminaloutput
    cargo run
    ```

# Architecture

The project is split into three core modules:

* `main.rs`: Starts the application.

* `input_handler.rs`: Handles the asynchronous loops and user inputs.

* `to_do_list.rs`: Manages the `Vec` of items and file I/O logic.

* `to_do_item.rs`: Defines the `ToDoItem` struct and its initialization logic.
