use super::*;

impl Generator for ScoutsSlingshot {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
