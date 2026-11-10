use super::*;

impl Generator for Sona {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate - Power Chord", Void)])
            // 30 - 405 (based on level) (+ 30% AP)
            .ability(
                Key::Q,
                [
                    ("Bonus Magic Damage", Void),
                    ("Magic Damage", _1),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(R(Void)), Ability(Q(Void)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack],
                &[Ability(R(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
