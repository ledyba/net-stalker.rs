use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Memo586;

const BASE_URL: &'static str = "https://fesix.sakura.ne.jp/notepad/";

impl Site for Memo586 {
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
  channel.set_title("y586さんのメモ帳".to_string());
  channel.set_description("日記や雑多なメモ、調べたことなどをまとめています。 ".to_string());
  channel.set_copyright("586".to_string());
  channel.set_link(BASE_URL.to_string());
  let mut items = Vec::<rss::Item>::new();
  let selector = scraper::Selector::parse("ul.menu > li.linkitem > a").expect("[BUG] Invalid selector");
  for elem in doc.select(&selector) {
    let Some(link) = elem.attr("href") else {
      continue;
    };
    let title = elem.text().collect::<String>();
    let mut item = rss::Item::default();
    item.set_title(title);
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
