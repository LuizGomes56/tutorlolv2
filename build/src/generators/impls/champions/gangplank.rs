use super::*;

impl Generator for Gangplank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| f![50 + 200 / 17 * (Level - 1) + BonusAd])?
            .comment(P(Void), "Trial by Fire true damage over 2.5 seconds")?
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::E, [("Bonus Champion Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage Per Wave", Min),
                    ("Total Magic Damage", Max),
                    ("Total Magic Damage with  Fire at Will", _1),
                    ("True Damage with  Death's Daughter", _2),
                ],
            )
            .damage_type(R(_2), True)?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(E(Void))],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
