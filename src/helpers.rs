//! Crate helpers

pub fn crc16_ex(buf: &Vec<u8>, start: usize, count: usize) -> Vec<u8> {
    let mut crc: u16 = 0xFFFF;

    for pos in start..start + count {
        crc ^= buf[pos] as u16 & 0x00FF_u16;   // XOR byte into least sig. byte of crc

        for _ in 0..8 {    // Loop over each bit
            if (crc & 0x0001) != 0 {      // If the LSB is set
                crc >>= 1;                    // Shift right and XOR 0x8408
                crc ^= 0x8408;
            } else {                           // Else LSB is not set
                crc >>= 1;                    // Just shift right
            }
        }
    }

    vec![((crc ^ 0xFFFF_u16) & 0x00FF_u16) as u8, 
    (((crc ^ 0xFFFF_u16) >> 8) & 0x00FF_u16) as u8]
}

pub fn crc16(buf: &Vec<u8>) -> Vec<u8> {
    let buf_len = buf.len();
    crc16_ex(buf, 0, buf_len)
}

pub fn byte_stuff(data: &Vec<u8>) -> Vec<u8> {
    let mut stuffed_data: Vec<u8> = vec![];
    for data_byte in data {
        if (data_byte & 0xFF) < 0xFD {
            stuffed_data.push(*data_byte);
        } else {
            stuffed_data.push(0xFF);
            stuffed_data.push(0xFF - (data_byte & 0xFF));
        }
    }
    return stuffed_data;
}

pub fn unbyte_stuff(stuffed_data: &Vec<u8>) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];
    let mut invert_next = false;
    for data_byte in stuffed_data {
        if (data_byte & 0xFF) == 0xFF {
            invert_next = true;
            continue;
        }
        if invert_next {
            data.push(0xFF - (data_byte & 0xFF));
            invert_next = false;
        } else {
            data.push(data_byte & 0xFF);
        }
    }
    return data;
}
