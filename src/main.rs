#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::brains::sql_brains::brain_sql::BrainSql;
use ecs_test_game::brains::sql_brains::sql_flat_table::BrainSqlFlatTable;
use ecs_test_game::brains::sql_interfaces::duckdb::InterfaceDuckDB;
use ecs_test_game::brains::sql_interfaces::sqlite::InterfaceSqlite;
use ecs_test_game::brains::sql_interfaces::SqlInterface;
use ecs_test_game::brains::Brain;
use ecs_test_game::challenges::get_nearest::ChallengeGetNearest;
use ecs_test_game::challenges::rts::ChallengeRts;
use ecs_test_game::challenges::spacial_array::ChallengeSpatialArray;
use ecs_test_game::challenges::Challenge;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::{BrainType, ChallengeType, GuiSettings};
use ecs_test_game::{test_controller, MAP_SIZE};
use ggez::graphics::{Color, Drawable};
use ggez::input::mouse::position;
use ggez::{Context, GameResult};
use glam::Vec2;
use std::io::read_to_string;

pub struct MainState {
    pub test_controller: TestController,
    pub egui_backend: ggez_egui::EguiBackend,
    pub gui_settings: GuiSettings,
    pub draw_time: u128,
    pub update_time: u128,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        let settings = GuiSettings::new();

        MainState {
            test_controller: TestController::gen_test_controller(&settings),
            egui_backend: ggez_egui::EguiBackend::new(ctx),
            gui_settings: settings,
            draw_time: 0,
            update_time: 0,
        }
    }

    fn reload(&mut self) {
        self.test_controller = TestController::gen_test_controller(&self.gui_settings);
    }
}

impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Save update time:
        let start = std::time::Instant::now();
        // Update game:
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let egui_ctx = self.egui_backend.ctx();
        egui::Window::new("Settings").show(&egui_ctx, |ui| {
            ui.label(format!("Draw time: {}us", self.draw_time));
            ui.label(format!("Update time: {}us", self.update_time));
            ui.label(format!("FPS: {}", ggez::timer::fps(ctx)));
            ui.label(format!(
                "Time delta: {}ms",
                ggez::timer::delta(ctx).as_millis()
            ));
            self.gui_settings.draw(ui);
            if ui.button("Save Graph").clicked() {
                self.test_controller.save_graph("graph.png");
            }
            if ui.button("Reload").clicked() {
                self.reload();
            }
        });
        self.update_time = start.elapsed().as_micros();
        self.test_controller.tick(dt, &self.gui_settings);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        // Measure time to draw:
        let start = std::time::Instant::now();
        // Batch draw the units:
        let mut batch = ggez::graphics::MeshBuilder::new();
        for entity in self
            .test_controller
            .brain
            .get_entities(self.gui_settings.view_universe)
        {
            batch
                .circle(
                    ggez::graphics::DrawMode::fill(),
                    entity.position,
                    5.0,
                    0.1,
                    Color::from((1.0, 1.0, entity.blue, 1.0)),
                )
                .unwrap();
        }

        let mesh = batch.build(ctx);
        if let Ok(existing_mesh) = mesh {
            ggez::graphics::draw(ctx, &existing_mesh, (Vec2::new(0., 0.),))?;
        }

        let end = std::time::Instant::now();
        self.draw_time = (end - start).as_micros();

        ggez::graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
        ggez::graphics::present(ctx).unwrap();
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.egui_backend.input.mouse_button_down_event(button);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.egui_backend.input.mouse_button_up_event(button);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.egui_backend.input.mouse_motion_event(x, y);
    }
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("ECS Benchmark", "ggez");

    cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
    cb = cb.window_mode(ggez::conf::WindowMode::default().dimensions(MAP_SIZE, MAP_SIZE).resizable(true));


    let (mut ctx, event_loop) = cb.build()?;
    let mut state = MainState::new(&mut ctx);
    ggez::event::run(ctx, event_loop, state)
}
