use std::io::{Cursor, Write};
use byteorder::{LittleEndian, WriteBytesExt};

pub fn query() -> Vec<u8> {
  let mut buf = Vec::with_capacity(1024);

  buf.clear();

  let mut cursor = Cursor::new(&mut buf);

  cursor.write_i32::<LittleEndian>(-1).unwrap();
  cursor.write_i8((b'I' as i32).try_into().unwrap()).unwrap();
  cursor.write_u8(DEFAULT_PROTO_VERSION).unwrap();

  let server_info = info();

  cursor.write_all(server_info.game_name.as_bytes()).unwrap();
  cursor.write_all(server_info.map_name.as_bytes()).unwrap();
  cursor.write_all(server_info.game_dir.as_bytes()).unwrap();
  cursor.write_all(server_info.gamemode_name.as_bytes()).unwrap();

  cursor.write_i16::<LittleEndian>(server_info.appid).unwrap();

  cursor.write_i8(server_info.amt_clients).unwrap();
  cursor.write_i8(server_info.max_clients).unwrap();
  cursor.write_i8(server_info.amt_bots).unwrap();
  cursor.write_i8(server_info.server_type).unwrap();
  cursor.write_i8(server_info.os_type).unwrap();
  cursor.write_i8(server_info.passworded).unwrap();

  cursor.write_i8(server_info.secure).unwrap();
  cursor.write_all(server_info.game_version.as_bytes()).unwrap();

  let notags = server_info.tags.is_empty();
  let flags = 0x80u8 as i8 | 0x10 | if notags { 0x00 } else { 0x20 } | 0x01;
  cursor.write_i8(flags).unwrap();
  cursor.write_i16::<LittleEndian>(server_info.udp_port).unwrap();
  cursor.write_u64::<LittleEndian>(server_info.steamid).unwrap();
  if !notags {
      cursor.write_all(server_info.tags.as_bytes()).unwrap();
  }
  cursor.write_i64::<LittleEndian>(server_info.appid.into()).unwrap();

  buf
}

const DEFAULT_PROTO_VERSION: u8 = 17;

fn info() -> Info {
  Info {
    game_name: "Garry's Mod\0".to_string(),
    map_name: "gm_construct\0".to_string(),
    game_dir: "example_dir\0".to_string(),
    gamemode_name: "Sandbox\0".to_string(),
    appid: 720,
    amt_clients: 0,
    max_clients: 20,
    amt_bots: 0,
    server_type: b'd' as i8,
    os_type: (b'w' as i32).try_into().unwrap(), // macos - m; linux - l
    passworded: 0,
    secure: 1,
    game_version: "18.12.05\0".to_string(),
    tags: "pvp\0".to_string(),
    udp_port: 27015,
    steamid: 12345678901234567,
  }
}

struct Info {
  game_name: String,
  map_name: String,
  game_dir: String,
  gamemode_name: String,
  appid: i16,
  amt_clients: i8,
  max_clients: i8,
  amt_bots: i8,
  server_type: i8,
  os_type: i8,
  passworded: i8,
  secure: i8,
  game_version: String,
  tags: String,
  udp_port: i16,
  steamid: u64,
}