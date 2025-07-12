

const MAX_FDS = 1024;
const WOKER_THREADS = 10;

pub struct WorkerQueue {
    queue: [0i32, MAX_FDS],
    head: i32,
    tail: i32,
    //pthread mutex lock
    //pthread condition
};

pub struct WorkerStats {
    fd_count: i32,
    active_fds: u32,
    cpu_usage: f32,
    avg_latency: f32,
    pending_tasks: u32,
    last_wakeup: u64,
}
