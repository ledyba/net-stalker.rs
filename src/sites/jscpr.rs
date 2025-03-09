use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Jscpr;

const BASE_URL: &'static str = "http://www.jscpr.org/news";

impl Site for Jscpr {
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
  channel.set_title("日本脱カルト協会".to_string());
  channel.set_description("当会は破壊的カルトの諸問題の研究をおこない、その成果を発展・普及させることを目的としたネットワークです。".to_string());
  channel.set_copyright("日本脱カルト協会".to_string());
  channel.set_link(BASE_URL.to_string());
  let mut items = Vec::<rss::Item>::new();
  let selector = scraper::Selector::parse("#contents a").expect("[BUG] Invalid selector");
  for elem in doc.select(&selector) {
    let selector = scraper::Selector::parse("dl > dt").expect("[BUG] Invalid selector");
    let it = elem.select(&selector).collect::<Vec<_>>();
    if it.is_empty() {
      continue;
    }
    let it = it[0];
    let Some(link) = elem.attr("href") else {
      continue;
    };
    let Some(title) = it.first_child() else {
      continue;
    };
    let Some(title) = title.next_sibling() else {
      continue;
    };
    let Some(title) = title.value().as_text() else {
      continue;
    };
    let mut item = rss::Item::default();
    item.set_title(title.text.to_string());
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
