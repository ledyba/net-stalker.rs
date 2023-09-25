use chrono::Datelike;
use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Idwr {}

impl Site for Idwr {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let now = chrono::Utc::now();
      let now = now.with_timezone(&chrono_tz::Asia::Tokyo);
      let url = format!("https://www.niid.go.jp/niid/ja/idwr-dl/{}.html", now.year());
      let content = reqwest::get(&url)
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
  let selector = scraper::Selector::parse("div.blog div.item").expect("[BUG] Invalid selector");
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("感染症発生動向調査週報".to_string());
  channel.set_description("平成11年4月1日から施行された感染症の予防及び感染症の患者に対する医療に関する法律（以下「感染症法」という。）に基づき、感染症法に規定された疾患の患者が、全国でどのくらい発生したのかを調査集計しています。".to_string());
  channel.set_copyright("Copyright 1998 National Institute of Infectious Diseases, Japan ".to_string());
  channel.set_link("https://www.niid.go.jp/niid/ja/idwr.html".to_string());
  let mut items = Vec::<rss::Item>::new();
  let title_selector = scraper::Selector::parse("p > strong").expect("[BUG] Invalid selector");
  let link_selector = scraper::Selector::parse(".body1 a").expect("[BUG] Invalid selector");
  for elem in doc.select(&selector) {
    let Some(title_elem) = elem.select(&title_selector).next() else {
      continue;
    };
    let title = title_elem.text().collect::<String>();
    let Some(link_elem) = elem.select(&link_selector).next() else {
      continue;
    };
    let link = link_elem.value().attr("href").map(ToString::to_string).unwrap();
    let mut item = rss::Item::default();
    item.set_title(title);
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
