//! Example of using a thread-safe non-const lazy static global singleton
//! For more: https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
mod lazy_static;
mod once_cell;

/// Public struct with a private field
/// This struct can be accessed outside of the module, but not constructed due to private fields.
/// https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
#[allow(dead_code)]
pub struct Database {
    query_count: i32,
}

impl Database {
    /// Private constructor for the Database struct.
    /// Thus this struct cannot be constructed outside of this module.
    fn new() -> Database {
        Database { query_count: 0 }
    }

    /// Logs an sql query
    #[allow(dead_code)]
    pub fn query(&mut self, sql: &str) {
        self.query_count += 1;
        println!("Following query has been executed: {sql}");
    }

    /// Returns the number of queies requested
    #[allow(dead_code)]
    pub fn query_count(&self) -> i32 {
        self.query_count
    }
}
