use super::*;

impl Generator for Zaahen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Bonus Physical Damage", Min),
                ("Physical Damage per Hit", Void),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::W,
            [
                ("Initial Physical Damage", Min),
                ("Subsequent Physical Damage", Void),
                ("Total Physical Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Bonus Magic Damage", Void),
                ("Increased Physical Damage", Max),
                ("Physical Damage", Min),
            ],
        )
        .damage_types([E(Min), E(Max)], Physical)?
        .damage_type(E(Void), Magic)?
        .ability(Key::R, [("Physical Damage", Void)])
        .end()
    }
}
