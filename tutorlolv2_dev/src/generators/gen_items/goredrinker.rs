use super::*;

impl Generator for Goredrinker {
    fn generate(&mut self) -> MayFail {
        self.min(Active).damage_type(Physical).end()
    }
}
