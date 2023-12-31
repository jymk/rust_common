pub fn encrypt<T: AsRef<[u8]>>(input: T) -> String {
    let h = hmac_sha512::Hash::hash(input);
    to_hex(&h)
}

pub fn to_hex<T: AsRef<[u8]>>(input: T) -> String {
    let mut res = String::default();
    for b in input.as_ref() {
        res.push_str(&format!("{:02x}", b));
    }
    res
}

#[test]
fn test_encrypt() {
    use crate::cm_log::log_init;
    log_init(log::LevelFilter::Debug);
    let h = to_hex(&hmac_sha512::Hash::hash("sss"));
    super::debug!("encrypt={}, hlen={}", h, h.len());
}
