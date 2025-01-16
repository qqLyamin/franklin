import pytest
from fastapi.testclient import TestClient
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker
from database import Base, get_engine, SessionLocal
from main import app, get_db

# Настраиваем движок для тестовой базы
engine = get_engine(testing=True)
TestingSessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

@pytest.fixture(scope="module")
def db_session():
    """
    Фикстура для предоставления сессии базы данных.
    """
    # Создаем таблицы в тестовой базе
    Base.metadata.create_all(bind=engine)
    session = TestingSessionLocal()
    try:
        yield session
    finally:
        session.close()
        # Удаляем таблицы после завершения тестов
        Base.metadata.drop_all(bind=engine)

@pytest.fixture(scope="function")
def client():
    """
    Фикстура для тестирования API.
    """
    # Очищаем базу данных перед тестами
    Base.metadata.drop_all(bind=engine)
    Base.metadata.create_all(bind=engine)

    def override_get_db():
        db = TestingSessionLocal()
        try:
            yield db
        finally:
            db.close()

    app.dependency_overrides[get_db] = override_get_db

    with TestClient(app) as c:
        yield c
