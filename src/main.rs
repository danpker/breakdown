mod tasks;

fn main() {
    let task = tasks::Task::new("hi!".to_string());
    task.print();
}
