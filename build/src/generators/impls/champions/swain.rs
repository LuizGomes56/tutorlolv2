use super::*;

impl Generator for Swain {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Void), /* Bonus Damage Per Bolt */
                (1, Min),  /* Magic Damage */
                (2, Max),  /* Total Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability_nth(1, Key::R, [(1, Void) /* Magic Damage per Tick */])
        .ability(Key::R, [(0, _1) /* Magic Damage */])
        .combo([
            Ability(Q(Void)),
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(R(Void)),
            Ability(E(Void)),
            Attack,
            Ability(R(_1)),
        ])?
        .combo([
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(R(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Attack,
            Ability(R(_1)),
        ])?
        .combo([
            Ability(R(Void)),
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(E(Void)),
            Attack,
            Ability(Q(Void)),
            Attack,
            Ability(R(_1)),
        ])?
        .combo([
            Ability(W(Void)),
            Ability(E(Void)),
            Ability(R(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Attack,
            Attack,
            Ability(R(_1)),
            Attack,
            Ability(Q(Void)),
        ])?
        .combo([
            Ability(Q(Void)),
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(E(Void)),
            Attack,
            Attack,
            Attack,
            Ability(Q(Void)),
        ])?
        .combo([
            Ability(E(Void)),
            Ability(W(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Attack,
        ])?
        .combo([
            Ability(W(Void)),
            Ability(E(Void)),
            Ability(E(Void)),
            Ability(Q(Void)),
            Attack,
        ])?
        .end()
    }
}
