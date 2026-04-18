use super::*;

impl Generator for Amumu {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(0, 0, Void)])
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(W(Void)),
                Ability(E(Void)),
            ])?
            .progress(Stable)
            .end()
    }
}
