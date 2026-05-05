use super::*;

impl Generator for RabadonsDeathcap {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
