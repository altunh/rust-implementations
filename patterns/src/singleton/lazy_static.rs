//! This implementation uses the lazy_static macro from the lazy_static crate.
//! lazy_static: https://crates.io/crates/lazy_static

use lazy_static::lazy_static;
use std::sync::Mutex;

use super::Database;

lazy_static! {
    /// Thread-safe lazy initialized singleton exported from the database module
    pub static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

#[cfg(test)]
mod tests {
    use super::DATABASE;
    use std::thread;

    #[test]
    fn lazy_static_mutex() {
        let mut db = DATABASE.lock().unwrap();
        db.query_immut("from main thread");
        db.query_mut("from main thread");
        assert_eq!(db.count_mut(), 1);
        db.reset();
    }

    #[test]
    fn lazy_static_concurrent() {
        let mut handles = Vec::new();
        for i in 1..=100 {
            let handle = thread::spawn(move || {
                DATABASE
                    .lock()
                    .unwrap()
                    .query_mut(&format!("from thread {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(DATABASE.lock().unwrap().count_mut(), 100);
    }
}
