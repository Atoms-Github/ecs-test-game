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
pub mod game_legion;
pub mod rts;
use game_legion::*;
use rts::WORLD_SIZE;

pub const UNIVERSES: usize = 5;

pub trait GameImplementation {
    fn update(&mut self, ctx: &mut Context, settings: &GuiSettings);
    fn get_unit_positions(&self, world_id: usize) -> Vec<(glam::Vec2, Color)>;
}
pub struct GuiSettings {
    pub push_force: f32,
    pub drag_coefficient: f32,
    pub max_speed: f32,
    pub meet_distance: f32,
    pub universe: usize,
}

struct MainState<T: GameImplementation> {
    game: T,
    circle: ggez::graphics::Mesh,
    egui_backend: ggez_egui::EguiBackend,
    gui_settings: GuiSettings,
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
                push_force: 1500.0,
                drag_coefficient: 1.0,
                max_speed: 100.0,
                meet_distance: 30.0,
                universe: 0
            },
        })
    }
}

impl<T: GameImplementation> ggez::event::EventHandler<ggez::GameError> for MainState<T> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.game.update(ctx, &self.gui_settings);
        let egui_ctx = self.egui_backend.ctx();
        egui::Window::new("egui-window").show(&egui_ctx, |ui| {
            ui.add(egui::DragValue::new(&mut self.gui_settings.meet_distance).speed(0.1));
            ui.add(egui::DragValue::new(&mut self.gui_settings.universe).speed(0.1));
        });
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        self.game.get_unit_positions(self.gui_settings.universe).iter().for_each(|(pos, color)| {
            ggez::graphics::draw(
                ctx,
                &self.circle,
                ggez::graphics::DrawParam::default().dest(*pos).color(*color),
            )
            .unwrap();
            // canvas.draw(&self.circle, *pos);
        });

        // canvas.finish(ctx)?;
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
    // Add key and mouse listeners for ggez_egui:
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez");

    cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
    cb = cb.window_mode(ggez::conf::WindowMode::default().dimensions(WORLD_SIZE, WORLD_SIZE));
    cb = cb.window_mode(ggez::conf::WindowMode::default().resizable(true));

    let (mut ctx, event_loop) = cb.build()?;
    let mut world = GameLegion::new();
    for universe_id in 0..UNIVERSES {
        world.generate_world(universe_id);
    }
    let state = MainState::new(&mut ctx, world)?;
    ggez::event::run(ctx, event_loop, state)
}
