use super::*;

impl Generator for UltraHydra {
    fn generate(&mut self) -> MayFail {
        // Ultra Cleave: bonus damage to the attack target of the empowered attack
        self.damage_type(Physical)
            .trigger(Trigger::Charged)
            .min(Event("Ultra Hydra", AttackTarget))?
            .end()
    }
}
