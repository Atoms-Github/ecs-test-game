use duckdb::types::{ToSqlOutput, Value};
use duckdb::{params, params_from_iter, Connection, ParamsFromIter, Statement, ToSql};
use ggez::graphics::Color;

use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::{InterfaceType, SqlArgument, SqlInterface, SqlStatement};
use crate::Point;

pub struct InterfaceDuckDB {
	conn: Connection,
}

impl ToSql for SqlArgument {
	fn to_sql(&self) -> duckdb::Result<ToSqlOutput<'_>> {
		match self {
			SqlArgument::Float(x) => Ok(ToSqlOutput::Owned(Value::Float(*x))),
			SqlArgument::Blob(x) => Ok(ToSqlOutput::Owned(Value::Blob(x.clone()))),
		}
	}
}

impl SqlInterface for InterfaceDuckDB {
	fn new() -> Self {
		let mut conn = Connection::open_in_memory().unwrap();
		conn.set_prepared_statement_cache_capacity(1000);
		Self { conn }
	}

	fn execute_batch(&mut self, statements: Vec<SqlStatement>) {
		let transaction = self.conn.transaction().unwrap();
		for statement in statements {
			let mut stmt = transaction.prepare_cached(&statement.statement).unwrap();
			stmt.execute(params_from_iter(statement.params)).unwrap();
		}
		transaction.commit().unwrap();
	}

	fn get_entities(&mut self, query_xyci: SqlStatement) -> Vec<ExportEntity> {
		let mut stmt = self.conn.prepare_cached(query_xyci.statement.as_str()).unwrap();
		let mut rows = stmt.query(params_from_iter(query_xyci.params)).unwrap();
		let mut ents = Vec::new();
		while let Some(row) = rows.next().unwrap() {
			let ent = ExportEntity {
				position:  Point::new(row.get(0).unwrap(), row.get(1).unwrap()),
				blue:      row.get(2).unwrap(),
				entity_id: row.get(3).unwrap(),
			};
			ents.push(ent);
		}
		ents
	}

	fn get_image(&mut self, query_image: SqlStatement) -> Vec<u8> {
		let mut stmt = self.conn.prepare_cached(query_image.statement.as_str()).unwrap();
		let mut rows = stmt.query(params_from_iter(query_image.params)).unwrap();
		
		let row = rows.next().unwrap().unwrap();
		let blob: Vec<u8> = row.get(0).unwrap();
		assert!(rows.next().unwrap().is_none(), "More than one row returned for image query");
		blob
	}

	fn execute_single(&mut self, statement: SqlStatement) {
		self.conn
			.prepare_cached(&statement.statement)
			.unwrap()
			.execute(params_from_iter(statement.params))
			.unwrap();
	}

	fn get_type() -> InterfaceType {
		InterfaceType::DuckDB
	}
}
