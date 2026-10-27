use strum::{EnumCount, VariantArray};

#[cfg(all(test, feature = "engine"))]
mod passive_regressions {
    use crate::{
        AbilityId::P,
        AbilityName::{_1, Void},
        ChampionId,
    };
    use tutorlolv2_types::Ctx;

    #[test]
    fn reparsed_passives_preserve_their_damage_components() {
        let ctx = Ctx {
            level: 18.0,
            w_level: 1.0,
            attack_damage: 200.0,
            ability_power: 100.0,
            bonus_armor: 100.0,
            enemy_max_health: 2000.0,
            stacks: 10.0,
            ..Default::default()
        };
        for (champion, ability, expected) in [
            (ChampionId::Bard, P(Void), 130.0),
            (ChampionId::Lucian, P(Void), 120.0),
            (ChampionId::Lucian, P(_1), 35.0),
            (ChampionId::Taric, P(Void), 108.0),
            (ChampionId::Urgot, P(Void), 320.0),
            (ChampionId::Brand, P(_1), 280.0),
            (ChampionId::Milio, P(Void), 50.0),
            (
                ChampionId::Samira,
                crate::AbilityId::P(crate::AbilityName::Min),
                40.0,
            ),
            (ChampionId::Malzahar, crate::AbilityId::W(Void), 96.5),
        ] {
            let actual = champion.eval(&ctx, ability);
            assert!(
                (actual - expected).abs() < 0.01,
                "{champion:?} {ability:?}: {actual} != {expected}"
            );
        }
        let vigilance = ChampionId::Lucian.index_of_ability(P(_1)).unwrap();
        assert_eq!(
            ChampionId::Lucian.metadata()[vigilance].damage_type,
            crate::DamageType::Magic
        );
    }
}

#[cfg(test)]
mod entity_kind_tests {
    use crate::{CastId, ChampionId, EntityKind, ItemId, RuneId};
    use strum::VariantArray;

    #[test]
    fn cast_id_default_kind_classifies_every_variant() {
        fn check<T: CastId + VariantArray>(expected: EntityKind) {
            for _ in T::VARIANTS {
                assert_eq!(T::KIND, expected);
            }
        }
        check::<ChampionId>(EntityKind::Champion);
        check::<ItemId>(EntityKind::Item);
        check::<RuneId>(EntityKind::Rune);
    }
}

#[cfg(all(test, feature = "yew"))]
mod recipe_tests {
    use crate::ItemId;

    #[test]
    fn item_statics_expose_const_recipe_slices() {
        const COMPONENTS: &[ItemId] = ItemId::BladeOfTheRuinedKing.data().builds_from;
        assert_eq!(
            COMPONENTS,
            &[ItemId::VampiricScepter, ItemId::RecurveBow, ItemId::Pickaxe]
        );

        const REPEATED: &[ItemId] = ItemId::PhantomDancer.data().builds_from;
        assert_eq!(REPEATED, &[ItemId::Dagger, ItemId::Zeal, ItemId::Dagger]);

        const UPGRADES: &[ItemId] = ItemId::Dagger.data().builds_into;
        assert!(UPGRADES.contains(&ItemId::PhantomDancer));
    }
}

#[cfg(feature = "engine")]
const _: () = {
    let mut i = 0;

    while i < crate::ChampionId::COUNT {
        let champion = crate::ChampionId::DATA[i];
        assert!(champion.metadata.len() == champion.closures.len());
        i += 1;
    }
};

#[cfg(feature = "yew")]
const _: () = {
    use {crate::ChampionId, tutorlolv2_types::ComboElement};

    let mut i = 0;

    while i < ChampionId::COUNT {
        let champion_id = ChampionId::VARIANTS[i];
        let merge_data = champion_id.merge_data();
        let combos = champion_id.combos();
        let len = champion_id.number_of_abilities();

        assert!(len == champion_id.number_of_abilities());
        assert!(len == champion_id.identifiers().len());

        let mut j = 0;

        while j < merge_data.len() {
            let m = &merge_data[j];

            assert!((m.min as usize) < len);
            assert!((m.max as usize) < len);
            assert!(m.min < m.max);

            if j + 1 < merge_data.len() {
                let a = &merge_data[j];
                let b = &merge_data[j + 1];
                assert!(a.max < b.max);
            }

            j += 1;
        }

        let mut k = 0;

        while k < combos.len() {
            let combo = combos[k];
            let mut l = 0;

            while l < combo.len() {
                let element = combo[l];

                if let ComboElement::Ability(ability_id) = element {
                    assert!(champion_id.index_of_ability(ability_id).is_some());
                }

                l += 1;
            }

            k += 1;
        }

        i += 1;
    }
};

const _: () = {
    use crate::{ItemId, RuneId};

    let mut i = 0;

    while i < ItemId::COUNT {
        let item = ItemId::VARIANTS[i];

        if item.deals_max_damage() {
            assert!(item.deals_damage());
        }

        i += 1;
    }

    let mut j = 0;

    while j < RuneId::COUNT {
        let rune = RuneId::VARIANTS[j];

        if rune.deals_max_damage() {
            assert!(rune.deals_damage());
        }

        j += 1;
    }
};
