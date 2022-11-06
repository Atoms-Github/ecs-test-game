pub trait DbSys {
    fn execute(&mut self, query: &str);
    fn execute_with_floats(&mut self, query: &str, values: Vec<f32>);
    fn get_name(&self) -> &'static str;
    fn get_type_cheat(&self) -> DbSysType;
}
pub enum DbSysType<'a> {
    Sqlite(&'a DbSqlIte),
    DuckDB(&'a DbDuckDb),
}
pub struct DbSqlIte {
    pub conn: sqlite::Connection,
}
pub struct DbDuckDb {
    pub conn: duckdb::Connection,
}
impl DbSys for DbSqlIte {
    fn execute(&mut self, query: &str) {
        self.conn.execute(query).unwrap();
    }
    fn execute_with_floats(&mut self, query: &str, values: Vec<f32>) {
        let mut stmt = self.conn.prepare(query).unwrap();
        for (i, value) in values.iter().enumerate() {
            stmt = stmt.bind(i + 1, *value as f64).unwrap();
        }
        stmt.next().unwrap();
    }
    fn get_name(&self) -> &'static str {
        "Sqlite"
    }
    fn get_type_cheat(&self) -> DbSysType {
        DbSysType::Sqlite(self)
    }
}
impl DbSys for DbDuckDb {
    fn execute(&mut self, query: &str) {}
    fn execute_with_floats(&mut self, query: &str, values: Vec<f32>) {}
    fn get_name(&self) -> &'static str {
        "DuckDB"
    }
    fn get_type_cheat(&self) -> DbSysType {
        DbSysType::DuckDB(self)
    }
}
