use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Attack Speed per Subsequent Stack", Void)])
            .modify(Q(Void), |_| f![1.1 * AttackDamage])?
            .ability(Key::W, [("Physical Damage", Void)])
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Maximum Physical Damage", Max),
                    ("Maximum Secondary Damage", _1Max),
                    ("Minimum Physical Damage", Min),
                    ("Minimum Secondary Damage", _1Min),
                ],
            )
            .combos([
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(W(Void)), Attack, Attack, Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
