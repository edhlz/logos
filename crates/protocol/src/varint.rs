use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum VarIntError {
    Io(io::Error),
    TooBig,
    Incomplete,
}

impl From<io::Error> for VarIntError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

pub fn write_varint(mut value: i32, out: &mut Vec<u8>) {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;

        if value != 0 {
            temp |= 0b1000_0000;
        }

        out.push(temp);

        if value == 0 {
            break;
        }
    }
}

pub fn read_varint_from_slice(input: &[u8]) -> Result<(i32, usize), VarIntError> {
    let mut num_read = 0usize;
    let mut result = 0i32;

    for byte in input.iter().copied() {
        let value = (byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;

        if num_read > 5 {
            return Err(VarIntError::TooBig);
        }

        if (byte & 0b1000_0000) == 0 {
            return Ok((result, num_read));
        }
    }

    Err(VarIntError::Incomplete)
}

pub fn write_varint_to_writer<W: Write>(mut value: i32, mut writer: W) -> Result<(), VarIntError> {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;

        if value != 0 {
            temp |= 0b1000_0000;
        }

        writer.write_all(&[temp])?;

        if value == 0 {
            break;
        }
    }

    Ok(())
}

pub fn read_varint_from_reader<R: Read>(mut reader: R) -> Result<i32, VarIntError> {
    let mut num_read = 0usize;
    let mut result = 0i32;

    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;

        let [byte] = buf;
        let value = (byte & 0b0111_1111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;

        if num_read > 5 {
            return Err(VarIntError::TooBig);
        }

        if (byte & 0b1000_0000) == 0 {
            return Ok(result);
        }
    }
}