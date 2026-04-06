use std::process::Command;
use tutorlolv2::ItemId;
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

#[test]
fn generate_html() {
    tutorlolv2_html::run();
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
/// cargo test --package tutorlolv2 --test dev -- cgen <champion> --exact --nocapture
fn cgen() {
    let name = std::env::args().nth(2).unwrap();
    ChampionFactory::run(&name).unwrap();
}

#[test]
fn cmkgen() {
    let name = std::env::args().nth(2).unwrap();
    println!("Champion: {name}");
    ChampionFactory::create(&name).unwrap();
}

#[test]
fn cgen_progress() {
    ChampionFactory::progress();
}

#[test]
fn update_items() {
    run_setup_items();
}
