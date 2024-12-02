mod sites;

use axum::Extension;

async fn root() -> &'static str {
  "Hello, World!"
}

fn main() -> anyhow::Result<()> {
  use tracing_subscriber::util::SubscriberInitExt;
  tracing_subscriber::fmt()
    .with_timer(tracing_subscriber::fmt::time::ChronoLocal::new("%Y/%m/%d %H:%M:%S%.3f".to_string()))
    .with_max_level(tracing::Level::INFO)
    .with_line_number(true)
    .with_file(true)
    .with_writer(std::io::stderr)
    .finish()
    .init();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()?;
  let rt = std::sync::Arc::new(rt);

  let shutdown_notify = {
    use std::sync::Arc;;
    let rt = Arc::clone(&rt);
    let notify = Arc::new(tokio::sync::Notify::new());
    let notify = Arc::clone(&notify);
    let notifier = Arc::clone(&notify);
    ctrlc::set_handler(move || {
      let notify = Arc::clone(&notify);
      rt.spawn(async move {
        notify.notify_waiters();
      });
    })?;
    notifier
  };

  rt.block_on(async {
    use tracing::info;
    use axum::{
      routing::get,
      Router,
    };

    let app = Router::new()
      .route("/", get(root))
      .route("/:name", get(sites::serve))
      .layer(Extension(sites::Service::new()));

    let tcp_listener = tokio::net::TcpListener::bind(("::", 3000)).await.expect("[BUG] Failed to parse addr");
    let socket_addr = tcp_listener.local_addr()?;

    let server =
      axum::serve(tcp_listener, app)
        .with_graceful_shutdown(async move {
          shutdown_notify.notified().await;
        });

    info!("Listening on http://localhost:{}/", socket_addr.port());
    server.await?;
    Ok(())
  })
}
