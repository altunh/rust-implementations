use super::Database;

#[allow(dead_code)]
fn send_query(db: &Database) {
    db.query("SELECT * ");
}

#[cfg(test)]
mod tests {
    use super::send_query;
    use crate::singleton::Database;
    use std::{sync::OnceLock, thread};

    #[test]
    fn lazy_static() {
        static DATABASE: OnceLock<Database> = OnceLock::new();
        thread::spawn(|| {
            DATABASE.get_or_init(|| Database::new());
        })
        .join()
        .unwrap();
        assert_eq!(DATABASE.get().unwrap().count(), 0);
        thread::spawn(|| {
            send_query(DATABASE.get().expect("Uninitialized database"));
        })
        .join()
        .unwrap();
        assert_eq!(DATABASE.get().unwrap().count(), 1);
    }

    /// OnceLock is a good hint that this variable is intentionally set once, i.e. Singleton.
    /// Used as a non-static, but instead as a top-level local variable.
    #[test]
    fn non_static() {
        let mut db: OnceLock<Database> = OnceLock::new();
        assert!(db.get().is_none());
        let _ = db.set(Database::new());
        assert!(db.get().is_some());
        send_query(db.get_mut().unwrap());
        assert_eq!(db.get().unwrap().count(), 1);
    }
}
