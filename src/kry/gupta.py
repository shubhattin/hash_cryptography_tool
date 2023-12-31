import hashlib, uuid, hashlib, base64
from cryptography.fernet import Fernet
import bcrypt


def get_type(fg):
    return str(type(fg))[8:-2]


def from_base64(v):
    return base64.b64decode(v).decode("utf-8")


def to_base64(v):
    return base64.b64encode(bytes(v, "utf-8")).decode("utf-8")


def bin_str(val):
    if get_type(val) == "bytes":
        return val
    else:
        return bytes(val, "utf-8")


def salt() -> str:
    return uuid.uuid4().hex


def sha3_256(val) -> str:
    return hashlib.sha3_256(bin_str(val)).hexdigest()


def sha3_512(val) -> str:
    return hashlib.sha3_512(bin_str(val)).hexdigest()


def sha_256(val) -> str:
    return hashlib.sha256(bin_str(val)).hexdigest()


def sha_512(val) -> str:
    return hashlib.sha512(bin_str(val)).hexdigest()


def md5(val) -> str:
    return hashlib.md5(bin_str(val)).hexdigest()


def bcrypt_verify(val, hash):
    return bcrypt.checkpw(bin_str(val), bin_str(hash))


def bcrypt_hash(val):
    return bcrypt.hashpw(bin_str(val), bcrypt.gensalt()).decode("utf-8")


def bcrypt_salt():
    return bcrypt.gensalt().decode("utf-8")


# Encrypt/Decrypt


def encrypt_text(val, key: str):
    key = to_base64(md5(key))
    fernet = Fernet(key)
    return fernet.encrypt(bin_str(val)).decode("utf-8")


def decrypt_text(val, key: str):
    key = to_base64(md5(key))
    fernet = Fernet(key)
    return fernet.decrypt(bin_str(val)).decode("utf-8")
