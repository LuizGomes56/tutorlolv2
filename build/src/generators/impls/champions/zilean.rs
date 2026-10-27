use super::*;

impl Generator for Zilean {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .combo([Ability(Q(Void)), Ability(Q(Void))])?
            .combo([Attack, Ability(Q(Void)), Attack, Ability(Q(Void)), Attack])?
            .end()
    }
}
