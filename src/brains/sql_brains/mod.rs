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
    fn execute(&mut self, statement: &str, params: Vec<f32>);
    fn get_entities(&mut self, universe_id: usize, query_posxy_col: &str) -> Vec<ExportEntity>;
}
