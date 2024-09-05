#![allow(unused)]
use std::mem::size_of;

use super::listener::Package;

#[derive(Debug, PartialEq)]
pub(super) enum PackageType {
  Invalid = -1,
  Good,
  Info,
  Player
}

pub(super) fn analyze_package(package: &Package) -> anyhow::Result<PackageType> {
  if package.len == 0 {
    return Ok(PackageType::Invalid);
  }

  if package.len < 5 {
    return Ok(PackageType::Good);
  }

  let data = package.data;

  let channel = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);

  log::debug!("channel {channel}");

  if channel == -2 {
    return Ok(PackageType::Invalid);
  } else if channel != -1 {
    return Ok(PackageType::Good);
  }

  if package.len < 9 {
    return Ok(PackageType::Invalid)
  }

  let challenge = i32::from_le_bytes(data[5..9].try_into()?);

  log::debug!("challenge {challenge}");

  if challenge == -1 {
    return Ok(PackageType::Good);
  }

  match data[4] as char {
    'T' => Ok(PackageType::Info),
    'U' => Ok(PackageType::Player),
    _ => Ok(PackageType::Good)
  }
}