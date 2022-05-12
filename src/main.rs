mod sites;

use axum::Extension;

async fn root() -> &'static str {
  "Hello, World!"
}

async fn kouan(
  Extension(state): Extension<sites::SharedState>,
) -> axum::response::Result<impl axum::response::IntoResponse> {
  let r = sites::kouan::fetch(state.clone()).await;
  let msg = r.map_err(|it| axum::response::ErrorResponse::from(it.to_string()))?;
  axum::response::Response::builder()
    .header("content-type", "application/rss+xml")
    .body(axum::body::Full::from(msg))
    .map_err(|err| axum::response::ErrorResponse::from(err.to_string()))
}

fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
  rt.block_on(async {
    use axum::{
      routing::get,
      Router,
    };
    let app = Router::new()
      .route("/", get(root))
      .route("/kouan", get(kouan))
      .layer(Extension(sites::SharedState::default()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await?;
    Ok(())
  })
}
