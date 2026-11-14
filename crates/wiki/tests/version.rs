use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use wiki::{
    read_to_string,
    version::{archive, is_version},
};

fn temp_cache(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "tutorlolv2-version-{name}-{}-{unique}",
        std::process::id()
    ))
}

#[test]
fn versions_are_recognized() {
    assert!(is_version("16.21.1"));
    assert!(is_version("9.1"));
    assert!(!is_version("riot"));
    assert!(!is_version("16.21."));
    assert!(!is_version("16..1"));
    assert!(!is_version("16.21.1a"));
}

#[test]
fn archive_moves_cache_and_copies_stats() {
    let cache = temp_cache("archive");
    for (file, data) in [
        ("riot/items.json", "riot"),
        ("wiki/champions/full.json", "wiki"),
        ("stats/data.json", "stats"),
        ("16.20.1/riot/items.json", "older"),
    ] {
        let path = cache.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, data).unwrap();
    }

    let lock_file = cache.with_extension("lock.json");
    std::fs::write(&lock_file, "lock").unwrap();

    archive(&cache, "16.21.1", &lock_file).unwrap();

    let archived = cache.join("16.21.1");
    let read = |path: PathBuf| read_to_string(path).unwrap();
    assert_eq!(read(archived.join("riot/items.json")), "riot");
    assert_eq!(read(archived.join("wiki/champions/full.json")), "wiki");
    assert_eq!(read(archived.join("stats/data.json")), "stats");
    assert_eq!(read(cache.join("stats/data.json")), "stats");
    assert_eq!(read(cache.join("16.20.1/riot/items.json")), "older");
    assert!(!cache.join("riot").exists());
    assert!(!cache.join("wiki").exists());
    assert!(!archived.join("16.20.1").exists());
    assert_eq!(read(archived.join("wiki.lock.json")), "lock");
    assert_eq!(read(lock_file.clone()), "lock");

    let again = archive(&cache, "16.21.1", &lock_file);
    std::fs::remove_dir_all(&cache).unwrap();
    std::fs::remove_file(&lock_file).unwrap();
    assert!(
        again.is_err(),
        "an existing archive must not be overwritten"
    );
}

#[test]
fn archive_without_cache_does_nothing() {
    let cache = temp_cache("missing");
    archive(&cache, "16.21.1", cache.join("wiki.lock.json")).unwrap();
    assert!(!cache.exists());
}

#[test]
fn version_url_does_not_read_the_version_in_use() {
    // `version::check` fetches this url before deciding the version in use, so
    // reading it here would panic, as tests run without `LOL_VERSION` or `.env`
    assert_eq!(
        wiki::client::DDragon::Version.url(),
        "https://ddragon.leagueoflegends.com/api/versions.json"
    );
}

#[test]
fn archive_keeps_recommendations_active_and_snapshots_other_app_outputs() {
    let cache = temp_cache("recommendations");
    let recommendations = r#"{"Aatrox":{"Top":[["BlackCleaver"],["Conqueror"]]}}"#;
    for (file, data) in [
        ("app/scraper/data.json", recommendations),
        ("app/items.json", "old generated items"),
        ("app/html/champions/Aatrox.html.br", "old docs"),
    ] {
        let path = cache.join(file);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, data).unwrap();
    }

    archive(&cache, "16.22.1", cache.join("missing.lock.json")).unwrap();

    let active = cache.join("app/scraper/data.json");
    let snapshot = cache.join("16.22.1/app/scraper/data.json");
    assert_eq!(read_to_string(&active).unwrap(), recommendations);
    assert_eq!(read_to_string(&snapshot).unwrap(), recommendations);
    assert_eq!(
        read_to_string(cache.join("16.22.1/app/items.json")).unwrap(),
        "old generated items"
    );
    assert_eq!(
        read_to_string(cache.join("16.22.1/app/html/champions/Aatrox.html.br")).unwrap(),
        "old docs"
    );
    assert!(!cache.join("app/items.json").exists());
    assert!(!cache.join("app/html").exists());

    // Refreshing active recommendations must not change the previous patch's snapshot.
    let updated = r#"{"Aatrox":{"Top":[["SunderedSky"],["Conqueror"]]}}"#;
    std::fs::write(&active, updated).unwrap();
    archive(&cache, "16.23.1", cache.join("missing.lock.json")).unwrap();
    assert_eq!(read_to_string(&active).unwrap(), updated);
    assert_eq!(read_to_string(&snapshot).unwrap(), recommendations);
    assert_eq!(
        read_to_string(cache.join("16.23.1/app/scraper/data.json")).unwrap(),
        updated
    );
    std::fs::remove_dir_all(cache).unwrap();
}

#[test]
fn archive_without_recommendations_does_not_recreate_app_cache() {
    let cache = temp_cache("no-recommendations");
    let items = cache.join("app/items.json");
    std::fs::create_dir_all(items.parent().unwrap()).unwrap();
    std::fs::write(items, "generated items").unwrap();

    archive(&cache, "16.22.1", cache.join("missing.lock.json")).unwrap();

    assert!(!cache.join("app").exists());
    assert_eq!(
        read_to_string(cache.join("16.22.1/app/items.json")).unwrap(),
        "generated items"
    );
    std::fs::remove_dir_all(cache).unwrap();
}
