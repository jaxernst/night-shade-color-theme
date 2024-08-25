// Define an interface for the base structure of a Task
interface Task {
  id: number;
  title: string;
  description: string;
  completed: boolean;
}

// Define a union type for task status
type TaskStatus = "open" | "in progress" | "completed";

// Define an interface for detailed tasks that extends the base Task
interface DetailedTask extends Task {
  status: TaskStatus;
  deadline: Date;
  assignee?: string;
}

// Generic function to log task details
function logTask<T extends Task>(task: T): void {
  console.log(
    `Task: ${task.title} (${task.completed ? "Completed" : "Not Completed"})`
  );
  if ("status" in task) {
    console.log(`Status: ${task.status}`);
  }
  if ("deadline" in task) {
    console.log(`Deadline: ${task.deadline.toLocaleDateString()}`);
  }
}

// Function to complete a task
function completeTask(task: Task): Task {
  return { ...task, completed: true };
}

// Example usage of the task system
const task1: Task = {
  id: 1,
  title: "Setup TypeScript project",
  description: "Configure tsconfig and install dependencies",
  completed: false,
};

const detailedTask: DetailedTask = {
  id: 2,
  title: "Write TypeScript interfaces",
  description: "Create interfaces for the task management system",
  completed: false,
  status: "in progress",
  deadline: new Date(2023, 11, 31), // December 31, 2023
  assignee: "John Doe",
};

logTask(task1);
logTask(detailedTask);

const completedTask = completeTask(detailedTask);
logTask(completedTask);
