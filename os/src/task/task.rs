//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task's syscall time
    pub call_times: [isize; 500],
}
//// The task control block (TCB) of a task.
impl TaskControlBlock {
    /// syscall time 次数加一
    pub fn calltime_add(&mut self, syscall_id: usize) {
        self.call_times[syscall_id] += 1;
    }

    /// 查询 syscalltime
    pub fn calltime(&self, syscall_id: usize) -> isize {
        self.call_times[syscall_id]
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
