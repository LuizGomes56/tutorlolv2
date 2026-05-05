use super::*;

impl Generator for IcebornGauntlet {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
