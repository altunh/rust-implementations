//! This implementation uses the Lazy struct which is built on top of the OnceCell.
//! Similar to OnceLock, Lazy is thread-safe and can be used with statics.
//! 
//! A standard library version of it is coming, but is currently nightly-only.
//! Crate used in this file: https://crates.io/crates/once_cell
//!
//! OnceCell: A cell which can be written to only once.
//! Non-sync: https://doc.rust-lang.org/stable/std/cell/struct.OnceCell.html
//! Sync: https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html
//!
//! LazyCell: A value which is initialized on the first access (unstable).
//! Non-Sync: https://doc.rust-lang.org/stable/std/cell/struct.LazyCell.html
//! Sync: https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html

#[cfg(test)]
mod tests {
    use crate::singleton::Database;
    use std::sync::{Arc, Mutex};
    use once_cell::sync::Lazy;
    use std::thread;

    #[test]
    fn lazy() {
        // static DATABASE: Lazy<Database> = Lazy::new(|| Database::new());
        // let mut db = &mut DATABASE;
        // db.insert("sql");
    }

    /// Manually ensure the borrowing rules and surround with unsafe blocks.
    #[test]
    fn multithreaded() {
        // let shared_db = unsafe { &DATABASE };
        // let mut handles = Vec::new();
        // for i in 1..=5 {
        //     let handle = thread::spawn(move || shared_db.select(&format!("Query from thread {i}")));
        //     handles.push(handle);
        // }
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        // assert_eq!(shared_db.count(), 5);
        // shared_db.reset();
    }

    #[test]
    #[should_panic]
    fn multithreaded_fail() {
        // let mut handles = Vec::new();
        // for i in 1..=5 {
        //     let handle = thread::spawn({
        //         let mutable_db = unsafe { &mut DATABASE };
        //         move || {
        //             mutable_db.insert(&format!("Query from thread {i}"));
        //         }
        //     });
        //     handles.push(handle);
        // }
        // for handle in handles {
        //     handle.join().unwrap();
        // }
        // assert_eq!(unsafe { &DATABASE }.count(), 5);
    }
}
