#![cfg(feature = "yew")]

use tutorlolv2::ItemId;

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
