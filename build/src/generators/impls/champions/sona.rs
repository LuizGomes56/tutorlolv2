use super::*;

impl Generator for Sona {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(3, Void) /* Innate - Power Chord */])
            // 30 - 405 (based on level) (+ 30% AP)
            .ability(
                Key::Q,
                [
                    (0, Void), /* Bonus Magic Damage */
                    (1, _1),   /* Magic Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(R(Void)),
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
                Attack,
            ])?
            .combo([Ability(R(Void)), Ability(Q(Void)), Attack, Attack])?
            .combo([Attack, Ability(Q(Void)), Attack])?
            .combo([Ability(R(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(Q(Void)), Attack])?
            .end()
    }
}
