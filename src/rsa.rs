use lazy_static::lazy_static;

use crate::algorithms;
use crate::bigint::{mod_div, BigInt, ONE};

const E: u64 = 114493; // biggest prime smaller than 114514;

lazy_static! {
    static ref E_BIGINT: BigInt = BigInt::from_slice(&[E]);
}

fn gen_prime(bit_len: usize) -> BigInt {
    let big_int_len = bit_len / BigInt::VALUE_LEN as usize;
    loop {
        let mut num = BigInt::rand(big_int_len);
        num.value[0] |= 1; // 确保不是偶数

        let (_, r) = mod_div(&num, &E_BIGINT);
        if r.is_zero() {
            continue;
        }
        if algorithms::miller_rabin(&num) {
            return num;
        }
    }
}

pub fn gen_keys(length: usize) -> (BigInt, BigInt) {
    let pq_len = length / 2;
    let p = gen_prime(pq_len);
    let q = gen_prime(pq_len);
    let n = &p * &q;
    let phi_n = &(&p - &ONE) * &(&q - &ONE);
    let barrett_m = phi_n.barrett_m();

    // 手动 gcd 一次，使得数变小到可以放入 u64
    let (div, r) = mod_div(&phi_n, &E_BIGINT);
    let (_, mut u, v) = algorithms::extended_euclid(E, r.to_int().unwrap(), &barrett_m, &phi_n);
    let div_v = algorithms::barrett_mod(&(&v * &div), &barrett_m, &phi_n);
    if u < div_v {
        u = &u + &phi_n;
    }
    let d = algorithms::barrett_mod(&(&u - &div_v), &barrett_m, &phi_n);
    (n, d)
}

pub fn str_to_bigints(input: &str, max_length: usize) -> Vec<BigInt> {
    input
        .to_owned()
        .into_bytes()
        .chunks(max_length * 4)
        .map(|blk| blk.to_vec())
        .map(|block| {
            let value = block
                .chunks(4)
                .map(|blk| blk.to_vec())
                .map(|four_u8s| {
                    four_u8s
                        .into_iter()
                        .enumerate()
                        .fold(0 as u64, |acc, (i, x)| acc + ((x as u64) << (i * 8)))
                })
                .collect::<Vec<_>>();
            let length = value.len();
            BigInt { value, length }
        })
        .collect()
}

pub fn bigints_to_str(xs: Vec<BigInt>) -> String {
    let res = String::from_utf8(
        xs.into_iter()
            .map(|x| {
                x.value[0..x.length]
                    .iter()
                    .map(|v| {
                        let mut res = vec![];
                        for i in 1..=4 {
                            // 取第 i 个 byte，并移到最低位
                            let vv = (v & (((1 as u64) << (i * 8)) - 1)) >> (i - 1) * 8;
                            res.push(vv as u8);
                        }
                        res
                    })
                    .collect::<Vec<_>>()
                    .concat()
            })
            .collect::<Vec<_>>()
            .concat(),
    )
    .expect("utf8 decode failed");
    res.strip_suffix("\0").unwrap_or(&res).to_owned()
}

pub fn encrypt(input: &str, n: &BigInt, barrett_m: &BigInt) -> String {
    str_to_bigints(input, n.length - 1)
        .into_iter()
        .map(|m| algorithms::mod_power(&m, &E_BIGINT, barrett_m, n).fmt_hex())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn decrypt(input: &str, n: &BigInt, barrett_m: &BigInt, d: &BigInt) -> String {
    let ms = input
        .split(",")
        .into_iter()
        .map(|s| {
            let c = BigInt::from_hex(s).expect("Reading hex data failed");
            algorithms::mod_power(&c, &d, &barrett_m, &n)
        })
        .collect();
    bigints_to_str(ms)
}

pub fn sign(input: &str, n: &BigInt, barrett_m: &BigInt, d: &BigInt) -> String {
    str_to_bigints(input, n.length - 1)
        .into_iter()
        .map(|m| algorithms::mod_power(&m, d, barrett_m, n).fmt_hex())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn ver_sign(message: &str, input: &str, n: &BigInt, barrett_m: &BigInt) -> (bool, String) {
    let ms: Vec<BigInt> = input
        .split(",")
        .into_iter()
        .map(|s| {
            let c = BigInt::from_hex(s).expect("Reading hex data failed");
            algorithms::mod_power(&c, &E_BIGINT, &barrett_m, &n)
        })
        .collect();
    let m = bigints_to_str(ms);
    for ch in message.as_bytes() {
        print!("{} ", ch);
    }
    println!();
    for ch in m.as_bytes() {
        print!("{} ", ch);
    }
    println!();
    println!("message = {}, m = {}", message, m);
    (m.trim_end_matches("\0") == message, m)
}
