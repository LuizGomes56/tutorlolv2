use super::*;

impl Generator for Vi {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Maximum Physical Damage", Max),
                ("Minimum Physical Damage", Min),
            ],
        )
        .ability(Key::W, [("Bonus Physical Damage", Void)])
        .ability(Key::E, [("Physical Damage", Void)])
        .ability(Key::R, [("Physical Damage", Void)])
        .damage_types([Q(Min), Q(Max), E(Void)], Physical)?
        .combos([
            &[
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
            ],
            &[Ability(Q(Max)), Ability(R(Void))],
            &[
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Max)),
                Attack,
                Attack,
            ],
            &[
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(Q(Max)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[Ability(Q(Max)), Attack, Ability(E(Void)), Attack, Attack],
        ])?
        .end()
    }
}
