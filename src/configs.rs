pub struct AppConfigs {
    pub database: String,
    pub table: String,
}

impl Default for AppConfigs {
    fn default() -> Self {
        Self {
            database: "test-database".into(),
            table: "test-table".into(),
        }
    }
}
