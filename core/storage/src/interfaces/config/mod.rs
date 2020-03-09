// Built-in deps
// External imports
use diesel::prelude::*;
// Workspace imports
// Local imports
use self::records::ServerConfig;
use crate::StorageProcessor;

pub mod records;

pub trait ConfigInterface {
    fn load_config(&self) -> QueryResult<ServerConfig>;
}

impl ConfigInterface for StorageProcessor {
    fn load_config(&self) -> QueryResult<ServerConfig> {
        use crate::schema::server_config::dsl::*;
        server_config.first(self.conn())
    }
}
