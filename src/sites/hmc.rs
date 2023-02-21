use rss::Channel;
use super::*;

#[derive(Default)]
pub struct HMC {}

impl Site for HMC {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://hmc.u-tokyo.ac.jp/ja/open-seminar/")
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
  let selector = scraper::Selector::parse("div.list article.item a").unwrap();
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("東京大学 ヒューマニティーズセンター オープンセミナー".to_string());
  channel.set_description("「公募研究」の採択者を中心にして「オープンセミナー」を一般公開形式で開催することになりました。参加料は無料です。奮ってご参加ください。".to_string());
  channel.set_copyright(" © 2017 The University of Tokyo Humanities Center ".to_string());
  channel.set_link("https://hmc.u-tokyo.ac.jp/ja/open-seminar/".to_string());
  let mut items = Vec::<rss::Item>::new();
  for elem in doc.select(&selector) {
    let link = elem.value().attr("href").map(ToString::to_string).unwrap();
    let selector = scraper::Selector::parse("div.title > strong").unwrap();
    let elem = if let Some(elem) = elem.select(&selector).next() {
      elem
    } else {
      continue;
    };
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
