use super::*;

impl Generator for Tiamat {
    fn generate(&mut self) -> MayFail {
        self.min(Active).damage_type(Physical).end()
    }
}
