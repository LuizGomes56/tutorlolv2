use super::*;

impl Generator for RapidFirecannon {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
