struct Task {
    id: u32,
    text: String,
    done: bool
}
struct ToDoList {
    current_index: u32,
    tasks: Vec<Task>
}

impl ToDoList {
    fn new() -> Self {
        ToDoList {
            current_index: 0,
            tasks: Vec::<Task>::new()
        }
    }
    fn show(&self) {
        if self.tasks.len() == 0 { println!("Enjoy! You have no tasks"); return }
        println!("Index | Done | Task");
        for task in &self.tasks {
            println!("{} | {} | {}",
                     task.id,
                     match task.done { true => 'V', _ => 'X' },
                     task.text
            );
        }
    }

    fn add(&mut self, task_content: String) {
        self.tasks.push(Task {
            id: self.current_index,
            text: task_content,
            done: false
        });
        self.current_index += 1;
    }

    fn remove(&mut self, id: u32) -> bool {
        if self.exist(id) {
            self.tasks.remove(id as usize);
            return true;
        }
        false
    }

    fn exist(&self, id: u32) -> bool {
        match self.tasks.iter().find(|t| t.id == id) {
            Some(t) => true,
            None => false
        }
    }

    fn edit(&mut self, id: u32, task_content: Option<String>, done: Option<bool>) -> bool {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(t) => {
                if task_content.is_some() {
                    t.text = task_content.unwrap();
                }
                if done.is_some() {
                    t.done = done.unwrap();
                }
                true
            },
            None => false
        }
    }
}

fn read_line() -> String {
    let mut input = String::from("");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            println!("Fail to read line");
            input
        }
    }
}


fn main() {
    let mut app = ToDoList::new();
    let introduction = "\r\
        Press S: Show \r
        Press A: Add \r
        Press E: Edit \r
        Press R: Remove \r
        Press Q: Quit
    ";
    println!("Me do to do app");
    loop {
        println!("--------------");
        println!("{}", introduction);
        match read_line().as_str() {
            "S" => { app.show() },
            "A" => {
                println!("Add task...");
                println!("Task's content: ");
                let content = read_line();
                app.add(content);
            },
            "R" => {
                println!("Remove task by id...");
                println!("See task list by press S and input the task's you want to remove.");
                let id = match read_line().parse::<u32>() {
                    Ok(input) => input,
                    Err(_) => {
                        println!("Fail to get id");
                        continue
                    }
                };
                println!("Are you sure to remove task? (Y/N)" );
                match read_line().as_str() {
                    "Y" => { app.remove(id); },
                    "N" => continue,
                    _ => continue
                }

            },
            "E" => {
                println!("Edit task...");
                println!("Type the task'id you want to edit...");
                let id = match read_line().parse::<u32>() {
                    Ok(input) => input,
                    Err(_) => {
                        println!("Fail to get id");
                        continue
                    }
                };
                if !app.exist(id) {
                    println!("No task found");
                }
                let mut edit_content: Option<String> = None;
                let mut edit_done: Option<bool> = None;
                println!("Edit content (Skip by press enter)");
                if let input = read_line(){
                    if input.len() > 0 { edit_content = Option::from(input) };
                }
                println!("Done? (V = Done; X = Undone: Skip = Enter)");
                if let input = read_line() {
                    if input == "V".to_string() { edit_done = Option::from(true) }
                    if input == "X".to_string() { edit_done = Option::from(false) }
                }
                app.edit(id, edit_content, edit_done);
            },
            "Q" => { println!("Quit..."); break },
            _ => println!("Fail to choose option")
        }
    }
}
