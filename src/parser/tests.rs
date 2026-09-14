// converted from tests/ integration tests: this package is binary-only,
// so crate-internal tests must live inside the module tree
use std::env;
use std::fs;
use std::path::PathBuf;
use crate::parser::load_resources_from_dir;
use crate::parser::rss_parser::{fetch, parse_feed};
use crate::model::resource::FeedResource;

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
//   cargo test test_fetch_all_urls_from_toml -- --ignored --nocapture
#[tokio::test]
#[ignore = "hits real network: run manually with `cargo test -- --ignored`"]
async fn test_fetch_all_urls_from_toml(){
    let feeds = feed_urls_from_toml();
    assert_eq!(feeds.len(), 6, "expect 6 enabled feeds in resources/feeds.example.toml");

    let cli = reqwest::Client::builder()
        .user_agent("EverydayRSS")
        .build()
        .unwrap();
    let mut failures: Vec<String> = vec![];

    for (name, url) in feeds{
        let resource = FeedResource{ name: name.clone(), url: url.clone(), enabled: true };
        let result = fetch(&resource, cli.clone()).await;
        match result{
            Ok(feed)=>{
                if feed.articles.is_empty(){
                    failures.push(format!("{} ({}): parsed 0 entries", name, url));
                }else{
                    let sample = &feed.articles[0];
                    println!(
                        "{} ({}): {} entries, first:title={:?} url={:?}",
                        name, url, feed.articles.len(), sample.title, sample.url
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
