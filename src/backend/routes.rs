use std::time::SystemTime;

use crate::{bigint::BigInt, rsa};
use rocket::{get, post, serde::json::Json};

use super::models::*;

#[get("/keygen/<len>")]
pub fn key_gen(len: usize) -> Json<KeyGenRsp> {
    let start = SystemTime::now();
    let (public_key, private_key) = rsa::gen_keys(len);
    let end = SystemTime::now();
    Json(KeyGenRsp {
        keys: Keys {
            public_key: public_key.fmt_hex(),
            private_key: private_key.fmt_hex(),
        },
        time_taken: end.duration_since(start).unwrap().as_millis(),
    })
}

#[post("/encrypt", data = "<encrypt_req>")]
pub fn encrypt(encrypt_req: Json<EncryptReq>) -> Json<EncryptRsp> {
    let start = SystemTime::now();
    let public_key = BigInt::from_hex(&encrypt_req.public_key).unwrap();
    let m = public_key.barrett_m();
    let ciphertext = rsa::encrypt(&encrypt_req.message, &public_key, &m);
    let end = SystemTime::now();
    Json(EncryptRsp {
        ciphertext,
        time_taken: end.duration_since(start).unwrap().as_millis(),
    })
}

#[post("/decrypt", data = "<decrypt_req>")]
pub fn decrypt(decrypt_req: Json<DecryptReq>) -> Json<DecryptRsp> {
    let start = SystemTime::now();
    let public_key = BigInt::from_hex(&decrypt_req.public_key).unwrap();
    let m = public_key.barrett_m();
    let private_key = BigInt::from_hex(&decrypt_req.private_key).unwrap();
    let message = rsa::decrypt(&decrypt_req.ciphertext, &public_key, &m, &private_key);
    let end = SystemTime::now();
    Json(DecryptRsp {
        message,
        time_taken: end.duration_since(start).unwrap().as_millis(),
    })
}

#[post("/sign", data = "<sign_req>")]
pub fn sign(sign_req: Json<SignReq>) -> Json<SignRsp> {
    let start = SystemTime::now();
    let public_key = BigInt::from_hex(&sign_req.public_key).unwrap();
    let m = public_key.barrett_m();
    let private_key = BigInt::from_hex(&sign_req.private_key).unwrap();
    let message_signed = rsa::sign(&sign_req.message, &public_key, &m, &private_key);
    let end = SystemTime::now();
    Json(SignRsp {
        message_signed: message_signed,
        time_taken: end.duration_since(start).unwrap().as_millis(),
    })
}

#[post("/verify_sign", data = "<verify_sign_req>")]
pub fn verify_sign(verify_sign_req: Json<VerifySignReq>) -> Json<VerifySignRsp> {
    let start = SystemTime::now();
    let public_key = BigInt::from_hex(&verify_sign_req.public_key).unwrap();
    let m = public_key.barrett_m();
    let (verified, _) = rsa::ver_sign(
        &verify_sign_req.message,
        &verify_sign_req.message_signed,
        &public_key,
        &m,
    );
    let end = SystemTime::now();
    Json(VerifySignRsp {
        verified,
        time_taken: end.duration_since(start).unwrap().as_millis(),
    })
}
