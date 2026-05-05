use super::*;

impl Generator for SlightlyMagicalBoots {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
