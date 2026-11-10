use super::*;

impl Generator for Seraphine {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Description 2", Void)])
            .ability(
                Key::Q,
                [
                    ("Magic Damage", Min),
                    ("Maximum Enhanced Damage", Max),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max))],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(R(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
