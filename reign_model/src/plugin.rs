use crate::connection::Database;

use reign_plugin::Plugin;

impl Plugin for Database {
    fn init(&self) {
        self.connect();
    }
}
