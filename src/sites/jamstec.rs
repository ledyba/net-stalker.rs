use rss::Channel;
use super::*;

#[derive(Default)]
pub struct Jamstec;

impl Site for Jamstec {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://www.jamstec.go.jp/j/about/press_release/")
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
  let selector = scraper::Selector::parse("div.ly_contentCard_inner ul.page_newsroom_newsLinkWrapper a").expect("[BUG] Invalid selector");
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("JAMSTEC | 海洋研究開発機構".to_string());
  channel.set_description("海洋研究開発機構（JAMSTEC ジャムステック）は、平和と福祉の理念に基づき、海洋に関する基盤的研究開発、海洋に関する学術研究に関する協力等の業務を総合的に行うことにより海洋科学技術の水準の向上を図るとともに、学術研究の発展に資することを目的とした組織です。".to_string());
  channel.set_copyright("© JAMSTEC, www.jamstec.go.jp".to_string());
  channel.set_link("https://www.jamstec.go.jp/j/about/press_release/".to_string());
  let mut items = Vec::<rss::Item>::new();
  for elem in doc.select(&selector) {
    let link = elem.value().attr("href").map(ToString::to_string).unwrap();
    let link = format!("https://www.jamstec.go.jp{}", link);
    let selector = scraper::Selector::parse("div.nr_newsLink_body div.nr_newsLink_text").unwrap();
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
  if items.is_empty() {
    return Err(anyhow::Error::msg("No items!"));
  }
  channel.set_items(items);
  Ok(channel)
}
