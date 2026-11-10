use super::*;

impl Generator for Syndra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::W,
                [
                    ("Bonus True Damage", Void),
                    ("Magic Damage", Min),
                    ("Total Mixed Damage", Max),
                ],
            );

        let w_bonus = self.damage_of(W(Void))?.clone();

        self.modify(W(Max), |dmg: &str| f![((dmg) * MagicMultiplier) + w_bonus])?
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(
                Key::R,
                [
                    ("Magic Damage per Sphere", Void),
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(E(Void))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
            ])?
            .end()
    }
}
