use super::*;

impl Generator for DrMundo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Max),
                ("Minimum Damage", Min),
            ],
        )
        .ability(
            Key::W,
            [
                ("Magic Damage", Min),
                ("Magic Damage per Tick", _1),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Maximum Bonus Physical Damage", Max),
                ("Minimum Bonus Physical Damage", Min),
            ],
        )
        .damage_types([E(Min), E(Max)], Physical)?
        .end()
    }
}
