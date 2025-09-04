use actix_web::{get, web, App, HttpResponse, HttpServer};
use opensearch::{OpenSearch, http::transport::Transport};
use serde::Deserialize;

#[derive(Deserialize)]
struct Q { q: Option<String> }

#[get("/search/{cat}")]
async fn search(path: web::Path<String>, q: web::Query<Q>) -> HttpResponse {
    let cat = path.into_inner(); // apps | leisure | sites
    let term = q.q.clone().unwrap_or_default();

    let transport = Transport::single_node("http://localhost:9200").unwrap();
    let os = OpenSearch::new(transport);

    let query = serde_json::json!({
      "query": {
        "multi_match": {
          "query": term,
          "fields": ["title^3","body","url"]
        }
      },
      "size": 20
    });

    let resp = os.search().index(&cat).body(query).send().await;
    match resp {
      Ok(r) => {
        let txt = r.text().await.unwrap_or_else(|_|"{}".into());
        HttpResponse::Ok().content_type("application/json").body(txt)
      },
      Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(search))
    .bind(("0.0.0.0", 8088))?
    .run().await
}
