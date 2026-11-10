use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(
                Key::Q,
                [
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                ],
            )
            .ability(
                Key::W,
                [
                    ("Magic Damage Per Bolt", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Min)])
            .clone_with(E(Min), E(Max), |dmg| f![((dmg) * MagicMultiplier) + dmg])?
            .combos([
                &[Ability(E(Max)), Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Ability(Q(Min)), Attack],
                &[Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Attack, Ability(Q(Min)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
