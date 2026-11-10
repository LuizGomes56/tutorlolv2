use super::*;

impl Generator for MonkeyKing {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Bonus Physical Damage", Void)])
            .ability(Key::W, [("Clone Outgoing Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Maximum Total Physical Damage", _1Max),
                    ("Physical Damage Per Tick", Min),
                    ("Total Physical Damage", Max),
                ],
            )
            .damage_types([W(Void), R(Min), R(Max), R(_1Max)], Physical)?
            .modify(W(Void), |ratio| f![ratio * AttackDamage])?
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
                &[Ability(Q(Void)), Ability(W(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
