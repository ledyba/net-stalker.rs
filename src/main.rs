mod sites;

use signal_hook::{consts::SIGINT, consts::SIGTERM, iterator::Signals};
use axum::Extension;

async fn root() -> &'static str {
  "Hello, World!"
}

fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();
  // Prepare signal handling.
  let (tx, rx) = tokio::sync::oneshot::channel::<()>();
  let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
  std::thread::spawn(move || {
    for _sig in signals.forever() {
      tx.send(()).expect("Failed to send signal");
      break;
    }
  });

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

    axum::Server::bind(&"0.0.0.0:3000".parse().expect("[BUG] Failed to parse addr"))
      .serve(app.into_make_service())
      .with_graceful_shutdown(async {
        rx.await.ok();
      })
      .await?;
    Ok(())
  })
}
