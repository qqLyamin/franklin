import pytest
from schemas import UserCreate
from pydantic import ValidationError

def test_valid_user_create():
    user = UserCreate(name="Test User", email="test@example.com", password="securepassword")
    assert user.name == "Test User"
    assert user.email == "test@example.com"

def test_invalid_user_create():
    with pytest.raises(ValidationError):
        UserCreate(name="Test User", email="invalidemail", password="123")
