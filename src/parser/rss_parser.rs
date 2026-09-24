use crate::app::AppError;
use crate::model::article::{Article, Feed};
use crate::model::resource::FeedResource;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use tracing::{debug, info};

#[derive(Debug, Clone, Copy)]
pub struct DateFilter {
    pub lookback_hours: u32,
    pub include_undated: bool,
}

/// fetch rss feed
pub async fn fetch(
    feed_rs: &FeedResource,
    cli: Client,
    filter: DateFilter,
) -> Result<Feed, AppError> {
    let res = cli.get(&feed_rs.url).send().await?.error_for_status()?;
    let raw = res.text().await?;
    let len = raw.len();
    debug!("Request content {}", raw);
    info!("Total bytes of {} result {:?}", feed_rs.name, len);
    //feed parser
    let articles = parse_feed_with_filter(raw, filter, Utc::now())?;
    let f = Feed {
        title: feed_rs.name.clone(),
        url: feed_rs.url.clone(),
        articles,
    };
    Ok(f)
}
/// parse content of rss feed into EverydayRSS::model::article
#[cfg(test)]
pub fn parse_feed(content: String) -> Result<Vec<Article>, AppError> {
    parse_feed_entries(content, None, Utc::now())
}

pub fn parse_feed_with_filter(
    content: String,
    filter: DateFilter,
    now: DateTime<Utc>,
) -> Result<Vec<Article>, AppError> {
    parse_feed_entries(content, Some(filter), now)
}

fn parse_feed_entries(
    content: String,
    filter: Option<DateFilter>,
    now: DateTime<Utc>,
) -> Result<Vec<Article>, AppError> {
    let raw_feed = feed_rs::parser::parse(content.as_bytes())?;
    let mut feed: Vec<Article> = vec![];
    debug!("raw feed content {:?}", raw_feed);
    for entry in raw_feed.entries {
        let published_at = entry
            .published
            .or(entry.updated)
            .map(|date| date.with_timezone(&Utc));
        if let Some(filter) = filter
            && !is_in_date_window(published_at.as_ref(), filter, now)
        {
            continue;
        }

        let mut article: Article = Article::default();
        if let Some(title) = entry.title {
            article.title = title.content;
        }
        if let Some(link) = entry.links.first() {
            article.url = link.href.clone();
        }
        if let Some(summary) = entry.summary {
            article.description += summary.content.as_str();
        }
        if let Some(content) = entry.content {
            article.description += content.body.unwrap_or_default().as_str();
        }
        article.published_at = published_at;
        feed.push(article);
    }
    Ok(feed)
}

fn is_in_date_window(
    published_at: Option<&DateTime<Utc>>,
    filter: DateFilter,
    now: DateTime<Utc>,
) -> bool {
    let Some(published_at) = published_at else {
        return filter.include_undated;
    };
    let earliest = now - Duration::hours(i64::from(filter.lookback_hours));
    // Allow a small amount of clock skew, but reject genuinely future-dated posts.
    let latest = now + Duration::minutes(5);
    *published_at >= earliest && *published_at <= latest
}

#[cfg(test)]
mod tests {
    use crate::parser::rss_parser::{DateFilter, parse_feed, parse_feed_with_filter};
    use chrono::{TimeZone, Utc};

