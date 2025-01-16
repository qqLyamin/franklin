from passlib.context import CryptContext
from datetime import datetime, timedelta, timezone
from typing import Optional
import jwt
from config import SECRET_KEY, ALGORITHM, ACCESS_TOKEN_EXPIRE_MINUTES, REFRESH_TOKEN_EXPIRE_DAYS

# Настройка CryptContext для работы с паролями
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

# Функции для работы с паролями
def hash_password(password: str) -> str:
    """Хэширует пароль."""
    return pwd_context.hash(password)

def verify_password(plain_password: str, hashed_password: str) -> bool:
    """Проверяет соответствие введенного пароля хэшированному."""
    if plain_password is None or hashed_password is None:
        return False
    return pwd_context.verify(plain_password, hashed_password)

# Функции для работы с токенами
def create_access_token(data: dict, expires_delta: Optional[timedelta] = None) -> str:
    """Создает access token.

    :param data: Данные для токена.
    :param expires_delta: Время жизни токена. Если None, используется значение из конфигурации.
    :return: Подписанный JWT токен.
    """
    if not isinstance(data, dict):
        raise ValueError("Ошибка при создании access токена")  # Синхронизируем с тестами

    to_encode = data.copy()
    try:
        expire = datetime.now(timezone.utc) + (expires_delta or timedelta(minutes=ACCESS_TOKEN_EXPIRE_MINUTES))
        to_encode.update({"exp": expire})
        encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    except Exception as e:
        raise ValueError(f"Ошибка при создании access токена: {str(e)}")
    return encoded_jwt

def create_refresh_token(data: dict) -> str:
    """Создает refresh token.

    :param data: Данные для токена.
    :return: Подписанный JWT токен.
    """
    
    if not isinstance(data, dict):
        raise ValueError("Данные для токена должны быть словарем.")  # Синхронизируем с тестами
    to_encode = data.copy()
    try:
        expire = datetime.now(timezone.utc) + timedelta(days=REFRESH_TOKEN_EXPIRE_DAYS)
        to_encode.update({"exp": expire})
        encoded_jwt = jwt.encode(to_encode, SECRET_KEY, algorithm=ALGORITHM)
    except Exception as e:
        raise ValueError(f"Ошибка при создании refresh токена: {str(e)}")
    return encoded_jwt


def decode_access_token(token: str) -> dict:
    """Декодирует и проверяет токен.

    :param token: JWT токен.
    :return: Декодированные данные из токена.
    :raises ValueError: Если токен недействителен, истёк или отсутствуют обязательные поля.
    """
    try:
        payload = jwt.decode(token, SECRET_KEY, algorithms=[ALGORITHM])
        # Проверяем наличие обязательных полей
        if "sub" not in payload:
            raise ValueError("Token is missing the 'sub' field")
        if "exp" not in payload:
            raise ValueError("Token is missing the 'exp' field")
        return payload
    except jwt.ExpiredSignatureError:
        raise ValueError("Token has expired")
    except jwt.InvalidTokenError:
        raise ValueError("Invalid token")
    except Exception as e:
        raise ValueError(f"Ошибка при декодировании токена: {str(e)}")

