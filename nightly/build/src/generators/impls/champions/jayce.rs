use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Physical Damage", Min),
                ("Increased Damage", Max),
            ],
        )
        .comment(Q(Min), "Shock Blast")?
        .comment(Q(Max), "Shock Blast through Acceleration Gate")?
        .ability_nth(1, Key::Q, [("Physical Damage", _1)])
        .comment(Q(_1), "To the Skies!")?
        .ability_nth(
            1,
            Key::W,
            [
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::W,
            [
                ("Physical Damage", _1Min),
                ("Total Physical Damage", _1Max),
            ],
        )
        .ability_nth(1, Key::E, [("Magic Damage", Void)])
        .damage_types([Q(Min), Q(Max), Q(_1), W(_1Min), W(_1Max)], Physical)?
        .combos([
            &[Ability(W(Max)), Attack, Attack, Ability(E(Void)), Attack],
            &[Ability(Q(Max)), Attack, Ability(E(Void))],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
                Ability(W(Max)),
                Ability(Q(Min)),
                Ability(W(Max)),
                Attack,
                Ability(E(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
            ],
        ])?
        .end()
    }
}
