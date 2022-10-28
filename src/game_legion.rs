use ggez::mint::Point2;
use crate::GameImplementation;


pub struct GameLegion{

}
impl GameImplementation for GameLegion{
    fn update() {
    }
    fn get_unit_positions() -> Vec<Point2<f32>> {
        let mut positions = Vec::new();
        for _ in 0..10{
            positions.push(Point2::new(rand::random::<f32>() * 800.0, rand::random::<f32>() * 600.0));
        }
        positions
    }
    fn get_projectile_positions() -> Vec<Point2<f32>> {
        let mut positions = Vec::new();
        for _ in 0..10{
            positions.push(Point2::new(rand::random::<f32>() * 800.0, rand::random::<f32>() * 600.0));
        }
        positions
    }
}