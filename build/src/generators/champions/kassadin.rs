use super::*;

impl Generator for Kassadin {
    fn generate(&mut self) -> MayFail {
        let [reduction] = self
            .description(Key::P, "Innate")?
            .capture(regex!(r"takes (\d+)% reduced\s+magic damage"))?;

        self.constant("VOID_STONE_MAGIC_REDUCTION", reduction / 100.0)?
            .take("Magic Damage", Q(Void))?
            .take("Increased Bonus Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Bonus Damage Per Stack", R(_1Min))?
            .take("Magic Damage", R(Min))?
            .take("Maximum Bonus Damage", R(_1Max))?
            .take("Maximum Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Attack, Ability(W(Void))],
                &[
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Min)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(W(Void)), Ability(Q(Void))],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Void))],
                &[Ability(R(Max)), Ability(E(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Max))],
            ])?
            .end()
    }
}
