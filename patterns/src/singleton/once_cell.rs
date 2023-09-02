//! This implementation uses the Lazy struct which is built on top of the OnceCell.
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

    #[test]
    fn simple() {
        DATABASE.lock().unwrap().query("SELECT * from users");
        assert_eq!(DATABASE.lock().unwrap().query_count(), 1);
    }

    #[test]
    fn multi_thread() {
        let t1 = thread::spawn(|| {
            DATABASE.lock().unwrap().query("Query from thread 1");
        });
        let t2 = thread::spawn(|| {
            DATABASE.lock().unwrap().query("Query from thread 2");
        });
        let _ = t1.join();
        let _ = t2.join();
        assert_eq!(DATABASE.lock().unwrap().query_count(), 3);
    }
}
