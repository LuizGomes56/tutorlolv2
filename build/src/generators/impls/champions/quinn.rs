use super::*;

impl Generator for Quinn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .combo([
                Ability(R(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([Ability(E(Void)), Attack, Ability(Q(Void))])?
            .combo([
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(E(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
            ])?
            .end()
    }
}
