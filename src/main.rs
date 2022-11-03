#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
pub mod gamedb;
use ggez::graphics::{Color, Drawable};
use ggez::{Context, GameResult};
use glam::Vec2;

pub mod game_legion;
pub mod rts;

use game_legion::*;
use rts::WORLD_SIZE;

pub trait GameImplementation {
    fn update(&mut self, ctx: &mut Context, settings: &GuiSettings);
    fn get_unit_positions(&self, unierse_id: usize) -> Vec<(glam::Vec2, Color)>;
    fn load_universe(&mut self, universe_id: usize);
    fn unload_universe(&mut self, unierse_id: usize);
    fn on_click(&mut self, universe_id: usize, position: Vec2);
}
#[derive(Clone)]
pub struct GuiSettings {
    pub meet_distance: f32,
    pub universe: usize,
    pub requested_universe_count: usize,
}

struct MainState<T: GameImplementation> {
    game: T,
    circle: ggez::graphics::Mesh,
    egui_backend: ggez_egui::EguiBackend,
    gui_settings: GuiSettings,
    loaded_universes: usize,
    draw_time: u128,
    update_time: u128,
}

impl<T: GameImplementation> MainState<T> {
    fn new(ctx: &mut Context, game_world: T) -> GameResult<MainState<T>> {
        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            glam::Vec2::new(0., 0.),
            10.0,
            2.0,
            Color::WHITE,
        )?;

        Ok(MainState {
            game: game_world,
            circle,
            egui_backend: ggez_egui::EguiBackend::new(ctx),
            gui_settings: GuiSettings {
                meet_distance: 30.0,
                universe: 0,
                requested_universe_count: 10,
            },
            loaded_universes: 0,
            draw_time: 0,
            update_time: 0,
        })
    }
}

impl<T: GameImplementation> ggez::event::EventHandler<ggez::GameError> for MainState<T> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Save update time:
        let start = std::time::Instant::now();

        self.game.update(ctx, &self.gui_settings);
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
            // Add ui for self.loaded_universes
            ui.label(format!("Loaded universes: {}", self.loaded_universes));
            ui.label(format!("FPS: {}", ggez::timer::fps(ctx)));
            ui.label(format!(
                "Time delta: {}ms",
                ggez::timer::delta(ctx).as_millis()
            ));
            ui.label(format!("Draw time: {}us", self.draw_time));
            ui.label(format!("Update time: {}us", self.update_time));
        });
        if self.loaded_universes < self.gui_settings.requested_universe_count {
            self.game.load_universe(self.loaded_universes);
            self.loaded_universes += 1;
        }
        if self.loaded_universes > self.gui_settings.requested_universe_count {
            self.game.unload_universe(self.loaded_universes);
            self.loaded_universes -= 1;
        }
        // Save update time:
        self.update_time = start.elapsed().as_micros();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        // Measure time to draw:
        let start = std::time::Instant::now();
        self.game
            .get_unit_positions(self.gui_settings.universe)
            .iter()
            .for_each(|(pos, color)| {
                ggez::graphics::draw(
                    ctx,
                    &self.circle,
                    ggez::graphics::DrawParam::default()
                        .dest(*pos)
                        .color(*color),
                )
                .unwrap();
            });
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
    let mut world = GameLegion::new();
    let state = MainState::new(&mut ctx, world)?;
    ggez::event::run(ctx, event_loop, state)
}
