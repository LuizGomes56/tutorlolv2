use super::*;

impl GeneratorV2 for Tryndamere {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", E(Void))?
            .damage_type(E(Void), Physical)
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
