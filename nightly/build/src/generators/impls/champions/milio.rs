use super::*;

impl Generator for Milio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .ability(Key::Q, [("Magic Damage", Void)])
            .combo([Ability(Q(Void)), Attack, Attack, Attack, Attack])?
            .end()
    }
}
