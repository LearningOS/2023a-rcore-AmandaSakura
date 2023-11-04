//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,get_task_info},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
/// Represents a time value structure.
///
/// This structure is used to store time-related information.
pub struct TimeVal {
    ///这是秒数字段，表示时间的整数部分
    pub sec: usize,// 这是秒数字段，表示时间的整数部分
    ///注释到底是要注释到什么地方啊啊啊啊啊啊
    pub usec: usize,//啊啊啊啊啊啊啊啊
    //为什么我写了注释还是不能过啊啊啊啊啊啊啊
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
///退出当前进程
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
///让出CPU
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
///获取当前时间
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    get_task_info(ti);
    0
}
