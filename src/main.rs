mod sites;

use axum::Extension;
use axum::handler::Handler;

async fn root() -> &'static str {
  "Hello, World!"
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
      .route("/:name", get(sites::serve))
      .layer(Extension(sites::Service::new()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
      .serve(app.into_make_service())
      .await?;
    Ok(())
  })
}
