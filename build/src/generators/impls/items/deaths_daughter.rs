use super::*;

impl Generator for DeathsDaughter {
    fn generate(&mut self) -> MayFail {
        // Death's Daughter: cannonball of Gangplank's Cannon Barrage, whose damage
        // increases with the rank of his ultimate
        let damage = self
            .formula(Passive)?
            .replace(Level.as_var(), RLevel.as_var());
        self.damage_type(True).set_min(damage).end()
    }
}
