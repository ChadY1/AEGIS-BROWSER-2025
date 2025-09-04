use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use opensearch::{OpenSearch, http::transport::Transport};
use serde::Serialize;
use tokio::time::{sleep, Duration};
use robots_txt::RobotFileParser;

#[derive(Serialize, Debug)]
struct Doc<'a> { url: &'a str, title: String, body: String, cat: &'a str }

async fn index_doc(os: &OpenSearch, index: &str, doc: &Doc<'_>) -> Result<()> {
  let resp = os.index(index).body(serde_json::to_value(doc)?).send().await?;
  let _ = resp.text().await?;
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let client = Client::builder().brotli(true).gzip(true).build()?;
  let transport = Transport::single_node("http://localhost:9200")?;
  let os = OpenSearch::new(transport);

  // seeds simples
  let seeds = vec![
    ("https://example.org", "sites"),
    ("https://example.com", "apps"),
  ];

  let title_sel = Selector::parse("title").unwrap();
  let body_sel = Selector::parse("body").unwrap();

  loop {
    for (u, cat) in &seeds {
      // robots
      let robots_url = format!("{}/robots.txt", u.trim_end_matches('/'));
      let txt = client.get(&robots_url).send().await.ok().and_then(|r| r.text().await.ok()).unwrap_or_default();
      let mut rp = RobotFileParser::default();
      rp.parse(&txt);
      if !rp.can_fetch("*", u) { continue; }

      if let Ok(resp) = client.get(*u).send().await {
        if let Ok(html) = resp.text().await {
          let dom = Html::parse_document(&html);
          let title = dom.select(&title_sel).next().map(|n| n.text().collect::<String>()).unwrap_or_default();
          let body = dom.select(&body_sel).next().map(|n| n.text().collect::<String>()).unwrap_or_default();
          let doc = Doc{ url: u, title, body, cat };
          let index = match *cat { "apps"=>"apps", "leisure"=>"leisure", _=>"sites" };
          let _ = index_doc(&os, index, &doc).await;
        }
      }
    }
    sleep(Duration::from_secs(30)).await;
  }
}
