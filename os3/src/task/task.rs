//! Types related to task management

use super::TaskContext;
use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub start_time: usize,
    pub syscall_times: [u32;MAX_SYSCALL_NUM]
    // LAB1: Add whatever you need about the Task.
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskControlBlock {
    pub fn update_syscall_times(&mut self, syscall_id:usize) {
        self.syscall_times[syscall_id] += 1;
    }
}