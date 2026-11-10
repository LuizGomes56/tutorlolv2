use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate [1]", Void)])
            .modify(P(Void), |_| {
                f![(0.05 + 0.04 / 17 * (Level - 1)) * EnemyMaxHealth]
            })?
            .ability_nth(
                1,
                Key::Q,
                [
                    ("Bonus Physical Damage per Hit", Min),
                    ("Total Bonus Physical Damage", Max),
                ],
            )
            .ability(Key::Q, [("Physical Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Physical Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .damage_type(W(Void), Magic)?
            .combos([
                &[Ability(R(Void)), Ability(W(Void))],
                &[Ability(E(Void)), Ability(R(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack, Attack],
            ])?
            .end()
    }
}
