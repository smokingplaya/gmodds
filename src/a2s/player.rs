use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};

pub fn query() -> Vec<u8> {
  let mut buf = Vec::with_capacity(1024);

  buf.clear();

  let mut cursor = Cursor::new(&mut buf);

  cursor.write_i32::<LittleEndian>(-1).unwrap();
  cursor.write_i8((b'D' as i32).try_into().unwrap()).unwrap();

  cursor.write_u8(0).unwrap();

  buf
}