use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Yomoyomo;

const BASE_URL: &'static str = "https://wirelesswire.jp/author/yomoyomo/";

impl Site for Yomoyomo {
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
  channel.set_title("yomoyomo - WirelessWire News（ワイヤレスワイヤーニュース）".to_string());
  channel.set_description("雑文書き／翻訳者。".to_string());
  channel.set_copyright("東京都".to_string());
  channel.set_link(BASE_URL.to_string());
  let mut items = Vec::<rss::Item>::new();
  let selector = scraper::Selector::parse("article.block1").expect("[BUG] Invalid selector");
  for elem in doc.select(&selector) {
    let selector = scraper::Selector::parse("p.block1__title a").expect("[BUG] Invalid selector");
    let it = elem.select(&selector).collect::<Vec<_>>();
    if it.is_empty() {
      continue;
    }
    let it = it[0];
    let Some(link) = it.attr("href") else {
      continue;
    };
    let title = it.inner_html();
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
  Ok(channel)
}
