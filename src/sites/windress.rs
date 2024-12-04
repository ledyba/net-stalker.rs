use std::future::Future;
use std::pin::Pin;
use rss::Channel;
use sha2::Digest;

#[derive(Default)]
pub struct Windress;

impl super::Site for Windress {
  fn fetch(&self) -> Pin<Box<dyn Future<Output=anyhow::Result<String>> + Send>> {
    Box::pin(async {
      let cli = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:199.0)")
        .build()?;
      let content = cli.get("https://mahoukoubou.jimdofree.com/")
        .send().await?
        .text_with_charset("UTF-8")
        .await?;
      tracing::info!("{}", &content);

      let doc = scraper::Html::parse_document(&content);
      let news = build_rss(&doc)?;
      Ok(news.to_string())
    })
  }
}

fn build_rss(doc: &scraper::Html) -> anyhow::Result<Channel> {
  let selector = scraper::Selector::parse("#content").expect("[BUG] Invalid selector");
  let mut channel = Channel::default();
  channel.set_language("ja".to_string());
  channel.set_title("まほー工房".to_string());
  channel.set_description("まほー工房は社会生活や日常生活に役立つ魔法やメンタルメソッド、心法などの技法をボイスドラマ形式の音声作品で提供しているサークルです。".to_string());
  channel.set_copyright("まほー工房".to_string());
  channel.set_link("https://mahoukoubou.jimdofree.com/".to_string());
  let mut items = Vec::<rss::Item>::new();
  for elem in doc.select(&selector) {
    let mut item = rss::Item::default();
    item.set_title("まほー工房 更新".to_string());
    item.set_link("https://mahoukoubou.jimdofree.com/".to_string());
    let guid = {
      let mut guid = rss::Guid::default();
      let text = elem.inner_html();
      guid.set_value(format!("{:x}", sha2::Sha256::digest(&text)));
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
