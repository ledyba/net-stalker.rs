use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Tokushusagi;

const BASE_URL: &'static str = "https://www.kagaiboushi.metro.tokyo.lg.jp/column/";

impl Site for Tokushusagi {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get(BASE_URL)
        .await?
        .text_with_charset("UTF-8")
        .await?;

      let doc = scraper::Html::parse_document(&content);
      let news = build_rss(&doc)?;
      Ok(news.to_string())
    })
  }
}

fn build_rss(doc: &scraper::Html) -> anyhow::Result<Channel> {
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("東京都 特殊詐欺加害防止 特設サイト".to_string());
  channel.set_description("そのバイト、犯罪です。".to_string());
  channel.set_copyright("東京都".to_string());
  channel.set_link("https://www.kagaiboushi.metro.tokyo.lg.jp/".to_string());
  let mut items = Vec::<rss::Item>::new();
  let selector = scraper::Selector::parse(".column-item > a").expect("[BUG] Invalid selector");
  for elem in doc.select(&selector) {
    let Some(link) = elem.value().attr("href") else {
      continue
    };
    let selector = scraper::Selector::parse(".column-content > .column-title").expect("[BUG] Invalid selector");
    let title = elem.select(&selector).next().expect("[BUG] No title");
    let title = title.inner_html();
    let mut item = rss::Item::default();
    item.set_title(title.to_string());
    item.set_link(link.to_string());
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
