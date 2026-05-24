use super::*;

impl Generator for KrakenSlayer {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
