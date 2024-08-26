// C++: Example of a task scheduling system using modern C++ features
#include <iostream>
#include <vector>
#include <queue>
#include <functional>
#include <chrono>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <atomic>

class Task {
public:
  using TaskFunction = std::function<void()>;

  Task(TaskFunction func, std::chrono::steady_clock::time_point schedule_time)
    : func_(std::move(func)), schedule_time_(schedule_time) {}

  void execute() const { func_(); }
  std::chrono::steady_clock::time_point getScheduleTime() const { return schedule_time_; }

private:
  TaskFunction func_;
  std::chrono::steady_clock::time_point schedule_time_;
};

class Scheduler {
public:
  void scheduleTask(Task::TaskFunction func, std::chrono::steady_clock::time_point schedule_time) {
    std::lock_guard<std::mutex> lock(mutex_);
    tasks_.emplace(Task(std::move(func), schedule_time));
    cv_.notify_one();

  }

  void run() {
    while (!stop_) {
      std::unique_lock<std::mutex> lock(mutex_);
      
      if (tasks_.empty()) {
        cv_.wait(lock);
      } else {
        auto now = std::chrono::steady_clock::now();
        if (tasks_.top().getScheduleTime() <= now) {
          auto task = tasks_.top();
          tasks_.pop();
          lock.unlock();
          task.execute();
        } else {
          cv_.wait_until(lock, tasks_.top().getScheduleTime());
        }
      }
    }
  }

  void stop() {
    stop_ = true;
    cv_.notify_all();
  }

private:
  struct TaskComparator {
    bool operator()(const Task& lhs, const Task& rhs) const {
      return lhs.getScheduleTime() > rhs.getScheduleTime();
    }
  };

  std::priority_queue<Task, std::vector<Task>, TaskComparator> tasks_;
  std::mutex mutex_;
  std::condition_variable cv_;
  std::atomic<bool> stop_{false};
};

int main() {
  Scheduler scheduler;
  std::thread scheduler_thread([&scheduler]() { scheduler.run(); });

  auto now = std::chrono::steady_clock::now();

  scheduler.scheduleTask([]() { std::cout << "Task 1 executed\n"; }, now + std::chrono::seconds(2));
  scheduler.scheduleTask([]() { std::cout << "Task 2 executed\n"; }, now + std::chrono::seconds(1));
  scheduler.scheduleTask([]() { std::cout << "Task 3 executed\n"; }, now + std::chrono::seconds(3));

  std::this_thread::sleep_for(std::chrono::seconds(4));
  scheduler.stop();
  scheduler_thread.join();

  return 0;
}
