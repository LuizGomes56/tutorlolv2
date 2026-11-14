use super::*;

impl Generator for Ekko {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Resonance", P(Void))?
            .map_change(P(Void), "Base damage")?
            .take("Initial Magic Damage", Q(_1Min))?
            .map_change(Q(_1Min), "Initial base damage")?
            .take("Total Magic Damage", Q(Max))?
            .take("Return Magic Damage", Q(_1Max))?
            .map_change(Q(_1Max), "Secondary base damage")?;

        let initial = self.damage(Q(_1Min))?;
        let secondary = self.damage(Q(_1Max))?;

        self.set(Q(Max), f![initial + secondary])
            .take("Bonus Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Max)), Ability(E(Void)), Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
