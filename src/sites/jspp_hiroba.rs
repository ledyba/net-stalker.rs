use rss::Channel;
use super::*;

#[derive(Default)]
pub struct JsppHiroba {}

impl Site for JsppHiroba {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://jspp.org/hiroba/q_and_a/")
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

const BASE_URL: &'static str = "https://jspp.org/";

fn build_rss(doc: &scraper::Html) -> Option<Channel> {
  let selector = scraper::Selector::parse(".left > table > tbody").expect("[BUG] Invalid selector");
  let mut selected = doc.select(&selector);
  if let Some(elem) = selected.next() {
    let mut channel = Channel::default();
    channel.set_language("ja".to_string());
    channel.set_title("一般社団法人日本植物生理学会 植物Q&A".to_string());
    channel.set_description("本質問コーナーは、日本植物生理学会の広報委員会が運営しています。「植物のふしぎ」に関するご質問に、サイエンスアドバイザーや日本植物生理学会の会員を中心とした植物科学の研究者がボランティアでお答えしています。".to_string());
    channel.set_copyright("一般社団法人日本植物生理学会".to_string());
    channel.set_link("https://jspp.org/hiroba/q_and_a/".to_string());
    let mut items = Vec::<rss::Item>::new();
    let selector = scraper::Selector::parse("a").expect("[BUG] Invalid selector");
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
    channel.set_items(items);
    return Some(channel);
  }
  None
}
