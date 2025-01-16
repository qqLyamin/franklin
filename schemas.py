from pydantic import BaseModel, EmailStr

# Базовая модель пользователя
class UserBase(BaseModel):
    name: str | None = None
    email: EmailStr

# Модель для создания пользователя
class UserCreate(UserBase):
    password: str
    skills: str | None = None
    interests: str | None = None

# Модель для возврата информации о пользователе
class User(UserBase):
    id: int
    skills: str | None = None
    interests: str | None = None

    class Config:
        orm_mode = True
