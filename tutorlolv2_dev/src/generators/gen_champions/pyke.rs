use super::*;

impl Generator for Pyke {
    fn generate(&mut self) -> MayFail {
        /* 250 – 550 (based on level) */
        /* (+ 80% bonus AD) */
        /* (+ 1.5 per 1 Lethality) */
        let r = [
            Scaling::based_on_level_raw(
                Level,
                [
                    LevelArm::new(..7, 250),
                    LevelArm::flat(7, 290),
                    LevelArm::flat(8, 330),
                    LevelArm::flat(9, 370),
                    LevelArm::flat(10, 400),
                    LevelArm::flat(11, 430),
                    LevelArm::flat(12, 450),
                    LevelArm::flat(13, 470),
                    LevelArm::flat(14, 490),
                    LevelArm::flat(15, 510),
                    LevelArm::flat(16, 530),
                    LevelArm::flat(17, 540),
                    LevelArm::new(18.., 550),
                ],
            ),
            Scaling::Simple {
                value: 0.8,
                ctx_var: BonusAd,
            },
            Scaling::Simple {
                value: 1.5,
                ctx_var: ArmorPenetrationFlat,
            },
        ]
        .into_iter()
        .map(|scaling| scaling.render(Level))
        .collect::<MayFail<Vec<_>>>()?
        .join(" + ");

        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void)])
            .modify(R(Void), |_| r.parenthesize().div(2))?
            .clone_to(R(Void), R(_1), r)?
            .damage_type(R(Void), Physical)?
            .damage_type(R(_1), True)?
            .end()
    }
}
