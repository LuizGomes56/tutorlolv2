use super::*;

impl GeneratorV2 for Yone {
    fn generate(&mut self) -> MayFail {
        let mix = |dmg: Formula| f![((dmg) * MagicMultiplier) + ((dmg) * PhysicalMultiplier)];

        self.take("Critical Strike Damage", Q(Max))?
            .take("Physical Damage", Q(Min))?
            .take("Magic Damage", W(Void))?
            .edit(W(Void), mix)
            .take("Magic Damage", R(Void))?
            .edit(R(Void), mix)
            .damage_type(W(Void), Mixed)
            .damage_type(R(Void), Mixed)
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
