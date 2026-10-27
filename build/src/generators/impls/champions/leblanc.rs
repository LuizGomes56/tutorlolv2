use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Min), /* Magic Damage */
                (1, Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Magic Damage */])
        .ability(
            Key::E,
            [
                (1, Min), /* Magic Damage */
                (2, Max), /* Total Damage */
                (0, _1),  /* Fracture Magic Damage */
            ],
        )
        .ability(
            Key::R,
            [
                (0, _1), /* Application Magic Damage */
                (3, _2), /* Mark Magic Damage */
                (2, _3), /* Magic Damage */
                (4, _4), /* Orb Magic Damage */
                (1, _5), /* Fracture Magic Damage */
                (5, _6), /* Total Magic Damage */
            ],
        )
        .combos([
            &[
                Ability(R(_1)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(E(Max)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Ability(R(_1)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(Q(Max)),
                Ability(E(Max)),
            ],
            &[
                Ability(E(Max)),
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Max)),
                Ability(W(Void)),
                Attack,
                Ability(R(_1)),
                Ability(Q(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(Q(Max)),
                Ability(E(Max)),
                Ability(W(Void)),
            ],
            &[
                Ability(E(Max)),
                Ability(Q(Max)),
                Ability(R(_1)),
                Ability(W(Void)),
                Ability(W(Void)),
            ],
            &[
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(E(Max)),
                Ability(W(Void)),
                Ability(R(_1)),
                Ability(Q(Max)),
            ],
            &[
                Ability(R(_1)),
                Ability(E(Max)),
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(W(Void)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Ability(W(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ],
            &[
                Ability(W(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Ability(R(_1)),
                Ability(Q(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(E(Max)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Ability(W(Void)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(R(_1)),
                Ability(E(Max)),
                Ability(W(Void)),
            ],
            &[Ability(W(Void)), Attack, Ability(Q(Max)), Ability(W(Void))],
            &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(W(Void))],
        ])?
        .end()
    }
}
