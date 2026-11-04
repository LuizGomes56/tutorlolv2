use super::*;

impl Generator for Nilah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Maximum Physical Damage", Max),
                ("Minimum Physical Damage", Min),
            ],
        )
        .ability(Key::E, [("Physical Damage", Void)])
        .ability(
            Key::R,
            [
                ("Burst Physical Damage", _2),
                ("Maximum Total Physical Damage", _1),
                ("Physical Damage per Tick", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
            ],
            &[Ability(E(Void)), Ability(R(Max))],
            &[Ability(E(Void)), Ability(R(Max)), Ability(E(Void))],
            &[
                Ability(E(Void)),
                Ability(E(Void)),
                Ability(R(Max)),
                Ability(Q(Max)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Max)),
                Attack,
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Max)),
                Ability(R(Max)),
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Attack,
                Attack,
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack, Attack],
        ])?
        .end()
    }
}
