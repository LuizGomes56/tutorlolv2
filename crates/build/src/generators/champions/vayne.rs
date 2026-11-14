use super::*;

impl Generator for Vayne {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Minimum Bonus Damage", W(Min))?
            .take("Bonus True Damage", W(Max))?
            .take("Bonus Physical Damage", E(Void))?
            .take("Physical Damage", E(Min))?
            .take("Total Physical Damage", E(Max))?
            .damage_type(Q(Void), Physical)
            .damage_type(E(Void), Physical)
            .damage_type(E(Min), Physical)
            .damage_type(E(Max), Physical)
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Void)), Attack, Attack],
            ])?
            .end()
    }
}
