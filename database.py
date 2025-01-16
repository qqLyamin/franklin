from sqlalchemy import create_engine
from sqlalchemy.orm import declarative_base
from sqlalchemy.orm import sessionmaker
from config import DATABASE_URL, TEST_DATABASE_URL

# Создаем базу моделей
Base = declarative_base()

# Функция для создания движка базы данных
def get_engine(testing: bool = False):
    url = TEST_DATABASE_URL if testing else DATABASE_URL
    return create_engine(url)

# Основной движок и сессия
engine = get_engine()
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)
