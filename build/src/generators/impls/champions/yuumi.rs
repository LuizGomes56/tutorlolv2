use super::*;

impl Generator for Yuumi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Increased Damage", Max),
                ("Magic Damage", Min),
                ("Maximum Bonus Magic Damage On-Hit", _1Max),
                ("Minimum Bonus Magic Damage On-Hit", _1Min),
            ],
        )
        .ability(
            Key::R,
            [
                ("Magic Damage per Hit", Void),
                ("Reduced Damage per Hit", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .combos([
            &[Ability(R(Void)), Ability(Q(Max))],
            &[Attack, Attack, Ability(Q(Max))],
        ])?
        .end()
    }
}
