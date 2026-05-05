use super::*;

impl Generator for LocketOfTheIronSolari {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
