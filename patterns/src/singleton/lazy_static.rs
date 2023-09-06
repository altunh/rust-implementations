//! This implementation uses the lazy_static macro from the lazy_static crate.
//! lazy_static: https://crates.io/crates/lazy_static

use lazy_static::lazy_static;
use std::sync::Mutex;

use super::Database;

lazy_static! {
    /// Thread-safe lazy initialized singleton exported from the database module
    #[allow(unused_variables)]
    pub static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

#[cfg(test)]
mod tests {
    use super::DATABASE;
    use std::thread;

    #[test]
    fn select_insert() {
        let mut db = DATABASE.lock().unwrap();
        db.select("Get Users");
        assert_eq!(db.count(), 1);
        db.insert("Insert User");
        assert_eq!(db.count(), 2);
        db.reset();
    }

    #[test]
    fn multithreaded() {
        let mut handles = Vec::new();
        for i in 1..=5 {
            let handle = thread::spawn(move || {
                DATABASE
                    .lock()
                    .unwrap()
                    .select(&format!("Query from thread {i}"));
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(DATABASE.lock().unwrap().count(), 5);
        DATABASE.lock().unwrap().reset();
    }

    #[test]
    fn multithreaded_fail() {
        let mut handles = Vec::new();
        for i in 1..=5 {
            let handle = thread::spawn(move || {
                DATABASE
                    .lock()
                    .unwrap()
                    .select(&format!("Query from thread {i}"));
            });
            handles.push(handle);
        }
        DATABASE.lock().unwrap().insert("SOME INSERT");
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
