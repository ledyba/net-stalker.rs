use std::future::Future;
use std::pin::Pin;
use rss::Channel;

#[derive(Default)]
pub struct AistShisan;

impl super::Site for AistShisan {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let content = reqwest::get("https://www.aist.go.jp/aist_j/procure/asset/jyuyou.html")
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
  let selector = scraper::Selector::parse("#infoCMScontents_14139 > table > tbody > tr").expect("[BUG] Invalid selector");
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("AIST 取得した資産の需要調査 調査中の案件".to_string());
  channel.set_description("産業技術総合研究所では、取得した資産の処分の検討にあたって、需要調査をおこなっております。".to_string());
  channel.set_copyright("Copyright © National Institute of Advanced Industrial Science and Technology （AIST）".to_string());
  channel.set_link("https://www.aist.go.jp/aist_j/procure/asset/jyuyou.html".to_string());
  let mut items = Vec::<rss::Item>::new();
  for elem in doc.select(&selector) {
    let selector = scraper::Selector::parse("td").unwrap();
    let it = elem.select(&selector).collect::<Vec<_>>();
    if it.is_empty() {
      continue;
    }
    let link = "https://www.aist.go.jp/aist_j/procure/asset/jyuyou.html";
    let title = format!("{} / {} / {}",
                       &it[1].inner_html(),
                       &it[2].inner_html(),
                       &it[3].inner_html(),
    );
    let mut item = rss::Item::default();
    item.set_title(title);
    item.set_link(link.to_string());
    let guid = {
      let mut guid = rss::Guid::default();
      guid.set_value(it[0].inner_html());
      guid
    };
    item.set_guid(guid);
    items.push(item);
  }
  channel.set_items(items);
  if items.is_empty() {
    return Err(anyhow::Error::msg("No items!"));
  }
  Ok(channel)
}
