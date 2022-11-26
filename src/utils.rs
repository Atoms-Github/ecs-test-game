use ggez::graphics::Color;

pub fn time_it<F>(to_time: F) -> u128
where
    F: FnOnce(),
{
    let start = std::time::Instant::now();
    to_time();
    let end = std::time::Instant::now();
    (end - start).as_micros()
}

pub fn team_to_color(team: usize) -> Color {
     match team {
         0 => Color::new(1.0, 0.0, 0.0, 1.0),
         1 => Color::new(0.0, 1.0, 0.0, 1.0),
         2 => Color::new(0.0, 0.0, 1.0, 1.0),
         _ => Color::new(0.0, 0.0, 0.0, 1.0),
     }
}