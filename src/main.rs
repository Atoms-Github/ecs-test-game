#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]

use ggez::graphics::{Color, Drawable};
use ggez::{Context, GameResult};
use ggez::input::mouse::position;
use glam::Vec2;
use ecs_test_game::brains::Brain;
use ecs_test_game::{MAP_SIZE, test_controller};
use ecs_test_game::brains::legion_sequential::BrainLegionSequential;
use ecs_test_game::challenges::Challenge;
use ecs_test_game::challenges::rts::RtsChallenge;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::{BrainType, ChallengeType, GuiSettings};


pub struct MainState {
    pub test_controller: TestController,
    pub egui_backend: ggez_egui::EguiBackend,
    pub gui_settings: GuiSettings,
    pub draw_time: u128,
    pub update_time: u128,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        let brain = Box::new(BrainLegionSequential::new());
        let brain2 = Box::new(BrainLegionSequential::new());
        let challenge = Box::new(RtsChallenge{
            units_count: 100,
        });
        let challenge2 = Box::new(RtsChallenge{
            units_count: 100,
        });
        MainState {
            test_controller: TestController::new(brain2, challenge2),
            egui_backend: ggez_egui::EguiBackend::new(ctx),
            gui_settings: GuiSettings {

                meet_distance: 10.0,
                view_universe: 0,
                universe_count: 1,
                entity_count: 100,
                brain_type: BrainType::LegionSequential,
                challenge_type: ChallengeType::Rts
            },
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
        let egui_ctx = self.egui_backend.ctx();
        egui::Window::new("Settings").show(&egui_ctx, |ui| {
            ui.label("Meet distance");
            ui.add(egui::DragValue::new(&mut self.gui_settings.meet_distance).speed(0.1));
            ui.label("Universe");
            ui.add(egui::DragValue::new(&mut self.gui_settings.view_universe).speed(0.1));
            ui.label("Requested universe count");
            ui.add(
                egui::DragValue::new(&mut self.gui_settings.universe_count).speed(0.1),
            );
            ui.label("Entity count");
            ui.add(egui::DragValue::new(&mut self.gui_settings.entity_count).speed(0.1));

            // Add ui for self.universe_count:


            ui.label(format!("FPS: {}", ggez::timer::fps(ctx)));
            ui.label(format!(
                "Time delta: {}ms",
                ggez::timer::delta(ctx).as_millis()
            ));
            ui.label(format!("Draw time: {}us", self.draw_time));
            ui.label(format!("Update time: {}us", self.update_time));
            egui::ComboBox::from_label("Brain type")
                .selected_text(format!("{:?}", self.gui_settings.brain_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.gui_settings.brain_type,
                        BrainType::LegionSequential,
                        "Legion sequential",
                    );
                })
                .response;
            egui::ComboBox::from_label("Challenge type")
                .selected_text(format!("{:?}", self.gui_settings.challenge_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.gui_settings.challenge_type,
                        ChallengeType::Rts,
                        "Rts",
                    );
                })
                .response;

            if ui.button("Reload").clicked() {
                let new_brain = match self.gui_settings.brain_type {
                    BrainType::LegionSequential => Box::new(BrainLegionSequential::new()),
                };
                let new_challenge = match self.gui_settings.challenge_type {
                    ChallengeType::Rts => Box::new(RtsChallenge {
                        units_count: self.gui_settings.entity_count,
                    }),
                };
                self.test_controller = TestController::new(new_brain, new_challenge);
                self.test_controller.init();
            }
        });
        self.update_time = start.elapsed().as_micros();
        self.test_controller.tick( dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        // Measure time to draw:
        let start = std::time::Instant::now();
        // Batch draw the units:
        let mut batch = ggez::graphics::MeshBuilder::new();
        for (position, color) in self.test_controller.brain.get_entities() {
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
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez");

    cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
    cb = cb.window_mode(ggez::conf::WindowMode::default().dimensions(MAP_SIZE, MAP_SIZE));
    cb = cb.window_mode(ggez::conf::WindowMode::default().resizable(true));

    let (mut ctx, event_loop) = cb.build()?;
    let mut state = MainState::new(&mut ctx);
    state.test_controller.init();
    ggez::event::run(ctx, event_loop, state)
}
