# Стандартные библиотеки
from typing import List

# Сторонние библиотеки
from fastapi.security import OAuth2PasswordRequestForm, OAuth2PasswordBearer
from fastapi import FastAPI, Depends, HTTPException
from pydantic import BaseModel
from sqlalchemy.orm import Session
from contextlib import asynccontextmanager
from fastapi.middleware.cors import CORSMiddleware

# Локальные модули
from config import DATABASE_URL
from database import SessionLocal, engine
from models import Base, User
from schemas import UserCreate, User as UserSchema
from crud import create_user, get_user_by_email
from utils import hash_password, verify_password, create_access_token, create_refresh_token, decode_access_token

# ---- Инициализация ----
app = FastAPI()
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="token")
print("Сервер запущен и готов принимать запросы!")

# Создание таблиц в базе данных при запускеНа
Base.metadata.create_all(bind=engine)

class RefreshTokenRequest(BaseModel):
    refresh_token: str

# ---- Зависимости ----
def get_db():
    """Генератор сессии базы данных."""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://127.0.0.1:5500"],  # Замените "*" на конкретный домен в продакшене
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# ---- Маршруты ----
@app.get("/")
def read_root():
    """Проверка работы приложения."""
    return {"message": "Welcome to the Skill Exchange Platform!"}

@app.post("/users/", response_model=UserSchema)
def create_new_user(user: UserCreate, db: Session = Depends(get_db)):
    """
    Создание нового пользователя.
    """
    if get_user_by_email(db, email=user.email):
        raise HTTPException(status_code=400, detail="User already exists")
    return create_user(db=db, name=user.name, email=user.email, password=user.password)

@app.get("/users/", response_model=List[UserSchema])
def read_users(skip: int = 0, limit: int = 10, db: Session = Depends(get_db)):
    """
    Получение списка пользователей с пагинацией.
    """
    return db.query(User).offset(skip).limit(limit).all()

@app.delete("/users/{user_id}", response_model=dict)
def delete_user(user_id: int, db: Session = Depends(get_db)):
    """
    Удаление пользователя по ID.
    """
    db_user = db.query(User).filter(User.id == user_id).first()
    if not db_user:
        raise HTTPException(status_code=404, detail="User not found")
    db.delete(db_user)
    db.commit()
    return {"message": f"User with id {user_id} was successfully deleted"}

class UserUpdate(BaseModel):
    """
    Модель для обновления пользователя.
    """
    skills: str | None = None
    interests: str | None = None

@app.put("/users/update", response_model=UserSchema)
def update_user_profile(user_update: UserUpdate, token: str = Depends(oauth2_scheme), db: Session = Depends(get_db)):
    """
    Обновление профиля пользователя.
    """
    payload = decode_access_token(token)
    email = payload.get("sub")
    if not email:
        raise HTTPException(status_code=400, detail="Invalid token payload")
    
    db_user = db.query(User).filter(User.email == email).first()
    if not db_user:
        raise HTTPException(status_code=404, detail="User not found")

    if user_update.skills:
        db_user.skills = user_update.skills
    if user_update.interests:
        db_user.interests = user_update.interests
    
    db.commit()
    db.refresh(db_user)
    return db_user


@app.get("/users/{user_email}", response_model=UserSchema)
def get_user(user_email: str, db: Session = Depends(get_db)):
    """
    Получение деталей пользователя по email.
    """
    db_user = db.query(User).filter(User.email == user_email).first()
    if not db_user:
        raise HTTPException(status_code=404, detail="User not found")
    return db_user

@app.get("/users/by-name/{user_name}", response_model=UserSchema)
def get_user_by_name(user_name: str, db: Session = Depends(get_db)):
    """
    Получение деталей пользователя по имени.
    """
    print(f"Получен запрос для имени пользователя: {user_name}")  # Логирование
    db_user = db.query(User).filter(User.name == user_name.strip()).first()
    if not db_user:
        raise HTTPException(status_code=404, detail="User not found")
    return db_user

@app.get("/users/filter/", response_model=List[UserSchema])
def filter_users(skills: str | None = None, interests: str | None = None, db: Session = Depends(get_db)):
    """
    Фильтрация пользователей по навыкам и интересам.
    """
    query = db.query(User)
    if skills:
        query = query.filter(User.skills.ilike(f"%{skills}%"))
    if interests:
        query = query.filter(User.interests.ilike(f"%{interests}%"))
    return query.all()

@app.post("/register/")
def register_user(user: UserCreate, db: Session = Depends(get_db)):
    """
    Регистрация нового пользователя.
    """
    if get_user_by_email(db, email=user.email):
        raise HTTPException(status_code=400, detail="Email already registered")
    return create_user(db=db, name=user.name, email=user.email, password=user.password)

@app.post("/token")
def login(form_data: OAuth2PasswordRequestForm = Depends(), db: Session = Depends(get_db)):
    """
    Логин и получение access и refresh токенов.
    """
    user = db.query(User).filter(User.email == form_data.username).first()
    if not user or not verify_password(form_data.password, user.hashed_password):
        raise HTTPException(status_code=401, detail="Invalid email or password")
    access_token = create_access_token(data={"sub": user.email})
    refresh_token = create_refresh_token(data={"sub": user.email})
    return {"access_token": access_token, "refresh_token": refresh_token, "token_type": "bearer"}

@app.post("/refresh")
def refresh_token(request: RefreshTokenRequest):
    """
    Обновление access-токена с использованием refresh-токена.
    """
    try:
        payload = decode_access_token(request.refresh_token)
        email = payload.get("sub")
        if not email:
            raise HTTPException(status_code=400, detail="Invalid token payload")
        new_access_token = create_access_token(data={"sub": email})
        return {"access_token": new_access_token, "token_type": "bearer"}
    except ValueError as e:
        raise HTTPException(status_code=401, detail=str(e))

@app.get("/protected/")
def protected_route(token: str = Depends(oauth2_scheme)):
    """
    Доступ только с валидным access токеном.
    """
    print("Запрос на /protected/ получен!")

    payload = decode_access_token(token)
    return {"message": f"Hello, {payload['sub']}!"}

@asynccontextmanager
async def lifespan(app: FastAPI):
    """
    Код для инициализации и завершения работы модуля.
    """
    # Нет какой-то особой инициализации
    yield
    # Нет и какой-либо особой деинициализации