use std::time::Duration;
use reqwest::{ Client, Proxy};
use tracing::debug;
use crate::app::AppError;
use crate::model::article::Article;

pub async fn fetch(url:&str, timeout: &Option<u32>, proxy: &Option<String>) ->Result<Vec<Article>,AppError>{
    //initialize client
    let timeout=timeout.unwrap_or(3);
    let mut builder =Client::builder();
    builder = builder.timeout(Duration::from_secs(timeout as u64));
    if let Some(proxy)=proxy{
        let proxy=Proxy::http(proxy)?;
        builder = builder.proxy(proxy);
    };
    builder = builder.user_agent("EveryDayRSS");
    let cli=builder.build()?;
    let res=cli.get(url).send().await?;
    let len=res.content_length().unwrap();
    let raw=res.text().await?;
    debug!("Request content {}",raw);
    debug!("Total bytes of request result {:?}",len);
    //feed parser
    Ok(parse_feed(raw)?)
}

pub fn parse_feed(content:String)->Result<Vec<Article>,AppError>{
    let raw_feed=feed_rs::parser::parse(content.as_bytes())?;
    let mut feed:Vec<Article>=vec![];
    #[cfg(test)]
    dbg!(&raw_feed);
    for entry in raw_feed.entries{
        let mut article: Article = Article::default();

        if let Some(title)=entry.title{
            article.title=title.content;
        }
        if let Some(link)=entry.links.first(){
            article.url=link.href.clone();
        }
        if let Some(summary)=entry.summary{
            article.description+=summary.content.as_str();
        }
        if let Some(content)=entry.content{
            article.description+=content.body.unwrap_or("".to_string()).as_str();
        }
        feed.push(article);
    }
    Ok(feed)
}

#[cfg(test)]
mod tests{
    use crate::parser::rss_parser::parse_feed;

    // ---------- RSS 0.92 (Userland) ----------
    // note: feed-rs 2.4.0 dispatches <rdf:RDF> roots to the RSS 1.0 parser unconditionally,
    // so authentic RSS 0.90 (Netscape namespace) documents yield no entries;
    // the 0.9x family is therefore covered via 0.91 and 0.92 here.
    #[test]
    fn test_parse_feed_rss092_basic(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),2);
        assert_eq!(feeds[0].title,"0.92 Item One");
        assert_eq!(feeds[0].url,"http://example.com/1");
        assert_eq!(feeds[0].description,"first item summary");
        assert_eq!(feeds[1].title,"0.92 Item Two");
        assert_eq!(feeds[1].url,"http://example.com/2");
    }

    #[test]
    fn test_parse_feed_rss092_cdata_description(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"0.92 HTML Item");
        assert_eq!(feeds[0].url,"http://example.com/html");
        assert_eq!(feeds[0].description,"<p>0.92 <i>html</i> description</p>");
    }

    // ---------- RSS 0.91 (Netscape/Userland, non-RDF) ----------
    #[test]
    fn test_parse_feed_rss091_basic(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),2);
        assert_eq!(feeds[0].title,"0.91 Item One");
        assert_eq!(feeds[0].url,"http://example.org/1");
        assert_eq!(feeds[0].description,"first item summary");
        assert_eq!(feeds[1].title,"0.91 Item Two");
        assert_eq!(feeds[1].url,"http://example.org/2");
    }

    #[test]
    fn test_parse_feed_rss091_cdata_description(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"HTML Item");
        assert_eq!(feeds[0].description,"<p>HTML <b>bold</b> content</p>");
    }

    // ---------- RSS 1.0 (RDF, purl.org/rss/1.0) ----------
    #[test]
    fn test_parse_feed_rss1_basic(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),2);
        assert_eq!(feeds[0].title,"RSS 1.0 First");
        assert_eq!(feeds[0].url,"http://example.org/1");
        assert_eq!(feeds[0].description,"first item summary");
        assert_eq!(feeds[1].title,"RSS 1.0 Second");
    }

    #[test]
    fn test_parse_feed_rss1_cdata_with_module(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"Item With CDATA");
        assert_eq!(feeds[0].url,"http://example.org/dc");
        assert_eq!(feeds[0].description,"<em>emphasised</em> summary");
    }

    // ---------- RSS 2.0 (Userland, default namespace) ----------
    #[test]
    fn test_parse_feed_rss2_basic(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),2);
        assert_eq!(feeds[0].title,"RSS 2.0 Item One");
        assert_eq!(feeds[0].url,"http://example.org/1");
        assert_eq!(feeds[0].description,"first item summary");
        assert_eq!(feeds[1].title,"RSS 2.0 Item Two");
        assert_eq!(feeds[1].description,"second item summary");
    }

    #[test]
    fn test_parse_feed_rss2_content_encoded(){
        let content=r#"<?xml version="1.0"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"Full Content Item");
        assert_eq!(feeds[0].url,"http://example.org/full");
        // description = <description>(summary) + <content:encoded>(content)
        assert_eq!(feeds[0].description,"short excerpt<p>The complete article body</p>");
    }

    // ---------- Atom (IETF RFC 4287) ----------
    #[test]
    fn test_parse_feed_atom_basic(){
        let content=r#"<?xml version="1.0" encoding="utf-8"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"Atom First Entry");
        assert_eq!(feeds[0].url,"http://example.org/1");
        assert_eq!(feeds[0].description,"atom entry summary");
    }

    #[test]
    fn test_parse_feed_atom_html_content(){
        let content=r#"<?xml version="1.0" encoding="utf-8"?>
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"Atom Content Entry");
        assert_eq!(feeds[0].url,"http://example.org/2");
        // entry only has <content>, no <summary>
        assert_eq!(feeds[0].description,"<p>atom full body</p>");
    }

    // ---------- JSON Feed (jsonfeed.org, v1.0 & v1.1) ----------
    #[test]
    fn test_parse_feed_json_v11(){
        let content=r#"{
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),1);
        assert_eq!(feeds[0].title,"JSON Feed Item One");
        assert_eq!(feeds[0].url,"http://example.org/1");
        // description = summary + content_html
        assert_eq!(feeds[0].description,"json item summary<p>json item body</p>");
    }

    #[test]
    fn test_parse_feed_json_v10_content_text(){
        let content=r#"{
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
        let feeds=parse_feed(content.to_string()).unwrap();
        assert_eq!(feeds.len(),2);
        assert_eq!(feeds[0].title,"Text Item A");
        assert_eq!(feeds[0].url,"http://example.org/a");
        assert_eq!(feeds[0].description,"plain text body a");
        assert_eq!(feeds[1].title,"Text Item B");
        assert_eq!(feeds[1].description,"plain text body b");
    }
}