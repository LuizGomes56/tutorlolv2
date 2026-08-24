use strum::{EnumCount, VariantArray};

#[cfg(feature = "yew")]
const _: () = {
    use {crate::ChampionId, tutorlolv2_types::ComboElement};

    let mut i = 0;

    while i < ChampionId::VARIANTS {
        let champion_id = ChampionId::VALUES[i];
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
