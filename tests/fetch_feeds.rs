use std::env;
use std::fs;
use std::path::PathBuf;
use EverydayRSS::parser::load_resources_from_dir;
use EverydayRSS::parser::rss_parser::{fetch, parse_feed};

// collect enabled feed urls from resources/*.toml in document order
fn feed_urls_from_toml() -> Vec<(String, String)>{
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let resource_dir = PathBuf::from(manifest_dir).join("resources");
    let rss_resources = load_resources_from_dir(resource_dir).unwrap();

    let mut feeds = vec![];
    for rss in &rss_resources{
        for group in &rss.group{
            for feed in &group.feed{
                if feed.enabled{
                    feeds.push((feed.name.clone(), feed.url.clone()));
                }
            }
        }
    }
    feeds
}

// offline: parse the real feed documents previously downloaded from the toml urls
// (fixtures fetched with curl on 2026-09-08; mirror.xyz was dead and
// jsonfeed.org/feed.json returned 404, so no fixtures exist for them)
#[test]
fn test_parse_real_feed_fixtures(){
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let fixtures_dir = PathBuf::from(manifest_dir).join("tests").join("fixtures");

    let mut checked = 0;
    for entry in fs::read_dir(&fixtures_dir).unwrap(){
        let file = entry.unwrap();
        let path = file.path();
        let name = file.file_name().to_string_lossy().to_string();
        let content = fs::read_to_string(&path).unwrap();
        let feeds = parse_feed(content)
            .unwrap_or_else(|e| panic!("parse fixture {} failed: {}", name, e));
        assert!(!feeds.is_empty(), "fixture {} parsed 0 entries", name);
        let first = &feeds[0];
        assert!(
            !first.title.is_empty() || !first.url.is_empty(),
            "fixture {} first entry has neither title nor url", name
        );
        println!("{}: {} entries, first title={:?}", name, feeds.len(), first.title);
        checked += 1;
    }
    assert!(checked >= 4, "expected at least 4 fixtures, got {}", checked);
}

// online: fetch and parse every enabled url from the toml config.
// requires direct outbound network access, run with:
//   cargo test --test fetch_feeds -- --ignored --nocapture
#[tokio::test]
#[ignore = "hits real network: run manually with `cargo test -- --ignored`"]
async fn test_fetch_all_urls_from_toml(){
    let feeds = feed_urls_from_toml();
    assert_eq!(feeds.len(), 6, "expect 6 enabled feeds in resources/feeds.example.toml");

    let mut failures: Vec<String> = vec![];

    for (name, url) in feeds{
        let result = fetch(&url, Some(10), None).await;
        match result{
            Ok(entries)=>{
                if entries.is_empty(){
                    failures.push(format!("{} ({}): parsed 0 entries", name, url));
                }else{
                    let sample = &entries[0];
                    println!(
                        "{} ({}): {} entries, first: title={:?} url={:?}",
                        name, url, entries.len(), sample.title, sample.url
                    );
                }
            },
            Err(e)=>{
                failures.push(format!("{} ({}): fetch error: {}", name, url, e));
            },
        }
    }

    assert!(
        failures.is_empty(),
        "failed feeds:\n{}",
        failures.join("\n")
    );
}
