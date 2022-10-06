use crypto::digest::Digest;

pub fn encrypt(s: &str) -> String {
    let mut hasher = crypto::sha2::Sha512::new();
    hasher.input_str(s);
    hasher.result_str()
}
