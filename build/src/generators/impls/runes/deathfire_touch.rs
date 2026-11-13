use super::*;

impl Generator for DeathfireTouch {
    fn generate(&mut self) -> MayFail {
        // Burn damage per tick, increased by 75% after lingering for 3 seconds
        for attack_type in [Melee, Ranged] {
            let damage = self.compose_for([0, 1], attack_type)?;

            self.assign(attack_type, Min, &damage);
            self.assign(attack_type, Max, f![1.75 * damage]);
        }

        self.damage_type(Adaptive).end()
    }
}
