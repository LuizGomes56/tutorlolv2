use super::*;

impl Generator for RekSai {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                ("Bonus Physical Damage", Min),
                ("Total Bonus Physical Damage", Max),
            ],
        )
        .ability(Key::Q, [("Magic Damage", _1)])
        .comment(Q(_1), "Prey Seeker while burrowed")?
        .ability(Key::W, [("Magic Damage", Void)])
        .ability_nth(
            1,
            Key::E,
            [
                ("Physical Damage", Void),
                ("True Damage", _1),
            ],
        )
        .damage_type(E(_1), True)?
        .ability(Key::R, [("Physical Damage", Void)])
        .combos([
            &[
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ],
            &[Attack, Ability(Q(Max)), Attack, Attack, Ability(E(Void))],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
