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
    use crate::singleton::{Database, SaferDatabase};
    use once_cell::sync::Lazy;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// Lazy is guaranteed to be initialized, and is an equivalent of lazy_static without any macros.
    #[test]
    fn lazy_static_immut() {
        static DATABASE: Lazy<Database> = Lazy::new(|| Database::new());
        DATABASE.query_immut("from main thread");
        assert_eq!(DATABASE.count_mut(), 0);
    }

    /// Lazy static with mutability using Mutex
    #[test]
    fn lazy_static_mut() {
        static DATABASE: Lazy<Mutex<Database>> = Lazy::new(|| Mutex::new(Database::new()));

        let mut handles = Vec::new();
        for i in 1..=100 {
            let handle = thread::spawn(move || {
                DATABASE
                    .lock()
                    .unwrap()
                    .query_mut(&format!("Query from thread {i}"))
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(DATABASE.lock().unwrap().count_mut(), 100);
    }

    /// Avoiding Mutex using an atomic counter
    #[test]
    fn lazy_static_immut_atomic() {
        static DATABASE: Lazy<SaferDatabase> = Lazy::new(|| SaferDatabase::new());

        let mut handles = Vec::new();
        for i in 1..=100 {
            let handle = thread::spawn(move || {
                DATABASE.query_immut(&format!("from thread {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(DATABASE.count(), 100);
    }

    /// Avoiding Mutex using an atomic counter for a nonstatic variable
    #[test]
    fn lazy_nonstatic_immut_atomic() {
        let database: Lazy<Arc<SaferDatabase>> = Lazy::new(|| Arc::new(SaferDatabase::new()));

        let mut handles = Vec::new();
        for i in 1..=100 {
            let db = Arc::clone(&database);
            let handle = thread::spawn(move || {
                db.query_immut(&format!("from thread {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(database.count(), 100);
    }

    /// Lazily initialized thread-safe singleton with mutability.
    #[test]
    fn lazy_nonstatic_mut() {
        let database: Lazy<Arc<Mutex<Database>>> =
            Lazy::new(|| Arc::new(Mutex::new(Database::new())));

        let mut handles = Vec::new();
        for i in 1..=100 {
            let db = Arc::clone(&database);
            let handle = thread::spawn(move || {
                db.lock().unwrap().query_mut(&format!("from thread {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(database.lock().unwrap().count_mut(), 100);
    }
}
