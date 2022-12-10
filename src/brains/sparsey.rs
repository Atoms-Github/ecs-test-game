use crate::brains::com::ExportEntity;
use crate::brains::SystemType::Velocity;
use crate::brains::{Brain, SystemType};
use crate::ui::ui_settings::GuiSettings;
use crate::Point;
use ggez::winit::dpi::Position;
use legion::systems::SystemFn;
use legion::{Resources, Schedule, World};
pub struct Sparsey {}

impl Brain for Sparsey {
    fn add_entity_unit(
        &mut self,
        position: Point,
        velocity: Point,
        team: usize,
        universe_id: usize,
    ) {
        todo!()
    }

    fn add_entity(&mut self, position: Point, velocity: Option<Point>, blue: f32) {
        todo!()
    }

    fn get_entities(&mut self, universe_id: usize) -> Vec<ExportEntity> {
        todo!()
    }

    fn init(&mut self, systems: &Vec<SystemType>) {
        todo!()
    }

    fn tick_systems(&mut self, delta: f32, settings: &GuiSettings, systems: &Vec<SystemType>) {
        todo!()
    }

    fn tick_system(&mut self, system: &SystemType, delta: f32, settings: &GuiSettings) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }
}
fn update_velocities(mut velocities: CompMut<Velocity>, frozen: Comp<Frozen>) {
    (&mut velocities).include(&frozen).for_each(|velocity| {
        velocity.0 = 0.0;
    });
}

fn update_positions(mut positions: CompMut<Position>, velocities: Comp<Velocity>) {
    (&mut positions, &velocities).for_each(|(position, velocity)| {
        position.0 += velocity.0;
    });
}

fn main() {
    let mut schedule = Schedule::builder()
        .add_system(update_velocities)
        .add_system(update_positions)
        .build();

    let mut world = World::default();
    schedule.set_up(&mut world);

    world.create((Position(0.0), Velocity(1.0)));
    world.create((Position(0.0), Velocity(2.0)));
    world.create((Position(0.0), Velocity(3.0), Frozen));

    let mut resources = Resources::default();

    for _ in 0..5 {
        schedule.run(&mut world, &mut resources);
    }
}
