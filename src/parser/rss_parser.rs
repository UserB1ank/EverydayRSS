use std::time::Duration;
use reqwest::{Body, Client, Proxy};

/// description: summary content description
/// title: title
/// url: link
#[derive(Debug)]
pub struct Feed{
    title:String,
    url:String,
    description:String
}


pub async fn fetch(url:&str,timeout:Option<usize>,proxy: Option<String>)->Result<Vec<Feed>,Box<dyn std::error::Error>>{
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
    let raw=res.text().await?;
    let text=urlencoding::decode(raw.as_str())?;
    #[cfg(debug_assertions)]
    println!("reqwest content: {}",text);
    //feed parser
    let mut feeds:Vec<Feed>=vec![];
    Ok(feeds)
}

fn parser(content:String)->Result<Feed,Box<dyn std::error::Error>>{
    let raw_feed=feed_rs::parser::parse(content.as_bytes())?;
    
}

#[cfg(test)]
mod tests{
    use crate::parser::rss_parser::fetch;

    #[tokio::test]
    async fn test_request() {
        let res=fetch("https://baidu.com",None,None).await.unwrap();
        println!("{:?}", res);
    }
}