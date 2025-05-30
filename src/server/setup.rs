use actix_web::{HttpResponse, Responder, post};

use crate::{
    server::schemas::APIResponse,
    setup::{
        images::{
            img_download_arts, img_download_instances, img_download_items, img_download_runes,
        },
        update::{
            append_prettified_item_stats, generate_writer_files, get_meta_items,
            identify_damaging_items, initialize_items, replace_item_names_with_ids,
            rewrite_champion_names, setup_champion_cache, setup_project_folders, update_instances,
            update_riot_cache,
        },
    },
};

#[post("/project")]
pub async fn setup_project() -> impl Responder {
    setup_project_folders();

    tokio::spawn(async move {
        let (_, _, _, _, _) = tokio::join!(
            tokio::spawn(update_riot_cache()),
            tokio::spawn(update_instances("champions")),
            tokio::spawn(generate_writer_files()),
            tokio::spawn(update_instances("items")),
            tokio::spawn(get_meta_items()),
        );

        replace_item_names_with_ids();
        setup_champion_cache();
        initialize_items();
        identify_damaging_items();
        rewrite_champion_names();
        append_prettified_item_stats().await;
    });

    let (_, _, _, _) = tokio::join!(
        tokio::spawn(img_download_arts()),
        tokio::spawn(img_download_instances()),
        tokio::spawn(img_download_items()),
        tokio::spawn(img_download_runes()),
    );

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Project complete setup started".to_string(),
        data: (),
    })
}

#[post("/folders")]
pub async fn setup_folders() -> impl Responder {
    setup_project_folders();
    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Folders are ready".to_string(),
        data: (),
    })
}

#[post("/champions")]
pub async fn setup_champions() -> impl Responder {
    setup_champion_cache();

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: "Champions are ready".to_string(),
        data: (),
    })
}
