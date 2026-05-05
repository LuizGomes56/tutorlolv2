use super::*;

impl Generator for GuardiansAmulet {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
