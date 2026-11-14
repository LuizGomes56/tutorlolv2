use super::*;

impl Generator for Jax {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Additional Magic Damage", W(Void))?
            .take("Minimum Magic Damage", E(Min))?
            .take("Maximum Magic Damage", E(Max))?
            .take("Magic Damage", R(Void))?
            .map_change(R(Void), "Active base damage")?
            .take("Additional Magic Damage", R(_1))?
            .map_change(R(_1), "Passive base damage")?
            .damage_type(Q(Void), Physical)
            .combos([
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Min)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void))],
                &[
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
