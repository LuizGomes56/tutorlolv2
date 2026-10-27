use super::*;

impl Generator for AurelionSol {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, Min), /* Magic Damage per Tick */
                (6, Max), /* Total Maximum Magic Damage */
                (1, _1),  /* Magic Damage per Second */
                (0, _2),  /* Bonus Magic Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, Min), /* Magic Damage per Tick */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability_nth(1, Key::R, [(0, Void) /* Magic Damage */])
        .ability(
            Key::R,
            [
                (1, _1), /* Magic Damage */
                (0, _2), /* Empowered Magic Damage */
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
