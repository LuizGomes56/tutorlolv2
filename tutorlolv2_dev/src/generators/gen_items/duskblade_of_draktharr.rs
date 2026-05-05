use super::*;

impl Generator for DuskbladeOfDraktharr {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
