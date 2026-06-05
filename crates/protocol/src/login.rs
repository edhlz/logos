use anyhow::{anyhow, Result};
use std::io::{Cursor, Read};

use crate::{
    packet::{write_mc_string, Packet},
    varint::{read_varint_from_reader, write_varint_to_writer},
};

pub const LOGIN_START_ID: i32 = 0x00;
pub const LOGIN_SUCCESS_ID: i32 = 0x02;
pub const SET_COMPRESSION_ID: i32 = 0x03;
pub const DISCONNECT_ID: i32 = 0x00;

pub const OFFLINE_UUID: &str = "00000000-0000-0000-0000-000000000000";

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub username: String,
}

pub fn decode_login_start(data: &[u8]) -> Result<LoginStart> {
    let mut cursor = Cursor::new(data);
    let username = read_mc_string(&mut cursor)?;

    Ok(LoginStart { username })
}

pub fn make_login_success(uuid: &str, username: &str) -> Packet {
    let mut data = Vec::new();

    write_mc_string(uuid, &mut data);
    write_mc_string(username, &mut data);

    Packet::new(LOGIN_SUCCESS_ID, data)
}

pub fn make_set_compression(threshold: i32) -> Packet {
    let mut data = Vec::new();

    write_varint_to_writer(threshold, &mut data).expect("failed to write compression threshold");

    Packet::new(SET_COMPRESSION_ID, data)
}

pub fn make_disconnect(reason_json: &str) -> Packet {
    let mut data = Vec::new();

    write_mc_string(reason_json, &mut data);

    Packet::new(DISCONNECT_ID, data)
}

fn read_mc_string(cursor: &mut Cursor<&[u8]>) -> Result<String> {
    let len = {
        let r: &mut dyn Read = cursor;
        read_varint_from_reader(r).map_err(|e| anyhow!("{:?}", e))?
    };
    let len = len as usize;

    let mut buf = vec![0u8; len];
    cursor.read_exact(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}