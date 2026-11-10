use super::*;

impl Generator for Shyvana {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Area Physical Damage", Void),
                ("True Damage", _1),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(
            Key::E,
            [
                ("Increased/Explosion Magic Damage", Max),
                ("Magic Damage", Void),
                ("Subsequent Explosion Damage", Min),
            ],
        )
        .ability(Key::R, [("Magic Damage", Void)])
        .damage_type(Q(Void), Physical)?
        .damage_type(Q(_1), True)?
        .combos([
            &[
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Void)),
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
