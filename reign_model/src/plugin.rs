use diesel::r2d2::{ConnectionManager, Pool};
use reign_boot::{once_cell::sync::OnceCell, Plugin};

#[cfg(feature = "model-postgres")]
use diesel::PgConnection as Connection;

static DB: OnceCell<Pool<ConnectionManager<Connection>>> = OnceCell::new();

pub struct DatabasePlugin {
    url: String,
}

impl DatabasePlugin {
    pub fn new<S: Into<String>>(url: S) -> Self {
        Self { url: url.into() }
    }

    pub fn get() -> &'static Pool<ConnectionManager<Connection>> {
        DB.get()
            .expect("Database must be connected before using it")
    }
}

impl Plugin for DatabasePlugin {
    fn init(&self) {
        let manager = ConnectionManager::<Connection>::new(&self.url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Unable to connect to the database");

        if DB.set(pool).is_err() {
            panic!("Unable to store the database connection");
        }
    }
}