use ggez::graphics::Color;
use glam::Vec2;
use rand::Rng;

pub fn time_it<F>(to_time: F) -> u128
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    to_time();
    let end = std::time::Instant::now();
    (end - start).as_micros()
}

pub trait GenRandom {
    fn gen_random() -> Self;
}
impl GenRandom for Vec2 {
    fn gen_random() -> Self {
        let mut rand = rand::thread_rng();
        Vec2::new(rand.gen_range(0.0..1.0), rand.gen_range(0.0..1.0))
    }
}
pub trait FromTeam {
    fn from_team(team: usize) -> Self;
}
pub fn color_from_team(team: usize) -> f32 {
    match team {
        0 => 0.0,
        1 => 0.5,
        2 => 1.0,
        _ => 0.0,
    }
}

impl GenRandom for Color {
    fn gen_random() -> Self {
        let mut rng = rand::thread_rng();
        Color::new(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            1.0,
        )
    }
}
macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat{..} = $target {
            // #1
            ..
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
        }
    }};
}
pub(crate) use cast;