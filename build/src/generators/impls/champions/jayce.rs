use super::*;

impl Generator for Jayce {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (1, Min), /* Physical Damage */
                (0, Max), /* Increased Damage */
            ],
        )
        .comment(Q(Min), "Shock Blast")?
        .comment(Q(Max), "Shock Blast through Acceleration Gate")?
        .ability_nth(1, Key::Q, [(0, _1) /* Physical Damage */])
        .comment(Q(_1), "To the Skies!")?
        .ability_nth(
            1,
            Key::W,
            [
                (0, Min), /* Magic Damage Per Tick */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(
            Key::W,
            [
                (0, _1Min), /* Physical Damage */
                (1, _1Max), /* Total Physical Damage */
            ],
        )
        .ability_nth(1, Key::E, [(1, Void) /* Magic Damage */])
        .damage_types([Q(Min), Q(Max), Q(_1), W(_1Min), W(_1Max)], Physical)?
        .combo([Ability(W(Max)), Attack, Attack, Ability(E(Void)), Attack])?
        .combo([Ability(Q(Max)), Attack, Ability(E(Void))])?
        .combo([
            Ability(Q(Min)),
            Ability(E(Void)),
            Attack,
            Ability(W(Max)),
            Ability(Q(Min)),
            Ability(W(Max)),
            Attack,
            Ability(E(Void)),
        ])?
        .combo([
            Ability(Q(Min)),
            Ability(E(Void)),
            Ability(Q(Min)),
            Attack,
            Ability(E(Void)),
        ])?
        .combo([
            Attack,
            Ability(Q(Min)),
            Ability(Q(Min)),
            Attack,
            Ability(E(Void)),
        ])?
        .end()
    }
}
