pub struct GuiSettings {
    pub meet_distance: f32,
    pub view_universe: usize,
    pub universe_count: usize,
    pub entity_count: usize,
    pub brain_type: BrainType,
    pub challenge_type: ChallengeType,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BrainType{
    LegionSequential,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChallengeType{
    Rts,
}

