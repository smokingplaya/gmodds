use chrono::Utc;
use env_logger::Builder;
use log::{Level, LevelFilter};
use std::io::Write;

pub fn setup() {
  Builder::new()
  .format(|buf, record| {
      let now = Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ");
      let level = record.level();
      let color = match level {
          Level::Error => "\x1b[31m",
          Level::Warn => "\x1b[33m",
          Level::Info => "\x1b[32m",
          Level::Debug => "\x1b[36m",
          Level::Trace => "\x1b[35m",
      };

      writeln!(
        buf,
        "\x1b[0m[{now} {color}{level}\x1b[0m] {}",
        record.args()
    )
  })
  .filter_level(LevelFilter::max())
  .init();
}