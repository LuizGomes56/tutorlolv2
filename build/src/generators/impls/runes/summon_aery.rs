use super::*;

impl Generator for SummonAery {
    fn generate(&mut self) -> MayFail {
        for attack_type in [Melee, Ranged] {
            let damage = self.compose_for([3, 4], attack_type)?;
            self.assign(attack_type, Min, &damage);
        }

        self.damage_type(Adaptive).end()
    }
}
