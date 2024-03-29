pub mod duckdb;
pub mod postgresql;
pub mod sqlite;

use std::backtrace::Backtrace;

use ggez::graphics::Color;

use crate::brains::com::ExportEntity;
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;
use crate::Point;

pub trait SqlInterface {
	fn new() -> Self;
	fn execute_batch(&mut self, statements: Vec<SqlStatement>);
	fn get_entities(&mut self, query_xyci: SqlStatement) -> Vec<ExportEntity>;
	fn get_image(&mut self, query_image: SqlStatement) -> Vec<u8>;
	fn execute_single(&mut self, statement: SqlStatement);
	fn get_type() -> InterfaceType;
}
#[derive(Debug)]
pub struct SqlStatement {
	pub statement: String,
	pub params:    Vec<SqlArgument>,
}

#[derive(Debug)]
pub enum SqlArgument {
	Float(f32),
	Blob(Vec<u8>),
}

pub enum InterfaceType {
	Sqlite,
	DuckDB,
	Postgres,
}
impl SqlStatement {
	pub fn new_f32(statement: &str, params: Vec<f32>) -> SqlStatement {
		SqlStatement {
			statement: statement.to_string(),
			params:    params.into_iter().map(|x| SqlArgument::Float(x)).collect(),
		}
	}

	pub fn new(statement: &str, params: Vec<SqlArgument>) -> SqlStatement {
		SqlStatement {
			statement: statement.to_string(),
			params,
		}
	}
}
