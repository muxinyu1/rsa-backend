#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptReq {
    pub message: String,
    pub public_key: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptRsp {
    pub ciphertext: String,
    pub time_taken: u128,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DecryptReq {
    pub ciphertext: String,
    pub public_key: String,
    pub private_key: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DecryptRsp {
    pub message: String,
    pub time_taken: u128,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Keys {
    pub public_key: String,
    pub private_key: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KeyGenRsp {
    pub keys: Keys,
    pub time_taken: u128,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SignReq {
    pub message: String,
    pub public_key: String,
    pub private_key: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SignRsp {
    pub message_signed: String,
    pub time_taken: u128,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifySignReq {
    pub message: String,
    pub message_signed: String,
    pub public_key: String
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VerifySignRsp {
    pub verified: bool,
    pub time_taken: u128
}