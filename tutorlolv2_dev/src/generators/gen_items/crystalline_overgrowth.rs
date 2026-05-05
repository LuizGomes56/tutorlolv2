use super::*;

impl Generator for CrystallineOvergrowth {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(True).end()
    }
}
