use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 1", Void)])
            .ability(
                Key::Q,
                [
                    ("Bonus Damage Per Second", _1),
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .ability(Key::W, [("Magic Damage", Void)])
            .damage_type(P(Void), Magic)?
            .combos([
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(W(Void))],
                &[Ability(Q(Max)), Ability(W(Void))],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Min)), Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
