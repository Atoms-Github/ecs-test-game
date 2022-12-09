use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::{SqlInterface, SqlStatement};
use crate::Point;
use duckdb::{params, params_from_iter, Connection, ParamsFromIter, Statement};
use ggez::graphics::Color;

pub struct InterfaceDuckDB<'a> {
    conn: Connection,
    phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> SqlInterface for InterfaceDuckDB<'a> {
    type PreppedStatement = Statement<'a>;
    fn new() -> Self {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.set_prepared_statement_cache_capacity(100);
        Self {
            conn,
            phantom: std::marker::PhantomData,
        }
    }

    fn execute(&mut self, statement: SqlStatement) {
        self.conn
            .prepare_cached(statement.statement.as_str())
            .unwrap()
            .execute(params_from_iter(statement.params))
            .unwrap();
    }

    fn get_entities(&mut self, query_xyc: SqlStatement) -> Vec<ExportEntity> {
        let mut stmt = self.conn.prepare(query_xyc.statement.as_str()).unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(ExportEntity {
                    position: Point::new(row.get(0).unwrap(), row.get(1).unwrap()),
                    color: Color {
                        r: row.get(2).unwrap(),
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    },
                })
            })
            .unwrap();
        return person_iter.collect::<Result<Vec<_>, _>>().unwrap();
    }
}
