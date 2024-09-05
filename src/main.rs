use std::time::Instant;

mod a2s;
mod logger;
mod udp;
mod concommand;
mod commandline;

struct Settings {
  pub io_enabled: bool,
}

#[tokio::main]
async fn main() {
  logger::setup();

  // todo
  let settings = Settings {
    io_enabled: true
  };

  let time = Instant::now();

  match udp::listener::setup() {
    Ok(_) => {},
    Err(err) => {
      log::error!("Unable to start GmodDS: {}", err.to_string());
      return;
    },
  }

  log::info!("🔥 GmodDS successfully loaded (elapsed {:.2?})", time.elapsed());

  if settings.io_enabled {
    let _ = commandline::io::setup();
  } else {
    loop {}
  }
}