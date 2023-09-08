//! This implementation of Singleton uses `std::sync::OnceLock`.
//! OnceLock is thread-safe and can be used in statics.
//! To introduce mutability, use either a Mutex or an RwLock
//! To use a nonstatic singleton variable across threads, wrap it with Arc.

#[cfg(test)]
mod tests {
    use crate::singleton::Database;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::thread;

    // OnceLock is stable and can be used with statics.
    #[test]
    fn oncelock_static_immut() {
        static DATABASE: OnceLock<Database> = OnceLock::new();
        assert!(DATABASE.get().is_none());

        let db = DATABASE.get_or_init(|| Database::new());
        assert!(DATABASE.get().is_some());

        db.query_immut("from main thread");
        assert_eq!(db.count_mut(), 0);
    }

    /// OnceLock is thread-safe, but static mut requires unsafe code.
    #[test]
    fn oncelock_static_mut_unsafe() {
        static mut DATABASE: OnceLock<Database> = OnceLock::new();
        unsafe { DATABASE.get_or_init(|| Database::new()) };

        let mut handles = Vec::new();
        for i in 1..=100 {
            let handle = thread::spawn(move || {
                if i % 2 == 0 {
                    unsafe { DATABASE.get() }
                        .unwrap()
                        .query_immut(&format!("from thread {i}"));
                } else {
                    unsafe { DATABASE.get_mut() }
                        .unwrap()
                        .query_mut(&format!("from thread {i}"));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(unsafe { DATABASE.get() }.unwrap().count_mut(), 50);
    }

    /// You can introduce mutability to a static OnceLock using Mutex or RwLock
    #[test]
    fn oncelock_static_mut_safe() {
        static DATABASE: OnceLock<Mutex<Database>> = OnceLock::new();

        let mut handles = Vec::new();
        for i in 1..=100 {
            let handle = thread::spawn(move || {
                let db = DATABASE.get_or_init(|| Mutex::new(Database::new()));
                db.lock().unwrap().query_mut(&format!("from thread {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(DATABASE.get().unwrap().lock().unwrap().count_mut(), 100);
    }

    /// For nonstatic variables, to safely share and protect the OnceLock data between threads use Arc<Mutex<Data>>
    #[test]
    fn oncelock_nonstatic_safe() {
        let database: OnceLock<Arc<Mutex<Database>>> = OnceLock::new();
        let db = database.get_or_init(|| Arc::new(Mutex::new(Database::new())));
        assert_eq!(db.lock().unwrap().count_mut(), 0);

        let mut handles = Vec::new();
        for i in 1..=100 {
            let db = Arc::clone(database.get().unwrap());
            let handle = thread::spawn(move || {
                db.lock()
                    .unwrap()
                    .query_mut(&format!("Exclusive reference {i}"));
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        assert_eq!(database.get().unwrap().lock().unwrap().count_mut(), 100);
    }
}
