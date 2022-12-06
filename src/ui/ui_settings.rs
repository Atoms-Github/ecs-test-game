#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GuiSettings {
    pub meet_distance: f32,
    pub view_universe: usize,
    pub universe_count: usize,
    pub entity_count: usize,
    pub blend_speed: f32,
    pub brain_type: BrainType,
    pub challenge_type: ChallengeType,
}
impl GuiSettings {
    pub fn new() -> GuiSettings {
        GuiSettings {
            meet_distance: 10.0,
            view_universe: 0,
            universe_count: 1,
            blend_speed: 10.0,
            entity_count: 100,
            brain_type: BrainType::LegionSequential,
            challenge_type: ChallengeType::GetNearest,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrainType{
    LegionSequential,
    LegionScheduled,
    SqlDuck,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChallengeType{
    Rts,
    GetNearest,
}

