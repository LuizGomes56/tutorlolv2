use super::*;

impl Generator for WarmogsArmor {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
