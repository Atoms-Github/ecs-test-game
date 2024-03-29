#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_macros)]

use std::io::read_to_string;
use std::{env, path};

use ecs_test_game::brains::brain_legion::BrainLegion;
use ecs_test_game::brains::sql_brains::brain_sql::BrainSql;
use ecs_test_game::brains::sql_brains::sql_flat_table::BrainSqlFlatTable;
use ecs_test_game::brains::sql_interfaces::duckdb::InterfaceDuckDB;
use ecs_test_game::brains::sql_interfaces::sqlite::InterfaceSqlite;
use ecs_test_game::brains::sql_interfaces::SqlInterface;
use ecs_test_game::brains::Brain;
use ecs_test_game::challenges::ch_paint_closest::ChallengePaintClosest;
use ecs_test_game::challenges::ch_spatial_array::ChallengeSpatialArray;
use ecs_test_game::challenges::ch_units_shooting::ChallengeRts;
use ecs_test_game::challenges::ChallengeTrait;
use ecs_test_game::simulation_settings::Challenge;
use ecs_test_game::test_controller::TestController;
use ecs_test_game::ui::ui_settings::GuiSettings;
use ecs_test_game::{test_controller, MAP_SIZE};
use egui::epaint::image;
use ggez::event::{quit, KeyCode, KeyMods};
use ggez::graphics::{Color, DrawParam, Drawable};
use ggez::input::mouse::position;
use ggez::{Context, GameResult};
use glam::Vec2;

pub struct MainState {
	pub test_controller: TestController,
	pub egui_backend: ggez_egui::EguiBackend,
	pub gui_settings: GuiSettings,
	pub draw_time: u128,
	pub update_time: u128,
	pub entity_image_index: usize,
	pub frames: usize,
	pub image: Option<ggez::graphics::Image>,
}

impl MainState {
	fn new(ctx: &mut Context) -> MainState {
		let settings = GuiSettings::new();

		MainState {
			test_controller: TestController::gen_test_controller(ctx, &settings.simulation_settings),
			egui_backend: ggez_egui::EguiBackend::new(ctx),
			gui_settings: settings,
			draw_time: 0,
			update_time: 0,
			entity_image_index: 0,
			frames: 0,
			image: None,
		}
	}

	fn reload(&mut self, ctx: &mut Context) {
		self.test_controller =
			TestController::gen_test_controller(ctx, &self.gui_settings.simulation_settings);
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
			ui.label(format!("Time delta: {}ms", ggez::timer::delta(ctx).as_millis()));
			self.gui_settings.draw(ui);

			// If query challenge:
			if self.gui_settings.simulation_settings.challenge_type != Challenge::ComplexQuery {
				let ent_count =
					self.test_controller.brain.get_entities(self.gui_settings.view_universe).len();
				ui.label(format!("Entity count: {}", ent_count));
			}

			if ui.button("Save Graph").clicked() {
				self.test_controller.save_graph("graph.png");
			}
			if ui.button("Reload").clicked() {
				self.reload(ctx);
			}
		});
		self.update_time = start.elapsed().as_micros();
		self.test_controller.tick(dt, &self.gui_settings.simulation_settings);
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		ggez::graphics::clear(ctx, Color::BLACK);

		// Measure time to draw:
		let start = std::time::Instant::now();
		// Batch draw the units:
		let mut batch = ggez::graphics::MeshBuilder::new();

		if self.gui_settings.simulation_settings.challenge_type != Challenge::ComplexQuery {
			let entities = self.test_controller.brain.get_entities(self.gui_settings.view_universe);

			for entity in &entities {
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
			if self.gui_settings.simulation_settings.challenge_type == Challenge::Slideshow {
				if self.frames % 100 == 0 && entities.len() > 0 {
					self.entity_image_index = (self.entity_image_index + 1) % entities.len();
					let image = self
						.test_controller
						.brain
						.get_image(entities[self.entity_image_index].entity_id);
					self.image = Some(ggez::graphics::Image::from_rgba8(ctx, 4000, 4000, &**image).unwrap());
				}
				if let Some(image) = &self.image {
					ggez::graphics::draw(ctx, image, (Vec2::new(0., 0.),))?;
				}
			}
			if self.gui_settings.simulation_settings.challenge_type == Challenge::ImageEditing {
				for entity in &entities {
					let image = self.test_controller.brain.get_image(entity.entity_id);
					let mut params = DrawParam::new();
					params =
						params.scale(Vec2::new(self.gui_settings.image_scale, self.gui_settings.image_scale));
					params = params
						.dest(Vec2::new(self.gui_settings.image_offset, self.gui_settings.image_offset));
					params = params.dest(Vec2::new(entity.position.x, entity.position.y));

					self.image = Some(ggez::graphics::Image::from_rgba8(ctx, 117, 117, &**image).unwrap());
					ggez::graphics::draw(ctx, self.image.as_ref().unwrap(), params)?;
				}
			}
		}

		let end = std::time::Instant::now();
		self.draw_time = (end - start).as_micros();

		ggez::graphics::draw(ctx, &self.egui_backend, ([0.0, 0.0],))?;
		ggez::graphics::present(ctx).unwrap();
		self.frames += 1;
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

	fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods, _repeat: bool) {
		self.egui_backend.input.key_down_event(keycode, keymods)
	}

	fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {
		self.egui_backend.input.text_input_event(_character)
	}
}

pub fn main() -> GameResult {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		path
	} else {
		path::PathBuf::from("./resources")
	};

	let mut cb = ggez::ContextBuilder::new("ECS Benchmark", "ggez").add_resource_path(resource_dir);

	cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
	let window_size_multiplier = 1.3;
	cb = cb.window_mode(
		ggez::conf::WindowMode::default()
			.dimensions(MAP_SIZE * window_size_multiplier, MAP_SIZE * window_size_multiplier)
			.resizable(true),
	);

	let (mut ctx, event_loop) = cb.build()?;
	let mut state = MainState::new(&mut ctx);
	ggez::event::run(ctx, event_loop, state)
}
