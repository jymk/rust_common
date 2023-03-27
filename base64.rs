use std::{fs::File, io::Read};

use crate::errs::{SError, SResult};

/// 文件转u8 vec
pub fn file_to_u8s(path: &str) -> SResult<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::default();
    file.read_to_end(&mut buf).unwrap();
    Ok(buf)
}

//编码表
const ENCODE_TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const MASK_EN: usize = 0x3F;
const MASK_DE: u8 = 0xFF;

/// 编码，不带data:image/png;base64,头
pub fn base64_encode(data: &[u8]) -> String {
    let mut res = String::default();

    let mut line_len = 0;
    let mut buf = [0u8; 4];
    let mut i = 0;
    while i < data.len() / 3 {
        buf[1] = data[i * 3];
        buf[2] = data[i * 3 + 1];
        buf[3] = data[i * 3 + 2];
        res.push(ENCODE_TABLE[buf[1] as usize >> 2] as char);
        res.push(
            ENCODE_TABLE[(((buf[1] as usize) << 4) | ((buf[2] as usize) >> 4)) & MASK_EN] as char,
        );
        res.push(
            ENCODE_TABLE[(((buf[2] as usize) << 2) | ((buf[3] as usize) >> 6)) & MASK_EN] as char,
        );
        res.push(ENCODE_TABLE[buf[3] as usize & MASK_EN] as char);
        line_len += 4;
        if line_len == 76 {
            res.push_str("\r\n");
            line_len = 0;
        }
        i += 1;
    }
    //对剩余数据进行编码
    let mold: usize = data.len() % 3;
    match mold {
        1 => {
            buf[1] = data[i * 3];
            res.push(ENCODE_TABLE[((buf[1] & 0xFC) as usize) >> 2] as char);
            res.push(ENCODE_TABLE[((buf[1] & 0x03) as usize) << 4] as char);
            res.push_str("==");
        }
        2 => {
            buf[1] = data[i * 3];
            buf[2] = data[i * 3 + 1];
            res.push(ENCODE_TABLE[((buf[1] & 0xFC) as usize) >> 2] as char);
            res.push(
                ENCODE_TABLE[(((buf[1] & 0x03) as usize) << 4) | (((buf[2] & 0xF0) as usize) >> 4)]
                    as char,
            );
            res.push(ENCODE_TABLE[((buf[2] & 0x0F) as usize) << 2] as char);
            res.push('=');
        }
        _ => {}
    }
    res
}

/// 文件转base64
pub fn file_to_base64(path: &str) -> SResult<String> {
    // 读取文件内容为Vec<u8>
    let u8_vec = file_to_u8s(path)?;
    // 将[u8]转为base64
    Ok(base64_encode(&u8_vec))
}

/// base64解码
pub fn base64_decode(data: &[u8]) -> SResult<Vec<u8>> {
    let mut res = Vec::default();

    let len = data.len();
    if len == 0 {
        return Ok(res);
    }

    if len % 4 != 0 {
        return SError::from(format!(
            "illegal base64, excepted a multiple of 4, its' len is {}",
            len
        ))
        .to_sresult();
    }

    let mut buf = [0u8; 4];

    let mut to_int = [0; 128];
    for i in 0..64 {
        to_int[ENCODE_TABLE[i] as usize] = i as u8;
    }

    for i in (0..len - 2).step_by(4) {
        buf[0] = to_int[data[i] as usize];
        buf[1] = to_int[data[i + 1] as usize];
        res.push((buf[0] << 2 | buf[1] >> 4) & MASK_DE);

        buf[2] = to_int[data[i + 2] as usize];
        res.push((buf[1] << 4 | buf[2] >> 2) & MASK_DE);

        buf[3] = to_int[data[i + 3] as usize];
        res.push((buf[2] << 6 | buf[3]) & MASK_DE);
    }

    Ok(res)
}

#[test]
fn test_base64() {
    let res = base64_encode("你好啊".as_bytes());
    println!("res={}", res);
    let res = base64_decode(res.as_bytes());
    println!("res={:?}", String::from_utf8_lossy(&res.unwrap()));
    // 5L2g5aW95ZWK
    println!("res={:?}", &base64_decode(b"5L2g5aW95ZW").unwrap_err());
}
