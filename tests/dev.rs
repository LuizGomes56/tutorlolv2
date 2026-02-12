use std::process::Command;
use tutorlolv2::{ChampionId, ItemId};
use tutorlolv2_dev::{
    HTTP_CLIENT,
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
};

#[test]
fn setup_folders() {
    tutorlolv2_dev::setup::update::setup_project_folders().unwrap();
}

#[tokio::test]
async fn scraper() {
    dotenvy::dotenv().unwrap();
    if let Err(e) = HTTP_CLIENT.call_scraper().await {
        println!("[!error] {e}");
    }
}

#[test]
fn update() {
    dotenvy::dotenv().unwrap();
    tutorlolv2_dev::setup::update::setup_project_folders().unwrap();
    ChampionFactory::create_all().unwrap();
    ChampionFactory::run_all().unwrap();
    ItemFactory::run_all().unwrap();
    Command::new("./build.bat").spawn().unwrap().wait().unwrap();
}

#[tokio::test]
async fn generate_html() {
    // tutorlolv2_html::run().await;
}

#[test]
fn prettify_items() {
    tutorlolv2_dev::setup::update::prettify_internal_items().unwrap();
}

#[test]
fn run_setup_items() {
    dotenvy::dotenv().unwrap();
    tutorlolv2_dev::setup::update::setup_internal_items().unwrap();
    prettify_items();
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
    let data = ItemFactory::run(ItemId::BladeOfTheRuinedKingArena).unwrap();
    println!("Data: {}", data.current_data.ranged.minimum_damage);
}

#[test]
fn update_items() {
    run_setup_items();
}
