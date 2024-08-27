use std::collections::VecDeque;

#[derive(Debug)]
struct Task {
    id: u32,
    description: String,
    priority: u8,
}

struct TaskQueue {
    tasks: VecDeque<Task>,
}

impl TaskQueue {
    fn new() -> Self {
        TaskQueue {
            tasks: VecDeque::new(),
        }
    }

    fn add_task(&mut self, task: Task) { self.tasks.push_back(task)}
    fn get_next_task(&mut self) -> Option<Task> { self.tasks.pop_front() }
    fn peek_next_task(&self) -> Option<&Task> { self.tasks.front() }
}

fn main() {
    let mut queue = TaskQueue::new();

    queue.add_task(Task {
        id: 1,
        description: String::from("Write documentation"),
        priority: 2,
    });

    queue.add_task(Task {
        id: 2,
        description: String::from("Fix critical bug"),
        priority: 1,
    });

    if let Some(task) = queue.peek_next_task() {
        println!("Next task: {:?}", task);
    }

    if let Some(completed_task) = queue.get_next_task() {
        println!("Completed task: {:?}", completed_task);
    }
}
