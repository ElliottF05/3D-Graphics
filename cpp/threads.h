// Made with the help of GeeksForGeeks
// https://www.geeksforgeeks.org/thread-pool-in-cpp/

#pragma once

#include <functional>
#include <thread>
#include <condition_variable>
#include <mutex>
#include <queue>
#include <atomic>


class ThreadPool {
public:
    ThreadPool(int num_threads);
    ~ThreadPool();
    void addTask(std::function<void()> task);
    int getNumberOfActiveTasks();
    void waitUntilTasksFinished();
private: 
    // Vector to store worker threads 
    std::vector<std::thread> threads_; 

    // Queue of tasks 
    std::queue<std::function<void()> > tasks_; 

    // Mutex to synchronize access to shared data 
    std::mutex queue_mutex_; 

    // Condition variable to signal changes in the state of 
    // the tasks queue 
    std::condition_variable cv_; 

    // Flag to indicate whether the thread pool should stop 
    // or not 
    bool stop_ = false; 

    // Atomic counter to keep track of number of active tasks
    std::atomic<int> active_tasks_;
};
static ThreadPool threadPool = ThreadPool(20);