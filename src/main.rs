mod tasks;

fn main() {
    let mut task = tasks::Task::new("hi!".to_string());
    let mut child_task = tasks::Task::new("child".to_string());
    let grand_child = tasks::Task::new("grand kid".to_string());

    child_task.add_child(grand_child);
    task.add_child(child_task);

    tasks::print_task(task);
}
