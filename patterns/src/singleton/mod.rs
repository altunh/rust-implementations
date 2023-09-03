//! Example of using a thread-safe non-const lazy static global singleton
//! For more: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

mod lazy_static;
mod once_cell_lazy;
mod std_once_lock;

/// Public struct with a private field
/// This struct can be accessed outside of the module, but not constructed due to private fields.
/// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
#[allow(dead_code)]
pub struct Database {
    count: AtomicU32,
}

impl Database {
    /// Private constructor for the Database struct.
    /// Thus this struct cannot be constructed outside of this module.
    fn new() -> Database {
        Database {
            count: AtomicU32::new(0),
        }
    }

    /// Logs an sql query
    #[allow(dead_code)]
    pub fn query(&self, sql: &str) {
        self.count.fetch_add(1, Ordering::Relaxed);
        println!("Following query has been executed: {sql}");
    }

    /// Returns the number of queies requested
    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }
}
