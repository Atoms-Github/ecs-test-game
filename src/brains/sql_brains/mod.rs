pub mod duckdb;

use std::backtrace::Backtrace;
use ggez::graphics::Color;
use crate::brains::{Brain, SystemType};
use crate::brains::com::ExportEntity;
use crate::Point;
use crate::ui::ui_settings::GuiSettings;
use crate::utils::FromTeam;

pub trait SqlBrain{
    type PreppedStatement;
    fn new() -> Self;
    fn execute(&mut self, statement: &str, params: Vec<f32>);
    fn get_entities(&mut self, universe_id: usize, query_posxy_col: &str) -> Vec<ExportEntity>;
}