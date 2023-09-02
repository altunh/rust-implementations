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
    fn simple() {
        let mut db = DATABASE.lock().unwrap();
        db.query("SELECT * from users");
        assert_eq!(db.query_count(), 1);
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
