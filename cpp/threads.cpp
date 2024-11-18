// Made with the help of GeeksForGeeks
// https://www.geeksforgeeks.org/thread-pool-in-cpp/

#include "threads.h"
#include <atomic>
#include <functional>
#include <thread>
#include <mutex>
#include <utility>
#include <iostream>

// CONSTRUCTOR
ThreadPool::ThreadPool(int num_threads) : stop_(false), active_tasks_(0) {
    // Creating worker threads 
    for (int i = 0; i < num_threads; ++i) { 
        threads_.emplace_back([this] { 
            while (true) { 
                std::function<void()> task; 
                // The reason for putting the below code 
                // here is to unlock the queue before 
                // executing the task so that other 
                // threads can perform enqueue tasks 
                { 
                    // Locking the queue so that data 
                    // can be shared safely 
                    std::unique_lock<std::mutex> lock( 
                        queue_mutex_); 

                    // Waiting until there is a task to 
                    // execute or the pool is stopped 
                    cv_.wait(lock, [this] { 
                        return !tasks_.empty() || stop_; 
                    }); 

                    // exit the thread in case the pool 
                    // is stopped and there are no tasks 
                    if (stop_ && tasks_.empty()) { 
                        return; 
                    } 

                    // Get the next task from the queue 
                    task = std::move(tasks_.front()); 
                    tasks_.pop(); 
                } 

                task();
                active_tasks_--;
            } 
        }); 
    } 
}

// DESTRUCTOR
// Destructor to stop the thread pool 
ThreadPool::~ThreadPool() { 
    { 
        // Lock the queue to update the stop flag safely 
        std::unique_lock<std::mutex> lock(queue_mutex_); 
        stop_ = true; 
    } 

    // Notify all threads 
    cv_.notify_all(); 

    // Joining all worker threads to ensure they have 
    // completed their tasks 
    for (auto& thread : threads_) { 
        thread.join(); 
    } 
}

// Enqueue task for execution by the thread pool 
void ThreadPool::addTask(std::function<void()> task) { 
    active_tasks_++;
    // std::cout << active_tasks_ << std::endl;
    { 
        std::unique_lock<std::mutex> lock(queue_mutex_); 
        tasks_.emplace(std::move(task)); 
    } 
    cv_.notify_one(); 
} 

int ThreadPool::getNumberOfActiveTasks() {
    // std::cout << "Active tasks: " << active_tasks_.load(std::memory_order_seq_cst) << std::endl;
    return active_tasks_.load(std::memory_order_seq_cst);
}