use super::*;

impl Generator for Xayah {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                ("Physical Damage Per Hit", Min),
                ("Reduced Damage per Hit", _1Min),
                ("Total Physical Damage", Max),
                ("Total Reduced Damage", _1Max),
            ],
        )
        .ability(Key::E, [("Physical Damage Per Feather", Void)])
        .ability(Key::R, [("Physical Damage", Void)])
        .combos([
            &[Ability(R(Void)), Ability(E(Void)), Attack, Attack],
            &[Ability(Q(Max)), Attack, Ability(E(Void))],
            &[
                Ability(R(Void)),
                Ability(E(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
            &[Attack, Ability(Q(Max)), Attack, Ability(E(Void))],
            &[Ability(R(Void)), Ability(E(Void))],
        ])?
        .end()
    }
}
