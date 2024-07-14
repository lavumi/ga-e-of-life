use crate::configs;

pub struct StageTick {
    pub current_spent: f32,
    pub stage_tick: f32,
}

impl Default for StageTick {
    fn default() -> Self {
        StageTick {
            current_spent: 0.0,
            stage_tick: configs::LIFE_TICK,
        }
    }
}
