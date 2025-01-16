import pytest
from sqlalchemy.orm import Session
from models import User
from crud import create_user, get_user_by_email

def test_create_user(db_session: Session):
    # Очистка базы
    db_session.query(User).delete()
    db_session.commit()
    
    user = create_user(db_session, name="Test User", email="test@example.com", password="securepassword")
    assert user.email == "test@example.com"
    assert user.name == "Test User"

def test_get_user_by_email(db_session: Session):
    # Очистка базы
    db_session.query(User).delete()
    db_session.commit()
    
    create_user(db_session, name="Test User", email="test@example.com", password="securepassword")
    user = get_user_by_email(db_session, email="test@example.com")
    assert user is not None
    assert user.email == "test@example.com"

def test_get_nonexistent_user(db_session: Session):
    # Очистка базы
    db_session.query(User).delete()
    db_session.commit()
    
    user = get_user_by_email(db_session, email="nonexistent@example.com")
    assert user is None
