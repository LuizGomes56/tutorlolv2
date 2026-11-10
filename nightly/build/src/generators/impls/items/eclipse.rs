use super::*;

impl Generator for Eclipse {
    fn generate(&mut self) -> MayFail {
        // Ever Rising Moon: bonus physical damage after applying 2 stacks. The
        // other scaling is its shield
        self.damage_type(Physical).min_with(Passive, [1])?.end()
    }
}
