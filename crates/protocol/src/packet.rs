use crate::varint::{
    read_varint_from_reader,
    write_varint_to_writer,
};

use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Packet {
    pub id: i32,
    pub data: Vec<u8>,
}

pub fn write_mc_string(
    value: &str,
    out: &mut Vec<u8>,
) {
    write_varint_to_writer(
        value.len() as i32,
        &mut *out,
    )
    .expect("failed to write string length");

    out.extend_from_slice(value.as_bytes());
}

impl Packet {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut body = Vec::new();

        write_varint_to_writer(self.id, &mut body)
            .expect("failed to write packet id");

        body.extend_from_slice(&self.data);

        let mut packet = Vec::new();

        write_varint_to_writer(body.len() as i32, &mut packet)
            .expect("failed to write packet length");

        packet.extend_from_slice(&body);

        packet
    }

    pub fn decode(bytes: &[u8]) -> anyhow::Result<Self> {
        let mut cursor = Cursor::new(bytes);

        let id = read_varint_from_reader(&mut cursor)
            .map_err(|e| anyhow::anyhow!("{:?}", e))?;

        let mut data = Vec::new();
        cursor.read_to_end(&mut data)?;

        Ok(Self { id, data })
    }
}

