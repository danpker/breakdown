pub struct Task {
    // A task is essentially just a node in a tree.
    // It has a title and (optional) children.
    pub title: String,
    pub children: Vec<Task>,
}

impl Task {
    pub fn new(title: String) -> Task {
        Task {
            title: title,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Task) {
        self.children.push(child);
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

    #[test]
    fn test_can_add_child_to_task() {
        let mut parent = Task::new("parent".to_string());
        let child = Task::new("child".to_string());

        // Task starts with no children
        assert_eq!(parent.children.len(), 0);

        parent.add_child(child);

        assert_eq!(parent.children.len(), 1);
    }
}
