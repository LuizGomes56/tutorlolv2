use super::*;

impl Generator for Predator {
    fn generate(&mut self) -> MayFail {
        for attack_type in [Melee, Ranged] {
            let damage = self.compose_for([1, 2], attack_type)?;
            self.assign(attack_type, Min, &damage);
        }

        self.damage_type(Adaptive).end()
    }
}
