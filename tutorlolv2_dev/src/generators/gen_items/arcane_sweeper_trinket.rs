use super::*;

impl Generator for ArcaneSweeperTrinket {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(True).end()
    }
}
