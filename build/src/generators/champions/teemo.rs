use super::*;

impl Generator for Teemo {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage On-Hit", E(Void))?
            .take("Magic Damage per Tick", E(Min))?
            .take("Total Poison Damage", E(Max))?
            .take("Magic Damage per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(Q(Void))],
                &[Ability(R(Max)), Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Attack, Attack, Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
