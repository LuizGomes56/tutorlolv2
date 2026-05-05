use super::*;

impl Generator for KrakenSlayer {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
