use duckdb::{Connection, params, params_from_iter, ParamsFromIter, Statement};
use ggez::graphics::Color;
use crate::brains::com::ExportEntity;
use crate::brains::sql_brains::SqlBrain;
use crate::Point;

pub struct DuckDB<'a> {
    conn: Connection,
    phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> SqlBrain for DuckDB<'a>{
    type PreppedStatement = Statement<'a>;
    fn new() -> Self {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.set_prepared_statement_cache_capacity(100);
        Self {
            conn,
            phantom: std::marker::PhantomData
        }
    }

    fn execute(&mut self, statement: &str, params: Vec<f32>) {
        self.conn.prepare_cached(statement).unwrap().execute(params_from_iter(params)).unwrap();
    }

    fn get_entities(&mut self, universe_id: usize)  -> Vec<ExportEntity> {
        // query table by rows
        let mut stmt = self.conn.prepare("SELECT position_x, position_y, color_r FROM entities").unwrap();
        let person_iter = stmt.query_map([], |row| {

            Ok(ExportEntity {
                pos: Point::new(row.get(0).unwrap(), row.get(1).unwrap()),
                color: Color{
                    r: row.get(2).unwrap(),
                    g: 0.5,
                    b: 0.5,
                    a: 1.0
                }
            })
        }).unwrap();
        return person_iter.collect::<Result<Vec<_>, _>>().unwrap();
    }
}
