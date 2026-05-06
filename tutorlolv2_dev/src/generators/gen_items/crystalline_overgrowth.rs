use super::*;

impl Generator for CrystallineOvergrowth {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
