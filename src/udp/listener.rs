use tokio::net::UdpSocket;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::{process, thread};

use crate::a2s;
use crate::udp::analyze::{self, PackageType};

use lazy_static::lazy_static;

lazy_static! {
  pub static ref SOCKET_ADDR: Mutex<Option<&'static str>> = Mutex::new(None);
}

#[allow(unused)]
#[derive(Debug)]
pub(super) struct Package<'a> {
  pub len: usize,
  pub addr: SocketAddr,
  pub data: &'a [u8]
}

pub fn setup() -> Result<(), Box<dyn Error + Send + Sync>> {
  thread::spawn(move || {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
      if let Err(e) = udp_listener().await {
        log::error!("UdpListener error: {}", e);
        process::exit(1);
      }
    });
  });

  Ok(())
}

async fn udp_listener() -> Result<(), Box<dyn Error + Send + Sync>> {
  let ip = "0.0.0.0:27015";
  let socket = UdpSocket::bind(ip).await?;
  log::info!("UdpListener binded to {ip}");

  let mut buf = [0; 1024];

  loop {
    let (len, addr) = socket.recv_from(&mut buf).await?;
    log::debug!("Received {} bytes from {}", len, addr);
    #[allow(unused)]
    let package = Package {
      len,
      addr,
      data: &buf[..len]
    };

    log::debug!("Received data: {:.02X?} (hexed)", &package.data);

    let class = match analyze::analyze_package(&package) {
      Ok(pkg) => pkg,
      Err(_) => {
        log::error!("Received broken package from {}", package.addr);
        PackageType::Good
      }
    };

    if class == PackageType::Good {
      return Ok(());
    }

    let response = match class {
      PackageType::Info => a2s::info::query(),
      PackageType::Player => a2s::player::query(),
      _ => package.data.into()
    };

    socket.send_to(response.as_slice(), &addr).await?;
  }
}
