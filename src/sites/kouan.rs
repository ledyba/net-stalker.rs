use rss::Channel;
use sha2::Digest;
use super::*;

#[derive(Default)]
pub struct Kouan {}

impl Site for Kouan {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://www.moj.go.jp/psia/index.html")
        .await?
        .text_with_charset("UTF-8")
        .await?;

      let doc = scraper::Html::parse_document(&content);
      let news = build_rss(&doc);
      if let Some(news) = news {
        return Ok(news.to_string());
      }
      Err(anyhow::Error::msg("News not found"))
    })
  }
}

fn build_rss(doc: &scraper::Html) -> Option<Channel> {
  let selector = scraper::Selector::parse(".newsList").unwrap();
  let mut selected = doc.select(&selector);
  if let Some(elem) = selected.next() {
    let mut channel = Channel::default();
    channel.set_language("ja".to_string());
    channel.set_title("公安調査庁".to_string());
    channel.set_description("すべては国民の安全のために".to_string());
    channel.set_copyright("Copyright (C) Public Security Intelligence Agency All Rights Reserved.".to_string());
    channel.set_link("https://www.moj.go.jp/psia/index.html".to_string());
    let mut items = Vec::<rss::Item>::new();
    let selector = scraper::Selector::parse("a").unwrap();
    for elem in elem.select(&selector) {
      let mut item = rss::Item::default();
      item.set_title(elem.inner_html());
      item.set_link(elem.value().attr("href").map(|it| format!("https://www.moj.go.jp{}", it)));
      let guid = {
        let mut guid = rss::Guid::default();
        let mut hasher = sha2::Sha384::default();
        hasher.update(elem.html());
        let hash = format!("{:x}", hasher.finalize());
        guid.set_value(hash.to_string());
        guid.set_permalink(false);
        guid
      };
      item.set_guid(guid);
      items.push(item);
    }
    channel.set_items(items);
    return Some(channel);
  }
  None
}
