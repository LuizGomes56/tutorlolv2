use tutorlolv2::{ChampionId, ItemId};
use tutorlolv2_dev::gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory};

#[test]
fn prettify_items() {
    tutorlolv2_dev::setup::update::prettify_internal_items().unwrap();
}

#[test]
fn run_setup_items() {
    tutorlolv2_dev::setup::update::setup_internal_items().unwrap();
    prettify_items();
}

#[test]
fn run_generate_items() {
    ItemFactory::create_all_raw().unwrap();
}

#[test]
fn run_items_generator() {
    ItemFactory::run_all().unwrap();
}

#[test]
fn run_generate_champions() {
    ChampionFactory::create_all().unwrap();
}

#[test]
fn run_champions_generator() {
    ChampionFactory::run_all().unwrap();
}

#[test]
fn run_ch_generator() {
    ChampionFactory::run(ChampionId::Akshan).unwrap();
}

#[test]
fn run_item_generator() {
    ItemFactory::run(ItemId::BladeOfTheRuinedKingArena).unwrap();
}

#[test]
fn update_items() {
    run_setup_items();
    run_generate_items();
}
