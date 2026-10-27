use super::*;

impl Generator for Tryndamere {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::E, [(0, Void) /* Physical Damage */])
            .damage_type(E(Void), Physical)?
            .combos([
                &[Ability(E(Void)), Attack],
                &[Attack, Attack, Ability(E(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void))],
                &[Ability(E(Void)), Attack, Attack, Attack, Attack],
                &[Attack, Attack, Attack, Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
