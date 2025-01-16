# config.py
import os
from datetime import timedelta

SECRET_KEY = os.getenv("SECRET_KEY", "eyJhbGciOiJIUzI1NiJ9.ew0KICAic3ViIjogIjEyMzQ1Njc4OTAiLA0KICAibmFtZSI6ICJBbmlzaCBOYXRoIiwNCiAgImlhdCI6IDE1MTYyMzkwMjINCn0.tY6DHhgVf57EwQSqIdeRd1sqMawlfJpn6-npu_jnUy0")  # Замените на безопасный ключ
ALGORITHM = "HS256"
ACCESS_TOKEN_EXPIRE_MINUTES = 180
REFRESH_TOKEN_EXPIRE_DAYS = 7

# URL для подключения к базе данных
DATABASE_URL = os.getenv("DATABASE_URL", "postgresql://frank:ben@localhost/franklinclub")
TEST_DATABASE_URL = os.getenv("TEST_DATABASE_URL", "postgresql://frank:ben@localhost/franklinclub_test")

# Header
# {"alg":"HS256"}
# State
# SIGNED
# Serialize
# eyJhbGciOiJIUzI1NiJ9.ew0KICAic3ViIjogIjEyMzQ1Njc4OTAiLA0KICAibmFtZSI6ICJBbmlzaCBOYXRoIiwNCiAgImlhdCI6IDE1MTYyMzkwMjINCn0.tY6DHhgVf57EwQSqIdeRd1sqMawlfJpn6-npu_jnUy0
# Singature
# tY6DHhgVf57EwQSqIdeRd1sqMawlfJpn6-npu_jnUy0
# SharedSecret (Generated for MAC key (Base64 encoded))
# TWrLFi7eGhxGoVGS2GBHeTEyOJmFaMhrq7MNz2r+KA0=