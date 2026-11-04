use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Magic Damage", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(Key::W, [("Magic Damage", Void)])
        .ability(
            Key::E,
            [
                ("Magic Damage", Min),
                ("Total Damage", Max),
                ("Fracture Magic Damage", _1),
            ],
        )
        .ability(
            Key::R,
            [
                ("Application Magic Damage", _1),
                ("Mark Magic Damage", _2),
                ("Magic Damage", _3),
                ("Orb Magic Damage", _4),
                ("Fracture Magic Damage", _5),
                ("Total Magic Damage", _6),
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
