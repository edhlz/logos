use crate::packet::{
    Packet,
    write_mc_string,
};

pub const STATUS_REQUEST_ID: i32 = 0x00;
pub const STATUS_RESPONSE_ID: i32 = 0x00;

pub const PING_ID: i32 = 0x01;
pub const PONG_ID: i32 = 0x01;

pub fn make_status_response() -> Packet {
    let json = r#"{
        "version": {
            "name": "1.21.11",
            "protocol": 774
        },
        "players": {
            "max": 67,
            "online": 100000
        },
        "description": {
            "text": "Test"
        }
    }"#;

    let mut data = Vec::new();

    write_mc_string(json, &mut data);

    Packet::new(STATUS_RESPONSE_ID, data)
}

pub fn make_pong(payload: i64) -> Packet {
    let mut data = Vec::new();

    data.extend_from_slice(&payload.to_be_bytes());

    Packet::new(PONG_ID, data)
}