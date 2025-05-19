use chrono::Datelike;
use rss::Channel;
use scraper::Element;
use super::*;

#[derive(Default)]
pub struct Idwr;

impl Site for Idwr {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let now = chrono::Utc::now();
      let now = now.with_timezone(&chrono_tz::Asia::Tokyo);
      let url = format!("https://id-info.jihs.go.jp/surveillance/idwr/jp/idwr/{}/index.html", now.year());
      let content = reqwest::get(&url)
        .await?
        .text_with_charset("UTF-8")
        .await?;

      let doc = scraper::Html::parse_document(&content);
      let news = build_rss(&url, &doc)?;
      Ok(news.to_string())
    })
  }
}

fn build_rss(url: &str, doc: &scraper::Html) -> anyhow::Result<Channel> {
  let base_url = url::Url::parse(url)?;
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("感染症発生動向調査週報".to_string());
  channel.set_description("平成11年4月1日から施行された感染症の予防及び感染症の患者に対する医療に関する法律（以下「感染症法」という。）に基づき、感染症法に規定された疾患の患者が、全国でどのくらい発生したのかを調査集計しています。".to_string());
  channel.set_copyright("Copyright 1998 National Institute of Infectious Diseases, Japan ".to_string());
  channel.set_link("https://www.niid.go.jp/niid/ja/idwr.html".to_string());
  let mut items = Vec::<rss::Item>::new();
  let link_selector = scraper::Selector::parse("a.sizeview").expect("[BUG] Invalid selector");
  for link_elem in doc.select(&link_selector) {
    let Some(paragraph_elem) = link_elem.parent_element() else {
      continue;
    };
    let Some(title_elem) = paragraph_elem.prev_sibling_element() else {
      continue;
    };
    let Some(link) = link_elem.attr("href").map(ToString::to_string) else {
      continue;
    };
    let link = base_url.join(&link)?;
    let title = title_elem.text().collect::<String>();
    let mut item = rss::Item::default();
    item.set_title(title);
    item.set_link(link.to_string());
    let guid = {
      let mut guid = rss::Guid::default();
      guid.set_value(link.clone());
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
