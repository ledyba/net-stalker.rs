use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Akotsu;

impl Site for Akotsu {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://akotsu.com/blog/")
        .await?
        .text_with_charset("UTF-8")
        .await?;

      let doc = scraper::Html::parse_document(&content);
      let news = build_rss(&doc)?;
      Ok(news.to_string())
    })
  }
}

const BASE_URL: &'static str = "https://akotsu.com/blog/";

fn build_rss(doc: &scraper::Html) -> anyhow::Result<Channel> {
  let selector = scraper::Selector::parse("#contents").expect("[BUG] Invalid selector");
  let mut selected = doc.select(&selector);
  if let Some(elem) = selected.next() {
    let mut channel = Channel::default();
    channel.set_language("ja".to_string());
    channel.set_title("蛙骨".to_string());
    channel.set_description("あこつの おみごと さいと".to_string());
    channel.set_copyright("齋木麻由（P.N　蛙骨）".to_string());
    channel.set_link("https://akotsu.com/".to_string());
    let mut items = Vec::<rss::Item>::new();
    let selector = scraper::Selector::parse("article > h2 > a").expect("[BUG] Invalid selector");
    for elem in elem.select(&selector) {
      let Some(link) = elem.value().attr("href") else {
        continue
      };
      let link = BASE_URL.to_owned() + link;
      let mut item = rss::Item::default();
      item.set_title(elem.inner_html());
      item.set_link(link.clone());
      let guid = {
        let mut guid = rss::Guid::default();
        guid.set_value(link);
        guid.set_permalink(true);
        guid
      };
      item.set_guid(guid);
      items.push(item);
    }
    if items.is_empty() {
      return Err(anyhow::Error::msg("No items!"));
    }
    channel.set_items(items);
    return Ok(channel);
  }
  Err(anyhow::Error::msg("News not found"))
}
