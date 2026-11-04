use super::*;

impl Generator for Teemo {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage On-Hit", Void),
                    ("Magic Damage per Tick", Min),
                    ("Total Poison Damage", Max),
                ],
            )
            .ability(
                Key::R,
                [
                    ("Magic Damage per Tick", Min),
                    ("Total Magic Damage", Max),
                ],
            )
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(Q(Void))],
                &[Ability(R(Max)), Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Attack, Attack, Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
