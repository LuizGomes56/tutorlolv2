use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage per Second", Min),
                ("Magic Damage per Tick", Max),
                ("Minimum Magic Damage", Void),
            ],
        )
        .ability(Key::E, [("Magic Damage", Void)])
        .combos([
            &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
            &[Ability(Q(Void)), Ability(E(Void)), Attack, Attack],
            &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack],
            &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack, Attack],
            &[Ability(E(Void)), Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
