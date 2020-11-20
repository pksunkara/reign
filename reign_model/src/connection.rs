use diesel::r2d2::{ConnectionManager, Pool};
use once_cell::sync::OnceCell;

#[cfg(feature = "model-postgres")]
use diesel::PgConnection as Connection;

static DB: OnceCell<Pool<ConnectionManager<Connection>>> = OnceCell::new();

pub struct Database {
    url: String,
}

impl Database {
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self { url: url.into() }
    }

    pub fn connect(&self) {
        let manager = ConnectionManager::<Connection>::new(&self.url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Unable to connect to the database");

        if DB.set(pool).is_err() {
            panic!("Unable to store the database connection");
        }
    }

    pub fn get_opt() -> Option<&'static Pool<ConnectionManager<Connection>>> {
        DB.get()
    }

    pub fn get() -> &'static Pool<ConnectionManager<Connection>> {
        Self::get_opt().expect("Database must be connected before using it")
    }
}
