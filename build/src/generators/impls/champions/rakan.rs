use super::*;

impl Generator for Rakan {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .combo([Ability(R(Void)), Ability(W(Void)), Ability(Q(Void))])?
            .combo([Ability(R(Void)), Ability(W(Void)), Attack, Ability(Q(Void))])?
            .combo([Ability(W(Void)), Attack, Ability(Q(Void))])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ])?
            .combo([Ability(R(Void)), Ability(W(Void)), Ability(Q(Void)), Attack])?
            .combo([Ability(Q(Void)), Ability(W(Void)), Attack])?
            .combo([Ability(Q(Void)), Attack, Ability(W(Void)), Attack])?
            .end()
    }
}
