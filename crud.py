from sqlalchemy.orm import Session
from models import User
from utils import hash_password

def create_user(db: Session, name: str, email: str, password: str) -> User:
    hashed_password = hash_password(password)
    user = User(name=name, email=email, hashed_password=hashed_password)
    db.add(user)
    db.commit()
    db.refresh(user)
    return user

def get_user_by_email(db: Session, email: str) -> User:
    return db.query(User).filter(User.email == email).first()
