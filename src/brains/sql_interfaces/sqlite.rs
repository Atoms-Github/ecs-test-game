use ggez::filesystem::open;
use ggez::graphics::Color;
use glam::*;
use legion::systems::CommandBuffer;
use legion::*;
use rand::Rng;
use rusqlite::types::{ToSqlOutput, Value};
use rusqlite::*;

use crate::brains::com::ExportEntity;
use crate::brains::sql_interfaces::{sqlite, InterfaceType, SqlArgument, SqlInterface, SqlStatement};
use crate::ui::ui_settings::GuiSettings;

pub struct InterfaceSqlite {
	connection: Connection,
}

impl ToSql for SqlArgument {
	fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
		match self {
			SqlArgument::Float(x) => Ok(ToSqlOutput::Owned(Value::Real(*x as f64))),
			SqlArgument::Blob(x) => Ok(ToSqlOutput::Owned(Value::Blob(x.clone()))),
		}
	}
}

impl SqlInterface for InterfaceSqlite {
	fn new() -> Self {
		let connection = Connection::open_in_memory().unwrap();

		InterfaceSqlite { connection }
	}

	fn execute_batch(&mut self, statements: Vec<SqlStatement>) {
		let transaction = self.connection.transaction().unwrap();
		for statement in statements {
			transaction
				.prepare_cached(&statement.statement)
				.unwrap()
				.execute(params_from_iter(statement.params))
				.unwrap();
		}
		transaction.commit().unwrap();
	}

	fn get_entities(&mut self, query_xyci: SqlStatement) -> Vec<ExportEntity> {
		let mut statement = self.connection.prepare_cached(&query_xyci.statement).unwrap();
		let mut rows = statement
			.query_map(params_from_iter(query_xyci.params), |row| {
				Ok(ExportEntity {
					position:  Vec2::new(row.get(0)?, row.get(1)?),
					blue:      row.get(2)?,
					entity_id: row.get(3)?,
				})
			})
			.unwrap();
		let mut positions_and_teams = Vec::new();
		while let Some(row) = rows.next() {
			if let Ok(row) = row {
				positions_and_teams.push(row);
			}
		}
		positions_and_teams
	}

	fn get_image(&mut self, query_image: SqlStatement) -> Vec<u8> {
		let mut statement = self.connection.prepare_cached(&query_image.statement).unwrap();
		let mut blobs = statement
			.query_map(params_from_iter(query_image.params), |row| {
				let blob: Vec<u8> = row.get(0).unwrap();
				Ok(blob)
			})
			.unwrap();

		let blob = blobs.next().unwrap().unwrap();
		assert!(blobs.next().is_none());
		blob
	}

	fn execute_single(&mut self, statement: SqlStatement) {
		self.connection
			.prepare_cached(&statement.statement)
			.unwrap()
			.execute(params_from_iter(statement.params))
			.unwrap();
	}

	fn get_type() -> InterfaceType {
		InterfaceType::Sqlite
	}
}
