use super::*;

impl Generator for Goredrinker {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
