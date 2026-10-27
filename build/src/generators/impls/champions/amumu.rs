use super::*;

impl Generator for Amumu {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage Per Tick */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(R(Void)),
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(W(Void)), Ability(R(Void))])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(W(Void)), Attack])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([Ability(W(Void)), Ability(E(Void)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ])?
            .combo([
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Attack,
            ])?
            .end()
    }
}
