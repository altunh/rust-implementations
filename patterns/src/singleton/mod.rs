//! Example of using a thread-safe non-const lazy static global singleton
//! For more: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

mod lazy_static;
mod once_cell_lazy;
mod once_cell_lazy_mutex;
mod std_once_lock;

/// Public struct with a private field
/// This struct can be accessed outside of the module, but not constructed due to private fields.
/// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
#[allow(dead_code)]
pub struct Database {
    count: AtomicU32,
}

/// Interface is designed to work with mostly shared references to Database.
impl Database {
    /// Private constructor for the Database struct.
    /// Thus this struct cannot be constructed outside of this module.
    fn new() -> Database {
        Database {
            count: AtomicU32::new(0),
        }
    }

    // Internal method for adding 1 to atomic count
    #[inline]
    fn add_one(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// Logs a select query with a shared reference
    #[allow(dead_code)]
    pub fn select(&self, sql: &str) {
        self.add_one();
        println!("Select Query: {sql}");
    }

    /// Logs an insert query with an exclusive reference
    pub fn insert(&mut self, sql: &str) {
        self.add_one();
        println!("Insert Query: {sql}");
    }

    /// Returns the number of queies requested
    #[allow(dead_code)]
    pub fn count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }

    // Reset count for testing purposes, since tests share the same static Database.
    pub fn reset(&self) {
        self.count.store(0, Ordering::Relaxed);
    }
}
