use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Jma {}

impl Site for Jma {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://www.jma.go.jp/jma/index.html")
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
  let selector = scraper::Selector::parse("#news a").expect("[BUG] Invalid selector");
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("気象庁".to_string());
  channel.set_description("国土交通省の下部団体です".to_string());
  channel.set_copyright("".to_string());
  channel.set_link("https://www.jma.go.jp/jma/".to_string());
  let mut items = Vec::<rss::Item>::new();
  for elem in doc.select(&selector) {
    let link = elem.value().attr("href").map(ToString::to_string).expect("Failed to get link");
    let mut item = rss::Item::default();
    item.set_title(elem.inner_html());
    item.set_link(link.clone());
    let guid = {
      let mut guid = rss::Guid::default();
      guid.set_value(link.clone());
      guid.set_permalink(true);
      guid
    };
    item.set_guid(guid);
    items.push(item);
  }
  channel.set_items(items);
  return Some(channel);
}
