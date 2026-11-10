use super::*;

impl Generator for Locke {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Min)])
            .modify(P(Min), |_| {
                f![5 + 35 / 17 * (Level - 1) + 0.1 * AbilityPower]
            })?
            .comment(P(Min), "Silver Stake on-hit damage")?
            .clone_with(P(Min), P(Max), |p| f![2 * (p)])?
            .comment(
                P(Max),
                "Silver Stake on-hit damage against low health targets",
            )?
            .ability(
                Key::Q,
                [
                    ("Magic Damage per Nail", Min),
                    ("Maximum Total Magic Damage", Max),
                ],
            )
            .ability(
                Key::E,
                [
                    ("Dash Magic Damage", Min),
                    ("Total Magic Damage", Max),
                    ("Blink Magic Damage", _1),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
            .damage_type(R(Void), Magic)?
            .end()
    }
}
