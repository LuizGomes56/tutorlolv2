use super::*;

impl Generator for Tryndamere {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [(0, Void) /* Physical Damage */])
            .damage_type(E(Void), Physical)?
            .combo([Ability(E(Void)), Attack])?
            .combo([Attack, Attack, Ability(E(Void)), Attack])?
            .combo([
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Attack, Ability(E(Void))])?
            .combo([Ability(E(Void)), Attack, Attack, Attack, Attack])?
            .combo([Attack, Attack, Attack, Ability(E(Void)), Attack])?
            .end()
    }
}
