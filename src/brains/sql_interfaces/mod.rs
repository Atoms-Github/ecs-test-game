pub mod duckdb;

use crate::brains::com::ExportEntity;
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;
use crate::Point;
use ggez::graphics::Color;
use std::backtrace::Backtrace;

pub trait SqlInterface {
    type PreppedStatement;
    fn new() -> Self;
    fn execute(&mut self, statement: SqlStatement);
    fn get_entities(&mut self, query_xyc: SqlStatement) -> Vec<ExportEntity>;
}
pub struct SqlStatement {
    pub statement: String,
    pub params: Vec<f32>,
}
impl SqlStatement {
    pub fn new(statement: &str, params: Vec<f32>) -> SqlStatement {
        SqlStatement {
            statement: statement.to_string(),
            params,
        }
    }
}
