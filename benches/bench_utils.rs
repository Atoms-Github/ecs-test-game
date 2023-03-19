use std::fmt::format;
use std::{env, path};

use criterion::measurement::WallTime;
use criterion::{AxisScale, BenchmarkGroup, BenchmarkId, Criterion, PlotConfiguration};
use ecs_test_game::simulation_settings::{BrainType, SimSettings};
use ecs_test_game::test_controller::TestController;
use ecs_test_game::MAP_SIZE;
use ggez::Context;
use rand::Rng;
pub fn gen_test_ctx() -> Context {
	let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		path.push("resources");
		path
	} else {
		path::PathBuf::from("./resources")
	};
	let mut cb = ggez::ContextBuilder::new("ECS Benchmark", "ggez").add_resource_path(resource_dir);

	cb = cb.window_setup(ggez::conf::WindowSetup::default().title("Ecs Performance Benchmark"));
	cb = cb.window_mode(ggez::conf::WindowMode::default().visible(false));

	let (ctx, event_loop) = cb.build().unwrap();
	ctx
}

pub fn benchmark(group: &mut BenchmarkGroup<WallTime>, settings: &SimSettings, frame_count: usize) {
	let mut ctx = gen_test_ctx();
	let mut controller = TestController::gen_test_controller(&mut ctx, settings);
	let mut entity_count = settings.entity_count;

	// let name = format!("{}, {}", settings.brain_type.to_string(), rand::thread_rng().gen_range(0..5000));
	group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
	group.bench_with_input(
		BenchmarkId::new(settings.brain_type.to_string(), entity_count),
		&mut entity_count,
		|b, _| {
			b.iter(|| {
				for _ in 0..frame_count {
					controller.tick(0.016, settings);
				}
			})
		},
	);
}
