#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

mod rts_gui;

use ecs_test_game::basic_legion::BasicLegion;
use ecs_test_game::gamesqlite::SqlIte;
use ecs_test_game::performance_map_legion::PerfMapLegion;
use ecs_test_game::relation_per_component::RelationPerComponent;
use ecs_test_game::rts::{GameImplementation, GuiSettings, WORLD_SIZE};
use ggez::graphics::{Color, Drawable};
use ggez::{Context, GameResult};
use glam::Vec2;

pub struct MainState {
    pub game: Box<dyn GameImplementation>,
    pub target_game_type: TargetGameType,
    pub egui_backend: ggez_egui::EguiBackend,
    pub gui_settings: GuiSettings,
    pub loaded_universes: usize,
    pub draw_time: u128,
    pub update_time: u128,
}
#[derive(Debug, PartialEq)]
pub enum TargetGameType {
    TargetBasicLegion,
    TargetPerfMapLegion,
    TargetSqlite,
    TargetRelationPerComp,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        MainState {
            game: Box::new(RelationPerComponent::new()),
            target_game_type: TargetGameType::TargetRelationPerComp,
            egui_backend: ggez_egui::EguiBackend::new(ctx),
            gui_settings: GuiSettings {
                meet_distance: 10.0,
                universe: 0,
                requested_universe_count: 1,
                entity_count: 1000,
            },
            loaded_universes: 0,
            draw_time: 0,
            update_time: 0,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Save update time:
        let start = std::time::Instant::now();
        // Update game:
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.game.update(dt, &self.gui_settings);
        let egui_ctx = self.egui_backend.ctx();
        egui::Window::new("egui-window").show(&egui_ctx, |ui| {
            ui.label("Meet distance");
            ui.add(egui::DragValue::new(&mut self.gui_settings.meet_distance).speed(0.1));
            ui.label("Universe");
            ui.add(egui::DragValue::new(&mut self.gui_settings.universe).speed(0.1));
            ui.label("Requested universe count");
            ui.add(
                egui::DragValue::new(&mut self.gui_settings.requested_universe_count).speed(0.1),
            );
            ui.label("Entity count");
            ui.add(egui::DragValue::new(&mut self.gui_settings.entity_count).speed(0.1));

            // Add ui for self.loaded_universes
            ui.label(format!("Loaded universes: {}", self.loaded_universes));
            ui.label(format!("FPS: {}", ggez::timer::fps(ctx)));
            ui.label(format!(
                "Time delta: {}ms",
                ggez::timer::delta(ctx).as_millis()
            ));
            ui.label(format!("Draw time: {}us", self.draw_time));
            ui.label(format!("Update time: {}us", self.update_time));
            let response = egui::ComboBox::from_label("Game type")
                .selected_text(format!("{:?}", self.target_game_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.target_game_type,
                        TargetGameType::TargetBasicLegion,
                        "BasicLegion",
                    );
                    ui.selectable_value(
                        &mut self.target_game_type,
                        TargetGameType::TargetPerfMapLegion,
                        "PerfMapLegion",
                    );
                    ui.selectable_value(
                        &mut self.target_game_type,
                        TargetGameType::TargetSqlite,
                        "Sqlite",
                    );
                    ui.selectable_value(
                        &mut self.target_game_type,
                        TargetGameType::TargetRelationPerComp,
                        "RelationPerComp",
                    );
                })
                .response;
            if ui.button("Reload").clicked() {
                match self.target_game_type {
                    TargetGameType::TargetBasicLegion => {
                        self.game = Box::new(BasicLegion::new());
                    }
                    TargetGameType::TargetPerfMapLegion => {
                        self.game = Box::new(PerfMapLegion::new());
                    }
                    TargetGameType::TargetSqlite => {
                        self.game = Box::new(SqlIte::new());
                    }
                    TargetGameType::TargetRelationPerComp => {
                        self.game = Box::new(RelationPerComponent::new());
                    }
                }
                self.loaded_universes = 0;
            }
        });
        if self.loaded_universes < self.gui_settings.requested_universe_count {
            self.game
                .load_universe(self.loaded_universes, self.gui_settings.entity_count);
            self.loaded_universes += 1;
        }
        if self.loaded_universes > self.gui_settings.requested_universe_count {
            self.loaded_universes -= 1;
            self.game.unload_universe(self.loaded_universes);
        }
        // Save update time:
        self.update_time = start.elapsed().as_micros();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        // Measure time to draw:
        let start = std::time::Instant::now();
        // Batch draw the units:
        let mut batch = ggez::graphics::MeshBuilder::new();
        for (position, color) in self.game.get_unit_positions(self.gui_settings.universe) {
            batch
                .circle(ggez::graphics::DrawMode::fill(), position, 10.0, 2.0, color)
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
        self.game
            .on_click(self.gui_settings.universe, Vec2::new(_x as f32, _y as f32));
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
    // Add key and mouse listeners for ggez_egui:
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez");

    cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
    cb = cb.window_mode(ggez::conf::WindowMode::default().dimensions(WORLD_SIZE, WORLD_SIZE));
    cb = cb.window_mode(ggez::conf::WindowMode::default().resizable(true));

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx);
    ggez::event::run(ctx, event_loop, state)
}
