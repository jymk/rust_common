use std::mem::size_of_val;

use crypto::digest::Digest;

pub fn encrypt(s: &str) -> String {
    let mut hasher = crypto::sha2::Sha512::new();
    hasher.input_str(s);
    hasher.result_str()
}

#[derive(Debug)]
struct Sha512 {
    // 固定hash，前8个质数的平方根的小数部分的前64位
    _h: [u64; 8],
    // 固定常量
    _cst: [u64; 80],
    _len: usize,
    _block: [u8; 2 * SHA384_512_BLOCK_SIZE],
    _tlen: usize,
    _digest: [u8; DIGEST_SIZE],
}

const SHA384_512_BLOCK_SIZE: usize = 1024 / 8;
const DIGEST_SIZE: usize = 512 / 8;

impl Sha512 {
    pub fn new() -> Self {
        let mut this = Self::default();
        this._h = [
            0x6a09e667f3bcc908,
            0xbb67ae8584caa73b,
            0x3c6ef372fe94f82b,
            0xa54ff53a5f1d36f1,
            0x510e527fade682d1,
            0x9b05688c2b3e6c1f,
            0x1f83d9abfb41bd6b,
            0x5be0cd19137e2179,
        ];
        this._cst = [
            0x428a2f98d728ae22,
            0x7137449123ef65cd,
            0xb5c0fbcfec4d3b2f,
            0xe9b5dba58189dbbc,
            0x3956c25bf348b538,
            0x59f111f1b605d019,
            0x923f82a4af194f9b,
            0xab1c5ed5da6d8118,
            0xd807aa98a3030242,
            0x12835b0145706fbe,
            0x243185be4ee4b28c,
            0x550c7dc3d5ffb4e2,
            0x72be5d74f27b896f,
            0x80deb1fe3b1696b1,
            0x9bdc06a725c71235,
            0xc19bf174cf692694,
            0xe49b69c19ef14ad2,
            0xefbe4786384f25e3,
            0x0fc19dc68b8cd5b5,
            0x240ca1cc77ac9c65,
            0x2de92c6f592b0275,
            0x4a7484aa6ea6e483,
            0x5cb0a9dcbd41fbd4,
            0x76f988da831153b5,
            0x983e5152ee66dfab,
            0xa831c66d2db43210,
            0xb00327c898fb213f,
            0xbf597fc7beef0ee4,
            0xc6e00bf33da88fc2,
            0xd5a79147930aa725,
            0x06ca6351e003826f,
            0x142929670a0e6e70,
            0x27b70a8546d22ffc,
            0x2e1b21385c26c926,
            0x4d2c6dfc5ac42aed,
            0x53380d139d95b3df,
            0x650a73548baf63de,
            0x766a0abb3c77b2a8,
            0x81c2c92e47edaee6,
            0x92722c851482353b,
            0xa2bfe8a14cf10364,
            0xa81a664bbc423001,
            0xc24b8b70d0f89791,
            0xc76c51a30654be30,
            0xd192e819d6ef5218,
            0xd69906245565a910,
            0xf40e35855771202a,
            0x106aa07032bbd1b8,
            0x19a4c116b8d2d0c8,
            0x1e376c085141ab53,
            0x2748774cdf8eeb99,
            0x34b0bcb5e19b48a8,
            0x391c0cb3c5c95a63,
            0x4ed8aa4ae3418acb,
            0x5b9cca4f7763e373,
            0x682e6ff3d6b2b8a3,
            0x748f82ee5defb2fc,
            0x78a5636f43172f60,
            0x84c87814a1f0ab72,
            0x8cc702081a6439ec,
            0x90befffa23631e28,
            0xa4506cebde82bde9,
            0xbef9a3f7b2c67915,
            0xc67178f2e372532b,
            0xca273eceea26619c,
            0xd186b8c721c0c207,
            0xeada7dd6cde0eb1e,
            0xf57d4f7fee6ed178,
            0x06f067aa72176fba,
            0x0a637dc5a2c898a6,
            0x113f9804bef90dae,
            0x1b710b35131c471b,
            0x28db77f523047d84,
            0x32caab7b40c72493,
            0x3c9ebe0a15c9bebc,
            0x431d67c49c100d4c,
            0x4cc5d4becb3e42b6,
            0x597f299cfc657e2a,
            0x5fcb6fab3ad6faec,
            0x6c44198c4a475817,
        ];
        this
    }

    pub fn update(&mut self, s: &str) {
        let len = s.len();
        let tmp_len = SHA384_512_BLOCK_SIZE - self._len;
        let mut rem_len = if len < tmp_len { len } else { tmp_len };
        let sv = s.as_bytes();
        _memcpy(&mut self._block, self._len, &sv[0..], rem_len);
        if self._len + len < SHA384_512_BLOCK_SIZE {
            self._len += len;
        }
        let new_len = len - rem_len;
        let block_nb = new_len / SHA384_512_BLOCK_SIZE;
        let shifted_msg = &sv[rem_len..];
        self.transform(&self._block.clone(), 1);
        self.transform(shifted_msg, block_nb);
        rem_len = new_len % SHA384_512_BLOCK_SIZE;
        _memcpy(&mut self._block, 0, &shifted_msg[block_nb << 7..], rem_len);
        self._len = rem_len;
        self._tlen += (block_nb + 1) << 7;
    }

