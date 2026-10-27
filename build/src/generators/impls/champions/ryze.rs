use super::*;

impl Generator for Ryze {
    fn generate(&mut self) -> MayFail {
        let r = &self.formula(Key::R, 0)?.to_string();

        self.ability(Key::Q, [(1, Void) /* Magic Damage */])
            .modify(Q(Void), |dmg| f![(dmg) * r])?
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(E(Void)), Ability(E(Void)), Ability(Q(Void))])?
            .combo([Ability(E(Void)), Ability(W(Void)), Attack, Ability(Q(Void))])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Attack,
            ])?
            .combo([Ability(Q(Void)), Ability(E(Void)), Ability(Q(Void))])?
            .end()
    }
}
