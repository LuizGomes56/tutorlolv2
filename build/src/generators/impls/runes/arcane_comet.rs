use super::*;

impl Generator for ArcaneComet {
    fn generate(&mut self) -> MayFail {
        for attack_type in [Melee, Ranged] {
            let damage = self.compose_for([0, 1], attack_type)?;

            self.assign(attack_type, Min, &damage);
            self.assign(attack_type, Max, f![damage * 2]);
        }

        self.damage_type(Adaptive).end()
    }
}
