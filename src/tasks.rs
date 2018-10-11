pub struct Task {
    // A task is essentially just a node in a tree.
    // It has a title and (optional) children.
    title: String,
    children: Vec<Task>,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title: title,
            children: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("{}", self.title);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_new_task_with_title() {
        let task = Task::new("hello".to_string());
        assert_eq!(task.title, "hello");
    }
}
