import pytest
from datetime import timedelta
import jwt
from config import SECRET_KEY, ALGORITHM
from utils import (
    create_access_token, create_refresh_token, decode_access_token,
    hash_password, verify_password
)

# Тесты для работы с токенами
def test_create_access_token():
    """Проверка создания access токена."""
    data = {"sub": "test@example.com"}
    token = create_access_token(data)
    payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    assert payload["sub"] == data["sub"]
    assert "exp" in payload

def test_create_access_token_with_custom_expiration():
    """Проверка создания access токена с пользовательским временем истечения."""
    data = {"sub": "test@example.com"}
    expires_delta = timedelta(minutes=10)
    token = create_access_token(data, expires_delta=expires_delta)
    payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    assert payload["sub"] == data["sub"]
    assert "exp" in payload

def test_create_refresh_token():
    """Проверка создания refresh токена."""
    data = {"sub": "test@example.com"}
    token = create_refresh_token(data)
    payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
    assert payload["sub"] == data["sub"]
    assert "exp" in payload

def test_create_refresh_token_with_exception():
    """Проверка обработки ошибки при создании refresh токена."""
    with pytest.raises(ValueError, match="Данные для токена должны быть словарем."):
        create_refresh_token(data=None)

def test_decode_access_token():
    """Проверка декодирования корректного access токена."""
    data = {"sub": "test@example.com"}
    token = create_access_token(data)
    payload = decode_access_token(token)
    assert payload["sub"] == data["sub"]

def test_expired_access_token():
    """Проверка обработки истекшего access токена."""
    data = {"sub": "test@example.com"}
    token = create_access_token(data, expires_delta=timedelta(seconds=-1))
    with pytest.raises(ValueError, match="Token has expired"):
        decode_access_token(token)

def test_invalid_access_token():
    """Проверка обработки некорректного access токена."""
    with pytest.raises(ValueError, match="Invalid token"):
        decode_access_token("invalid.token.value")

def test_invalid_signature():
    """Проверка обработки токена с некорректной подписью."""
    fake_token = jwt.encode({"sub": "fake@example.com"}, "WRONG_SECRET", algorithm=ALGORITHM)
    with pytest.raises(ValueError, match="Invalid token"):
        decode_access_token(fake_token)

def test_decode_token_without_exp():
    """Проверка токена без срока действия."""
    token = jwt.encode({"sub": "test@example.com"}, SECRET_KEY, algorithm=ALGORITHM)
    with pytest.raises(ValueError, match="Token is missing the 'exp' field"):
        decode_access_token(token)

def test_decode_access_token_invalid_algorithm():
    """Проверка обработки токена с некорректным алгоритмом."""
    fake_token = jwt.encode({"sub": "test@example.com"}, SECRET_KEY, algorithm="HS512")
    with pytest.raises(ValueError, match="Invalid token"):
        decode_access_token(fake_token)

def test_decode_empty_payload():
    """Проверка токена с пустым payload."""
    token = jwt.encode({}, SECRET_KEY, algorithm=ALGORITHM)
    with pytest.raises(ValueError, match="Token is missing the 'sub' field"):
        decode_access_token(token)

# Тесты для работы с паролями
def test_hash_password():
    """Проверка хэширования пароля."""
    password = "securepassword"
    hashed = hash_password(password)
    assert hashed != password

def test_hash_password_empty_string():
    """Проверка хэширования пустой строки."""
    hashed = hash_password("")
    assert hashed != ""

def test_verify_password():
    """Проверка валидации пароля."""
    password = "securepassword"
    hashed = hash_password(password)
    assert verify_password(password, hashed) is True
    assert verify_password("wrongpassword", hashed) is False

def test_verify_password_with_invalid_hash():
    """Проверка валидации с некорректным хэшем."""
    password = "securepassword"
    invalid_hashed = "invalid_hash"
    with pytest.raises(Exception):  # passlib выбрасывает UnknownHashError
        verify_password(password, invalid_hashed)

def test_verify_password_with_none():
    """Проверка валидации пароля против None."""
    password = "securepassword"
    hashed = hash_password(password)
    assert not verify_password(None, hashed)
    assert not verify_password(password, None)

def test_create_access_token_with_error():
    with pytest.raises(ValueError, match="Ошибка при создании access токена"):
        create_access_token(data=None)

def test_create_refresh_token_with_error():
    with pytest.raises(ValueError, match="Данные для токена должны быть словарем."):
        create_refresh_token(data=None)


    """Проверка валидации пароля против None."""
    password = "securepassword"
    hashed = hash_password(password)
    assert not verify_password(None, hashed)
    assert not verify_password(password, None)