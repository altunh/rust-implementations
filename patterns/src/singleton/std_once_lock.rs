//! This implementation of Singleton uses `std::sync::OnceLock`.
//! OnceLock is thread-safe and can be used in statics.
//! Borrow rules for the value can be checked at compile time using the `get` and `get_mut` methods.
//! To introduce mutability, use either a Mutex or an RwLock
//! To use across threads, wrap it with Arc.

#[cfg(test)]
mod tests {
    use crate::singleton::Database;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread;

    /// OnceLock returns an Option on `OnceLock::get`
    #[test]
    #[should_panic]
    fn oncelock_immutable() {
        static DATABASE: OnceLock<Database> = OnceLock::new();
        assert!(DATABASE.get().is_none());

        let db = DATABASE.get_or_init(|| Database::new());
        db.select("Query with a shared reference");
    }

    /// Immutable static variable, cannot be obtained mutably.
    /// Obtaining shared references is safe and can be used across threads.
    #[test]
    fn oncelock_immutable_concurrent() {
        static DATABASE: OnceLock<Database> = OnceLock::new();

        let mut handles = Vec::new();
        for i in 1..=5 {
            let handle = thread::spawn(move || {
                let db = DATABASE.get_or_init(|| Database::new());
                db.select(&format!("Query from thread {i}"));
            });
            handles.push(handle);
        }

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());
    }

    /// Mutable static variables require unsafe code.
    /// You can introduce mutability to the singleton using a Mutex or an RwLock.
    #[test]
    fn oncelock_mutable() {
        static DATABASE: OnceLock<Mutex<Database>> = OnceLock::new();
        let db = DATABASE.get_or_init(|| Mutex::new(Database::new()));

        let mut db = db.lock().unwrap();
        db.insert("Query with an exclusive reference");
    }

    /// For concurrency, introduce Arc and protect it with either a Mutex or an RwLock.
    #[test]
    fn oncelock_mutable_concurrent() {
        static DATABASE: OnceLock<Arc<Mutex<Database>>> = OnceLock::new();

        let mut handles = Vec::new();
        for i in 1..=5 {
            let db = DATABASE.get_or_init(|| Arc::new(Mutex::new(Database::new())));
            let db = Arc::clone(db);
            let handle = thread::spawn(move || {
                db.lock()
                    .unwrap()
                    .insert(&format!("Exclusive reference {i}"));
            });
            handles.push(handle);
        }

        handles
            .into_iter()
            .for_each(|handle| handle.join().unwrap());
    }
}
