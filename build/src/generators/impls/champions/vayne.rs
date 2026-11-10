use super::*;

impl Generator for Vayne {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Bonus True Damage", Max),
                    ("Minimum Bonus Damage", Min),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Bonus Physical Damage", Void),
                    ("Physical Damage", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .damage_types([Q(Void), E(Void), E(Min), E(Max)], Physical)?
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
