//! This implementation uses the Lazy struct which is built on top of the OnceCell.

use std::sync::Mutex;

use once_cell::sync::Lazy;

use super::Database;

/// Thread-safe lazy initialized singleton exported from the database module
/// Database cannot be constructed outside, and thus has a "private constructor".
/// This doesn't use macros like in lazy_static.
#[allow(dead_code)]
pub static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database::new()));

#[cfg(test)]
mod tests {
    use super::DATABASE;
    use std::thread;

    // #[test]
    // fn multithreaded() {
    //     let mut handles = Vec::new();
    //     for i in 1..=5 {
    //         let handle = thread::spawn(move || {
    //             DATABASE.lock().unwrap().select(&format!("Query from thread {i}"));
    //         });
    //         handles.push(handle);
    //     }
    //     for handle in handles {
    //         handle.join().unwrap();
    //     }
    //     assert_eq!(DATABASE.lock().unwrap().count(), 5);
    //     DATABASE.lock().unwrap().reset();
    // }
}
