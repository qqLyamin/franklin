def test_root(client):
    """Проверка корневого маршрута."""
    response = client.get("/")
    assert response.status_code == 200
    assert response.json() == {"message": "Welcome to the Skill Exchange Platform!"}

def test_get_users_empty(client):
    """Проверка получения списка пользователей, когда их нет в базе."""
    response = client.get("/users/")
    assert response.status_code == 200
    assert response.json() == []

def test_register_with_invalid_data(client):
    response = client.post("/register/", json={})
    assert response.status_code == 422  # Unprocessable Entity

def test_protected_route_without_token(client):
    response = client.get("/protected/")
    assert response.status_code == 401
    assert response.json()["detail"] == "Not authenticated"

def test_get_users_with_pagination(client):
    """Проверка пагинации при получении списка пользователей."""
    client.post("/register/", json={
        "name": "User 1",
        "email": "user1@example.com",
        "password": "password"
    })
    client.post("/register/", json={
        "name": "User 2",
        "email": "user2@example.com",
        "password": "password"
    })
    response = client.get("/users/?skip=0&limit=1")
    assert response.status_code == 200
    assert len(response.json()) == 1

def test_delete_user_success(client):
    """Проверка успешного удаления пользователя."""
    register_response = client.post("/register/", json={
        "name": "User to Delete",
        "email": "delete@example.com",
        "password": "password"
    })
    user_id = register_response.json()["id"]
    response = client.delete(f"/users/{user_id}")
    assert response.status_code == 200
    assert response.json() == {"message": f"User with id {user_id} was successfully deleted"}

def test_delete_user_not_found(client):
    """Проверка удаления несуществующего пользователя."""
    response = client.delete("/users/9999")
    assert response.status_code == 404
    assert response.json()["detail"] == "User not found"

def test_update_user_success(client):
    """Проверка успешного обновления пользователя."""
    register_response = client.post("/register/", json={
        "name": "User to Update",
        "email": "update@example.com",
        "password": "password"
    })
    user_id = register_response.json()["id"]
    response = client.put(f"/users/{user_id}", json={"skills": "Python", "interests": "AI"})
    assert response.status_code == 200
    assert response.json()["skills"] == "Python"
    assert response.json()["interests"] == "AI"

def test_update_user_not_found(client):
    """Проверка обновления несуществующего пользователя."""
    response = client.put("/users/9999", json={"skills": "Python", "interests": "AI"})
    assert response.status_code == 404
    assert response.json()["detail"] == "User not found"

def test_filter_users_by_skills(client):
    """Проверка фильтрации пользователей по навыкам."""
    client.post("/register/", json={
        "name": "Skillful User",
        "email": "skillful@example.com",
        "password": "password"
    })
    user_id = client.get("/users/").json()[0]["id"]
    client.put(f"/users/{user_id}", json={"skills": "Python"})
    response = client.get("/users/filter/?skills=Python")
    assert response.status_code == 200
    assert len(response.json()) == 1
    assert response.json()[0]["skills"] == "Python"

def test_create_user_duplicate_email(client):
    client.post("/users/", json={
        "name": "Duplicate User",
        "email": "duplicate@example.com",
        "password": "password"
    })
    response = client.post("/users/", json={
        "name": "Duplicate User",
        "email": "duplicate@example.com",
        "password": "password"
    })
    assert response.status_code == 400
    assert response.json()["detail"] == "User already exists"

def test_filter_users_by_interests(client):
    """Проверка фильтрации пользователей по интересам."""
    client.post("/register/", json={
        "name": "Interested User",
        "email": "interested@example.com",
        "password": "password"
    })
    user_id = client.get("/users/").json()[0]["id"]
    client.put(f"/users/{user_id}", json={"interests": "AI"})
    response = client.get("/users/filter/?interests=AI")
    assert response.status_code == 200
    assert len(response.json()) == 1
    assert response.json()[0]["interests"] == "AI"

def test_filter_users_no_results(client):
    """Проверка фильтрации пользователей, когда результатов нет."""
    response = client.get("/users/filter/?skills=NonexistentSkill")
    assert response.status_code == 200
    assert response.json() == []

def test_create_user_invalid_data(client):
    """Проверка создания пользователя с некорректными данными."""
    response = client.post("/register/", json={"email": "invalid@example.com"})
    assert response.status_code == 422
    assert "detail" in response.json()

def test_login_invalid_credentials(client):
    """Проверка входа с некорректными учетными данными."""
    response = client.post("/token", data={
        "username": "nonexistent@example.com",
        "password": "wrongpassword"
    })
    assert response.status_code == 401
    assert response.json()["detail"] == "Invalid email or password"

def test_refresh_token_success(client):
    """Проверка успешного обновления токена."""
    client.post("/register/", json={
        "name": "Test User",
        "email": "refresh@example.com",
        "password": "password"
    })
    login_response = client.post("/token", data={
        "username": "refresh@example.com",
        "password": "password"
    })
    refresh_token = login_response.json()["refresh_token"]
    response = client.post("/refresh", json={"refresh_token": refresh_token})
    assert response.status_code == 200
    assert "access_token" in response.json()

def test_refresh_token_invalid(client):
    """Проверка обновления токена с недействительным refresh-токеном."""
    response = client.post("/refresh", json={"refresh_token": "invalid.token.value"})
    assert response.status_code == 401
    assert "detail" in response.json()
