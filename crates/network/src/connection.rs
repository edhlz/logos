use anyhow::{anyhow, Result};
use protocol::{
    handshake::decode_handshake,
    login::{decode_login_start, make_login_success, OFFLINE_UUID},
    packet::Packet,
    status::{make_pong, make_status_response, PING_ID, STATUS_REQUEST_ID},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{info};

async fn read_varint_async(stream: &mut TcpStream) -> Result<i32> {
    let mut num_read = 0usize;
    let mut result = 0i32;

    loop {
        let byte = stream.read_u8().await?;
        let value = (byte & 0x7F) as i32;
        result |= value << (7 * num_read);

        num_read += 1;

        if num_read > 5 {
            return Err(anyhow!("VarInt is too big"));
        }

        if (byte & 0x80) == 0 {
            return Ok(result);
        }
    }
}

async fn read_packet(stream: &mut TcpStream) -> Result<Packet> {
    let length = read_varint_async(stream).await?;
    let mut buf = vec![0u8; length as usize];
    stream.read_exact(&mut buf).await?;
    Packet::decode(&buf)
}

async fn write_packet(stream: &mut TcpStream, packet: Packet) -> Result<()> {
    let bytes = packet.encode();
    stream.write_all(&bytes).await?;
    Ok(())
}

pub async fn handle_client(mut stream: TcpStream) -> Result<()> {
    let first_packet = read_packet(&mut stream).await?;
    if first_packet.id != 0x00 {
        return Err(anyhow!("expected handshake packet id 0x00"));
    }

    let handshake = decode_handshake(&first_packet.data)?;
    info!("handshake: {:?}", handshake);

    match handshake.next_state {
        1 => handle_status(&mut stream).await?,
        2 => handle_login(&mut stream).await?,
        other => {
            return Err(anyhow!("unknown next state: {}", other));
        }
    }

    Ok(())
}

async fn handle_status(stream: &mut TcpStream) -> Result<()> {
    let request = read_packet(stream).await?;
    if request.id != STATUS_REQUEST_ID {
        return Err(anyhow!("expected status request packet"));
    }

    let response = make_status_response();
    write_packet(stream, response).await?;

    let ping = read_packet(stream).await?;
    if ping.id != PING_ID {
        return Err(anyhow!("expected ping packet"));
    }

    if ping.data.len() != 8 {
        return Err(anyhow!("ping payload must be 8 bytes"));
    }

    let payload = i64::from_be_bytes(ping.data.as_slice().try_into().unwrap());
    let pong = make_pong(payload);
    write_packet(stream, pong).await?;

    Ok(())
}

async fn handle_login(stream: &mut TcpStream) -> Result<()> {
    let login_packet = read_packet(stream).await?;
    if login_packet.id != 0x00 {
        return Err(anyhow!("expected login start packet id 0x00"));
    }

    let login_start = decode_login_start(&login_packet.data)?;
    info!("player logging in: {}", login_start.username);

    let success = make_login_success(OFFLINE_UUID, &login_start.username);
    write_packet(stream, success).await?;

    Ok(())
}