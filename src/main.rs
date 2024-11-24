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

  #[cfg(not(windows))]
  // Prepare signal handling.
  let rx = {
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    use signal_hook::iterator::Signals;
    let mut signals = Signals::new(signal_hook::consts::TERM_SIGNALS)?;
    std::thread::spawn(move || {
      for _sig in signals.forever() {
        tx.send(()).expect("[BUG] Failed to send signal");
        break;
      }
    });
    rx
  };

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()?;

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

    let tcp_listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await.expect("[BUG] Failed to parse addr");
    let socket_addr = tcp_listener.local_addr()?;

    let server = axum::serve(tcp_listener, app);

    #[cfg(not(windows))]
    let server = {
      let fut = async {
        rx.await.expect("[BUG] Failed to recv signal.");
      };
      server.with_graceful_shutdown(fut)
    };

    info!("Listening on http://localhost:{}/", socket_addr.port());
    server.await?;
    Ok(())
  })
}
