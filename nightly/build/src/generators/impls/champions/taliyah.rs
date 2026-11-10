use super::*;

impl Generator for Taliyah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
                ("Empowered Damage", _1),
                ("Reduced Damage", _2),
            ],
        )
        .ability(
            Key::E,
            [
                ("Magic Damage", Void),
                ("Detonation Magic Damage", _1),
                ("Total Maximum Detonation Damage", _2),
            ],
        )
        .end()
    }
}
