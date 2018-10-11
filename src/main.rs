mod tasks;

fn main() {
    let mut task = tasks::Task::new("hi!".to_string());
    let child_task = tasks::Task::new("child".to_string());

    task.add_child(child_task);

    println!("{}", task.title);
}