    #[test]
    fn filters_entries_by_published_or_updated_date() {
        let content = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Date filter</title>
  <id>https://example.test/feed</id>
  <updated>2026-09-23T08:00:00Z</updated>
  <entry>
    <title>Recent published</title><id>recent</id>
    <updated>2026-09-20T08:00:00Z</updated>
    <published>2026-09-23T02:00:00Z</published>
  </entry>
  <entry>
    <title>Recent updated</title><id>updated</id>
    <updated>2026-09-22T20:00:00Z</updated>
  </entry>
  <entry>
    <title>Stale</title><id>stale</id>
    <updated>2026-09-20T08:00:00Z</updated>
  </entry>
  <entry>
    <title>Future</title><id>future</id>
    <updated>2026-09-24T08:00:00Z</updated>
  </entry>
  <entry><title>Undated</title><id>undated</id></entry>
</feed>"#;
        let now = Utc
            .with_ymd_and_hms(2026, 9, 23, 8, 0, 0)
            .single()
            .expect("valid test date");
        let filter = DateFilter {
            lookback_hours: 24,
            include_undated: false,
        };

        let articles =
            parse_feed_with_filter(content.to_string(), filter, now).expect("feed should parse");

        assert_eq!(
            articles
                .iter()
                .map(|item| item.title.as_str())
                .collect::<Vec<_>>(),
            vec!["Recent published", "Recent updated"]
        );
        assert_eq!(
            articles[0]
                .published_at
                .expect("published date should be retained"),
            Utc.with_ymd_and_hms(2026, 9, 23, 2, 0, 0)
                .single()
                .expect("valid test date")
        );
    }

    #[test]
    fn can_keep_entries_without_dates() {
        let content = r#"<?xml version="1.0"?>
<rss version="2.0"><channel><title>Example</title><link>https://example.test</link>
<description>Example</description><item><title>Undated</title></item></channel></rss>"#;
        let now = Utc
            .with_ymd_and_hms(2026, 9, 23, 8, 0, 0)
            .single()
            .expect("valid test date");

        let articles = parse_feed_with_filter(
            content.to_string(),
            DateFilter {
                lookback_hours: 24,
                include_undated: true,
            },
            now,
        )
        .expect("feed should parse");

        assert_eq!(articles.len(), 1);
        assert!(articles[0].published_at.is_none());
    }

