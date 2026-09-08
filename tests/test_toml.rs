use std::env;
use std::path::PathBuf;
use EverydayRSS::parser::load_resources_from_dir;

// offline: load the example feeds config and verify its structure
#[test]
fn test_load_resources_from_dir(){
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let resource_dir = PathBuf::from(manifest_dir).join("resources");
    let rss_resources = load_resources_from_dir(resource_dir).unwrap();

    assert_eq!(rss_resources.len(), 1, "expect exactly one toml file in resources/");

    let rss = &rss_resources[0];
    assert_eq!(rss.group.len(), 3, "expect 3 groups (RSS 2.0 / Atom 1.0 / JSON Feed)");

    let total: usize = rss.group.iter().map(|g| g.feed.len()).sum();
    assert_eq!(total, 6, "expect 6 feeds in total");

    for group in &rss.group{
        assert!(!group.name.is_empty());
        for feed in &group.feed{
            assert!(feed.enabled, "feed {} should be enabled", feed.name);
            assert!(feed.url.starts_with("http"), "feed {} url should be http(s): {}", feed.name, feed.url);
        }
    }
}