    pub fn enfin(&mut self) {
        let (block_nb, mut pm_len, mut len_b) = (
            1 + if SHA384_512_BLOCK_SIZE - 17 < self._len % SHA384_512_BLOCK_SIZE {
                1
            } else {
                0
            },
            0,
            0,
        );
        len_b = (self._tlen + self._len) << 3;
        pm_len = block_nb << 7;
        for i in 0..pm_len - self._len {
            self._block[self._len + i] = 0;
        }
        self._block[self._len] = 0x80;
        _sha2_unpack32(&mut self._block[pm_len - 4..], len_b);
        self.transform(&self._block.clone(), block_nb);
        for i in 0..8 {
            _sha2_unpack64(&mut self._digest[i << 3..], self._h[i] as usize)
        }
    }

    pub fn res(&self) -> String {
        let mut s = String::default();
        self._digest.iter().fold(String::default(), |mut s, c| {
            s.push_str(&format!("{:02x}", c));
            return s;
        })
    }

    fn transform(&mut self, msg: &[u8], block_nb: usize) {
        let (mut w, mut wv) = ([0u64; 80], [0u64; 8]);
        let (mut t1, mut t2) = (0u64, 0u64);
        let mut sub_block;
        for i in 0..block_nb {
            sub_block = &msg[i << 7..];
            for j in 0..16 {
                _sha2_pack64(sub_block, &mut w[j]);
            }
            for j in 16..80 {
                // w[j] = _sha512_f4(w[j - 2]) + w[j - 7] + _sha512_f3(w[j - 15]) + w[j - 16];
                w[j] = _sha512_f4(w[j - 2])
                    .saturating_add(w[j - 7])
                    .saturating_add(_sha512_f3(w[j - 15]))
                    .saturating_add(w[j - 16]);
            }
            for j in 0..8 {
                wv[j] = self._h[j];
            }
            for j in 0..80 {
                // t1 =
                //     wv[7] + _sha512_f2(wv[4]) + _sha2_ch(wv[4], wv[5], wv[6]) + self._cst[j] + w[j];
                t1 = wv[7]
                    .saturating_add(_sha512_f2(wv[4]))
                    .saturating_add(_sha2_ch(wv[4], wv[5], wv[6]))
                    .saturating_add(self._cst[j])
                    .saturating_add(w[j]);
                t2 = _sha512_f1(wv[0]).saturating_add(_sha2_maj(wv[0], wv[1], wv[2]));
                wv[7] = wv[6];
                wv[6] = wv[5];
                wv[5] = wv[4];
                wv[4] = wv[3].saturating_add(t1);
                wv[3] = wv[2];
                wv[2] = wv[1];
                wv[1] = wv[0];
                wv[0] = t1.saturating_add(t2);
            }
            for j in 0..8 {
                self._h[j] = self._h[j].saturating_add(wv[j]);
            }
        }
    }
}

fn _memcpy(dst: &mut [u8], start: usize, src: &[u8], len: usize) {
    for i in 0..len {
        dst[start + i] = src[i];
    }
}

fn _sha2_unpack32(s: &mut [u8], x: usize) {
    s[3] = _usize_to_u8(x);
    s[2] = _usize_to_u8(x >> 8);
    s[1] = _usize_to_u8(x >> 16);
    s[0] = _usize_to_u8(x >> 24);
}

fn _usize_to_u8(x: usize) -> u8 {
    if x > u8::MAX as usize {
        return 0;
    }
    return x as u8;
}

fn _sha2_unpack64(s: &mut [u8], x: usize) {
    s[7] = _usize_to_u8(x);
    s[6] = _usize_to_u8(x >> 8);
    s[5] = _usize_to_u8(x >> 16);
    s[4] = _usize_to_u8(x >> 24);
    s[3] = _usize_to_u8(x >> 32);
    s[2] = _usize_to_u8(x >> 40);
    s[1] = _usize_to_u8(x >> 48);
    s[0] = _usize_to_u8(x >> 56);
}

fn _sha2_pack64(s: &[u8], x: &mut u64) {
    *x = (s[7] as u64)
        | ((s[6] as u64) << 8)
        | ((s[5] as u64) << 16)
        | ((s[4] as u64) << 24)
        | ((s[3] as u64) << 32)
        | ((s[2] as u64) << 40)
        | ((s[1] as u64) << 48)
        | ((s[0] as u64) << 56);
}

fn _sha2_ch(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}

fn _sha2_maj(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn _sha512_f1(x: u64) -> u64 {
    _sha2_rotater(x, 28) ^ _sha2_rotater(x, 34) ^ _sha2_rotater(x, 39)
}

fn _sha512_f2(x: u64) -> u64 {
    _sha2_rotater(x, 14) ^ _sha2_rotater(x, 18) ^ _sha2_rotater(x, 41)
}

fn _sha512_f3(x: u64) -> u64 {
    _sha2_rotater(x, 1) ^ _sha2_rotater(x, 8) ^ _sha2_shiftr(x, 7)
}

fn _sha512_f4(x: u64) -> u64 {
    _sha2_rotater(x, 19) ^ _sha2_rotater(x, 61) ^ _sha2_shiftr(x, 6)
}

fn _sha2_rotater(x: u64, n: u64) -> u64 {
    (x >> n) | (x << ((size_of_val(&x) << 3) as u64 - n))
}

fn _sha2_shiftr(x: u64, n: u64) -> u64 {
    x >> n
}

impl Default for Sha512 {
    fn default() -> Self {
        Self {
            _h: [0; 8],
            _cst: [0; 80],
            _len: 0,
            _block: [u8::default(); 2 * SHA384_512_BLOCK_SIZE],
            _tlen: 0,
            _digest: [u8::default(); DIGEST_SIZE],
        }
    }
}

#[test]
fn test_encrypt() {
    let s = "sss";
    println!("encrypt={}", encrypt(s));
    let mut sha = Sha512::new();
    sha.update(s);
    sha.enfin();
    println!("sha={:?}", sha.res());
    // println!("c={:?}", 256 as u8);
}
