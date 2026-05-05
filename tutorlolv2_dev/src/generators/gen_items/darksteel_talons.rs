use super::*;

impl Generator for DarksteelTalons {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
