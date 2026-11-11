use super::*;

impl Generator for Kassadin {
    fn generate(&mut self) -> MayFail {
        let [reduction] = self
            .description(Key::P, "Innate")?
            .capture(regex!(r"takes (\d+)% reduced\s+magic damage"))?;

        self.constant("VOID_STONE_MAGIC_REDUCTION", reduction / 100.0)?
            .ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Increased Bonus Magic Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Bonus Damage Per Stack", _1Min),
                    ("Magic Damage", Min),
                    ("Maximum Bonus Damage", _1Max),
                    ("Maximum Magic Damage", Max),
                ],
            )
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
