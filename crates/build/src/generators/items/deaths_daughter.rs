use super::*;

impl Generator for DeathsDaughter {
    fn generate(&mut self) -> MayFail {
        // Death's Daughter: cannonball of Gangplank's Cannon Barrage, whose damage
        // increases with the rank of his ultimate
        let damage = self
            .formula(Named("Death's Daughter"))?
            .replace_var(Level, &Formula::Var(RLevel));

        self.damage_type(True).set_min(damage).end()
    }
}
