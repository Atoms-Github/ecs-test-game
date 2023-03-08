use futures::StreamExt;
use ggez::graphics::Color;
use sqlx::*;

use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::duckdb::InterfaceDuckDB;
use crate::brains::sql_interfaces::{InterfaceType, SqlInterface, SqlStatement};
use crate::Point;

pub struct InterfacePostgres {
	conn: PgConnection,
}
impl SqlInterface for InterfacePostgres {
	fn new() -> Self {
		let conn = PgConnection::connect("postgres::memory:");
		let conn = futures::executor::block_on(conn).unwrap();
		InterfacePostgres { conn }
	}

	fn execute_batch(&mut self, statements: Vec<SqlStatement>) {
		for statement in statements {
			self.execute_single(statement);
		}
	}

	fn get_entities(&mut self, query_xyc: SqlStatement) -> Vec<ExportEntity> {
		let mut ents = Vec::new();
		let query = sqlx::query(query_xyc.statement.as_str());
		let mut rows = futures::executor::block_on(query.fetch_all(&mut self.conn)).unwrap();
		for row in rows.iter_mut() {
			let x = row.try_get::<f32, _>("x").unwrap();
			let y = row.try_get::<f32, _>("y").unwrap();
			let c = row.try_get::<f32, _>("color").unwrap();

			let e = ExportEntity {
				position: Point::new(x, y),
				blue:     c,
			};
			ents.push(e);
		}

		ents
	}

	fn execute_single(&mut self, statement: SqlStatement) {
		let statement = sqlx::query(statement.statement.as_str()).execute(&mut self.conn);
		futures::executor::block_on(statement).unwrap();
	}

	fn get_type() -> InterfaceType {
		InterfaceType::Postgres
	}
}
