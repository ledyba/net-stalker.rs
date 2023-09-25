mod sites;

use axum::Extension;

async fn root() -> &'static str {
  "Hello, World!"
}

fn main() -> anyhow::Result<()> {
  tracing_subscriber::fmt::init();

  #[cfg(not(windows))]
  // Prepare signal handling.
  let rx = {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    use signal_hook::iterator::Signals;
    let mut signals = Signals::new(signal_hook::consts::TERM_SIGNALS)?;
    std::thread::spawn(move || {
      for _sig in signals.forever() {
        tx.send(()).expect("Failed to send signal");
        break;
      }
    });
    rx
  };

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()?;

  rt.block_on(async {
    use axum::{
      routing::get,
      Router,
    };

    let app = Router::new()
      .route("/", get(root))
      .route("/:name", get(sites::serve))
      .layer(Extension(sites::Service::new()));

    let server =
      axum::Server::bind(&"0.0.0.0:3000".parse().expect("[BUG] Failed to parse addr"))
      .serve(app.into_make_service());

    #[cfg(not(windows))]
    let server = {
      server
        .with_graceful_shutdown(async {
          rx.await.ok();
        });
    };

    server.await?;
    Ok(())
  })
}
