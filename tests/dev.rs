use tutorlolv2::ChampionId;

#[test]
fn run_setup_items() {
    tutorlolv2_dev::setup::update::setup_internal_items().unwrap();
    tutorlolv2_dev::setup::update::prettify_internal_items().unwrap();
}

#[test]
fn run_generate_items() {
    tutorlolv2_dev::gen_factories::fac_items::ItemFactory::create_all_raw().unwrap();
}

#[test]
fn run_items_generator() {
    tutorlolv2_dev::gen_factories::fac_items::ItemFactory::run_all().unwrap();
}

#[test]
fn run_generate_champions() {
    tutorlolv2_dev::gen_factories::fac_champions::ChampionFactory::create_all().unwrap();
}

#[test]
fn run_champions_generator() {
    tutorlolv2_dev::gen_factories::fac_champions::ChampionFactory::run_all().unwrap();
}

#[test]
fn run_ch_generator() {
    tutorlolv2_dev::gen_factories::fac_champions::ChampionFactory::run(ChampionId::Akshan).unwrap();
}

#[test]
fn update_items() {
    run_setup_items();
    run_generate_items();
}
