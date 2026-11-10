use super::*;

impl Generator for AurelionSol {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage per Tick", Min),
                ("Total Maximum Magic Damage", Max),
                ("Magic Damage per Second", _1),
                ("Bonus Magic Damage", _2),
            ],
        )
        .ability(
            Key::E,
            [
                ("Magic Damage per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability_nth(1, Key::R, [("Magic Damage", Void)])
        .ability(
            Key::R,
            [
                ("Magic Damage", _1),
                ("Empowered Magic Damage", _2),
            ],
        )
        .comment(R(_1), "The Skies Descend")?
        .comment(R(_2), "The Skies Descend empowered by Stardust")?
        .combos([
            &[Ability(E(Max)), Ability(R(Void)), Ability(Q(Max))],
            &[
                Ability(Q(Max)),
                Ability(E(Max)),
                Ability(R(Void)),
                Ability(Q(Max)),
            ],
            &[Ability(E(Max)), Ability(R(Void))],
            &[Ability(E(Max)), Ability(Q(Max))],
        ])?
        .end()
    }
}
