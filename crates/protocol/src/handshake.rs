use anyhow::{anyhow, Result};
use std::io::{Cursor, Read};

use crate::varint::read_varint_from_reader;

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

pub fn decode_handshake(data: &[u8]) -> Result<Handshake> {
    let mut cursor = Cursor::new(data);

    let protocol_version =
        read_varint_from_reader(&mut cursor).map_err(|e| anyhow!("{:?}", e))?;

    let server_address = read_mc_string(&mut cursor)?;

    let mut port_buf = [0u8; 2];
    cursor.read_exact(&mut port_buf)?;
    let server_port = u16::from_be_bytes(port_buf);

    let next_state =
        read_varint_from_reader(&mut cursor).map_err(|e| anyhow!("{:?}", e))?;

    Ok(Handshake {
        protocol_version,
        server_address,
        server_port,
        next_state,
    })
}

fn read_mc_string(cursor: &mut Cursor<&[u8]>) -> Result<String> {
    let len = read_varint_from_reader(&mut *cursor).map_err(|e| anyhow!("{:?}", e))?;
    let len = len as usize;

    let mut buf = vec![0u8; len];
    cursor.read_exact(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}