import requests
from pydantic import BaseModel

class Keys(BaseModel):
    public_key: str
    private_key: str

class KeyGenRsp(BaseModel):
    keys: Keys
    time_taken: int

class EncryptRsp(BaseModel):
    ciphertext: str
    time_taken: int

class DecryptRsp(BaseModel):
    message: str
    time_taken: int
    
class SignRsp(BaseModel):
    message_signed: str
    time_taken: int

class VerifySignRsp(BaseModel):
    verified: bool
    time_taken: int

sesssion = requests.Session()

message = "哥们你真的逆天啊"

with sesssion.get(url="http://127.0.0.1:8080/keygen/128") as response:
    keygen_rsp = KeyGenRsp(**response.json())
    public_key = keygen_rsp.keys.public_key
    private_key = keygen_rsp.keys.private_key
    print(f"公钥: {public_key}, 私钥: {private_key}")

with sesssion.post(url="http://127.0.0.1:8080/encrypt", json= {
    "message": message,
    "public_key": public_key
}) as response:
    encrypt_rsp = EncryptRsp(**response.json())
    ciphertext = encrypt_rsp.ciphertext
    print(f"\"我真是云深玩家\"加密后: \"{ciphertext}\"")
    
with sesssion.post("http://127.0.0.1:8080/decrypt", json={
    "ciphertext": encrypt_rsp.ciphertext,
    "public_key": public_key,
    "private_key": private_key
}) as response:
    decrypt_rsp = DecryptRsp(**response.json())
    print(f"\"{encrypt_rsp.ciphertext}\"解密后: \"{decrypt_rsp.message}\"")
    
with sesssion.post(url="http://127.0.0.1:8080/sign", json={
    "message": message,
    "public_key": public_key,
    "private_key": private_key
}) as response:
    sign_rsp = SignRsp(**response.json())
    message_signed = sign_rsp.message_signed
    print(f"\"我真是云深玩家\"签名后: \"{message_signed}\"")

with sesssion.post(url="http://127.0.0.1:8080/verify_sign", json={
    "message": message,
    "message_signed": message_signed,
    "public_key": public_key
}) as response:
    verify_sign_rsp = VerifySignRsp(**response.json())
    print(f"验证签名结果: {verify_sign_rsp.verified}")