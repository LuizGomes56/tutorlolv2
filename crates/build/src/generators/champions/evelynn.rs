use super::*;

impl Generator for Evelynn {
    fn generate(&mut self) -> MayFail {
        // The recast's spikes, which deal the dart's damage except on Arena
        let spike = self.value_nth("Magic Damage", 1, Key::Q)?;
        let spike = self.map_changed(Key::Q, "Spikes base damage", spike)?;
        let spike = self.map_changed(Key::Q, "Spikes AP ratio", spike)?;

        self.take_nth("Magic Damage", 0, Q(Min))?
            .map_change(Q(Min), "Dart base damage")?
            .map_change(Q(Min), "Dart AP ratio")?
            .take("Total Magic Damage", Q(Max))?
            .take("Bonus Magic Damage", Q(_1))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .map_change(E(Void), "Target health ratio")?
            .take("Empowered Magic Damage", E(_1))?
            .map_change(
                E(_1),
                "{{ai|Empowered Whiplash|Evelynn}} target health ratio",
            )?
            .take("Magic Damage", R(Min))?
            .take("Empowered Damage", R(Max))?;

        let dart = self.damage(Q(Min))?;
        let bonus = self.damage(Q(_1))?;

        // The dart, 3 spikes and the 3 bonus hits of its mark
        self.set(Q(Max), f![dart + 3 * spike + 3 * bonus])
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(R(Max)),
                ],
                &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                ],
            ])?
            .end()
    }
}
