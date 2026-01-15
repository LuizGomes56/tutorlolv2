#[test]
fn run_setup_items() {
    tutorlolv2_dev::setup::update::setup_internal_items().unwrap()
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
fn update_items() {
    run_items_generator();
    run_generate_items();
}
