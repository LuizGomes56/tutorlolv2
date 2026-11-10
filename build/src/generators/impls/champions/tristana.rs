use super::*;

impl Generator for Tristana {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::W, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    // ("Bonus Damage Per Stack", _1),
                    // ("Full Stack Bonus Damage", _2),
                    ("Full Stack Physical Damage", Max),
                    ("Magic Damage", Void),
                    ("Minimum Physical Damage", Min),
                ],
            )
            .comment(E(Void), "Passive explosion damage on kill")?
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[Attack, Ability(E(Void)), Attack, Attack, Attack, Attack],
            ])?
            .end()
    }
}
