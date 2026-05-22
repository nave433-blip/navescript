use threadpool::ThreadPool;

pub struct PoolHost {
    pool: ThreadPool,
}

impl PoolHost {
    pub fn new(size: u32) -> Self {
        PoolHost {
            pool: ThreadPool::new(size as usize),
        }
    }

    pub fn execute(&self, task_code: String) {
        self.pool.execute(move || {
            // In a full implementation, this would trigger the interpreter or compiler 
            // for the provided task_code string.
            println!("Executing task: {}", task_code);
        });
    }
}