    // ---------- RSS 0.92 (Userland) ----------
    // note: feed-rs 2.4.0 dispatches <rdf:RDF> roots to the RSS 1.0 parser unconditionally,
    // so authentic RSS 0.90 (Netscape namespace) documents yield no entries;
    // the 0.9x family is therefore covered via 0.91 and 0.92 here.
    #[test]
    fn test_parse_feed_rss092_basic() {
        let content = r#"<?xml version="1.0"?>
<rss version="0.92">
  <channel>
    <title>Example 0.92 Channel</title>
    <link>http://example.com/</link>
    <description>channel description</description>
    <item>
      <title>0.92 Item One</title>
      <link>http://example.com/1</link>
      <description>first item summary</description>
    </item>
    <item>
      <title>0.92 Item Two</title>
      <link>http://example.com/2</link>
      <description>second item summary</description>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].title, "0.92 Item One");
        assert_eq!(feeds[0].url, "http://example.com/1");
        assert_eq!(feeds[0].description, "first item summary");
        assert_eq!(feeds[1].title, "0.92 Item Two");
        assert_eq!(feeds[1].url, "http://example.com/2");
    }

    #[test]
    fn test_parse_feed_rss092_cdata_description() {
        let content = r#"<?xml version="1.0"?>
<rss version="0.92">
  <channel>
    <title>Example 0.92 Channel</title>
    <link>http://example.com/</link>
    <description>channel description</description>
    <item>
      <title>0.92 HTML Item</title>
      <link>http://example.com/html</link>
      <description><![CDATA[<p>0.92 <i>html</i> description</p>]]></description>
      <enclosure url="http://example.com/audio.mp3" length="12345" type="audio/mpeg"/>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "0.92 HTML Item");
        assert_eq!(feeds[0].url, "http://example.com/html");
        assert_eq!(feeds[0].description, "<p>0.92 <i>html</i> description</p>");
    }

    // ---------- RSS 0.91 (Netscape/Userland, non-RDF) ----------
    #[test]
    fn test_parse_feed_rss091_basic() {
        let content = r#"<?xml version="1.0"?>
<!DOCTYPE rss PUBLIC "-//Netscape Communications//DTD RSS 0.91//EN" "http://my.netscape.com/public/dtds/rss-0.91.dtd">
<rss version="0.91">
  <channel>
    <title>Example 0.91 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
    <item>
      <title>0.91 Item One</title>
      <link>http://example.org/1</link>
      <description>first item summary</description>
    </item>
    <item>
      <title>0.91 Item Two</title>
      <link>http://example.org/2</link>
      <description>second item summary</description>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].title, "0.91 Item One");
        assert_eq!(feeds[0].url, "http://example.org/1");
        assert_eq!(feeds[0].description, "first item summary");
        assert_eq!(feeds[1].title, "0.91 Item Two");
        assert_eq!(feeds[1].url, "http://example.org/2");
    }

    #[test]
    fn test_parse_feed_rss091_cdata_description() {
        let content = r#"<?xml version="1.0"?>
<rss version="0.91">
  <channel>
    <title>Example 0.91 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
    <item>
      <title>HTML Item</title>
      <link>http://example.org/html</link>
      <description><![CDATA[<p>HTML <b>bold</b> content</p>]]></description>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "HTML Item");
        assert_eq!(feeds[0].description, "<p>HTML <b>bold</b> content</p>");
    }

    // ---------- RSS 1.0 (RDF, purl.org/rss/1.0) ----------
    #[test]
    fn test_parse_feed_rss1_basic() {
        let content = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns="http://purl.org/rss/1.0/">
  <channel rdf:about="http://example.org/rss1">
    <title>Example RSS 1.0 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
  </channel>
  <item rdf:about="http://example.org/1">
    <title>RSS 1.0 First</title>
    <link>http://example.org/1</link>
    <description>first item summary</description>
  </item>
  <item rdf:about="http://example.org/2">
    <title>RSS 1.0 Second</title>
    <link>http://example.org/2</link>
    <description>second item summary</description>
  </item>
</rdf:RDF>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].title, "RSS 1.0 First");
        assert_eq!(feeds[0].url, "http://example.org/1");
        assert_eq!(feeds[0].description, "first item summary");
        assert_eq!(feeds[1].title, "RSS 1.0 Second");
    }

    #[test]
    fn test_parse_feed_rss1_cdata_with_module() {
        let content = r#"<?xml version="1.0"?>
<rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
         xmlns="http://purl.org/rss/1.0/"
         xmlns:dc="http://purl.org/dc/elements/1.1/">
  <channel rdf:about="http://example.org/rss1">
    <title>Example RSS 1.0 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
  </channel>
  <item rdf:about="http://example.org/dc">
    <title>Item With CDATA</title>
    <link>http://example.org/dc</link>
    <description><![CDATA[<em>emphasised</em> summary]]></description>
    <dc:creator>UserB1ank</dc:creator>
  </item>
</rdf:RDF>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Item With CDATA");
        assert_eq!(feeds[0].url, "http://example.org/dc");
        assert_eq!(feeds[0].description, "<em>emphasised</em> summary");
    }

    // ---------- RSS 2.0 (Userland, default namespace) ----------
    #[test]
    fn test_parse_feed_rss2_basic() {
        let content = r#"<?xml version="1.0"?>
<rss version="2.0">
  <channel>
    <title>Example RSS 2.0 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
    <item>
      <title>RSS 2.0 Item One</title>
      <link>http://example.org/1</link>
      <description>first item summary</description>
      <guid isPermaLink="false">tag:example.org,2026:1</guid>
      <pubDate>Mon, 07 Sep 2026 08:00:00 GMT</pubDate>
    </item>
    <item>
      <title>RSS 2.0 Item Two</title>
      <link>http://example.org/2</link>
      <description>second item summary</description>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].title, "RSS 2.0 Item One");
        assert_eq!(feeds[0].url, "http://example.org/1");
        assert_eq!(feeds[0].description, "first item summary");
        assert_eq!(feeds[1].title, "RSS 2.0 Item Two");
        assert_eq!(feeds[1].description, "second item summary");
    }

    #[test]
    fn test_parse_feed_rss2_content_encoded() {
        let content = r#"<?xml version="1.0"?>
<rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/">
  <channel>
    <title>Example RSS 2.0 Channel</title>
    <link>http://example.org/</link>
    <description>channel description</description>
    <item>
      <title>Full Content Item</title>
      <link>http://example.org/full</link>
      <description>short excerpt</description>
      <content:encoded><![CDATA[<p>The complete article body</p>]]></content:encoded>
    </item>
  </channel>
</rss>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Full Content Item");
        assert_eq!(feeds[0].url, "http://example.org/full");
        // description = <description>(summary) + <content:encoded>(content)
        assert_eq!(
            feeds[0].description,
            "short excerpt<p>The complete article body</p>"
        );
    }

    // ---------- Atom (IETF RFC 4287) ----------
    #[test]
    fn test_parse_feed_atom_basic() {
        let content = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Example Atom Feed</title>
  <link href="http://example.org/"/>
  <updated>2026-09-07T12:00:00Z</updated>
  <id>urn:uuid:60ec0e30-cab7-4bd6-98bb-6d5d3f9e71f6</id>
  <entry>
    <title>Atom First Entry</title>
    <link href="http://example.org/1" rel="alternate" type="text/html"/>
    <id>urn:uuid:8fd6a935-6a2f-4bb8-a1e3-2d47e7a4b18f</id>
    <updated>2026-09-07T12:00:00Z</updated>
    <summary>atom entry summary</summary>
  </entry>
</feed>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Atom First Entry");
        assert_eq!(feeds[0].url, "http://example.org/1");
        assert_eq!(feeds[0].description, "atom entry summary");
    }

    #[test]
    fn test_parse_feed_atom_html_content() {
        let content = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Example Atom Feed</title>
  <link href="http://example.org/"/>
  <updated>2026-09-07T12:00:00Z</updated>
  <id>urn:uuid:60ec0e30-cab7-4bd6-98bb-6d5d3f9e71f6</id>
  <entry>
    <title>Atom Content Entry</title>
    <link href="http://example.org/2" rel="alternate"/>
    <id>urn:uuid:9a76b3e5-0e1d-4f1b-b39d-4b0f8b8f31f9</id>
    <updated>2026-09-07T13:00:00Z</updated>
    <content type="html">&lt;p&gt;atom full body&lt;/p&gt;</content>
  </entry>
</feed>"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "Atom Content Entry");
        assert_eq!(feeds[0].url, "http://example.org/2");
        // entry only has <content>, no <summary>
        assert_eq!(feeds[0].description, "<p>atom full body</p>");
    }

    // ---------- JSON Feed (jsonfeed.org, v1.0 & v1.1) ----------
    #[test]
    fn test_parse_feed_json_v11() {
        let content = r#"{
  "version": "https://jsonfeed.org/version/1.1",
  "title": "Example JSON Feed",
  "home_page_url": "http://example.org/",
  "items": [
    {
      "id": "http://example.org/1",
      "url": "http://example.org/1",
      "title": "JSON Feed Item One",
      "summary": "json item summary",
      "content_html": "<p>json item body</p>"
    }
  ]
}"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].title, "JSON Feed Item One");
        assert_eq!(feeds[0].url, "http://example.org/1");
        // description = summary + content_html
        assert_eq!(
            feeds[0].description,
            "json item summary<p>json item body</p>"
        );
    }

    #[test]
    fn test_parse_feed_json_v10_content_text() {
        let content = r#"{
  "version": "https://jsonfeed.org/version/1",
  "title": "Example JSON Feed 1.0",
  "items": [
    {
      "id": "a",
      "url": "http://example.org/a",
      "title": "Text Item A",
      "content_text": "plain text body a"
    },
    {
      "id": "b",
      "url": "http://example.org/b",
      "title": "Text Item B",
      "content_text": "plain text body b"
    }
  ]
}"#;
        let feeds = parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].title, "Text Item A");
        assert_eq!(feeds[0].url, "http://example.org/a");
        assert_eq!(feeds[0].description, "plain text body a");
        assert_eq!(feeds[1].title, "Text Item B");
        assert_eq!(feeds[1].description, "plain text body b");
    }
}
