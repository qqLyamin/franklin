# Skill Exchange Platform

## Описание проекта

Skill Exchange Platform — это веб-приложение, предназначенное для обмена навыками между пользователями. Пользователи могут зарегистрироваться, указать свои навыки и интересы, а также находить подходящих партнеров для обмена навыками.

---

## Функциональность

1. Регистрация пользователей
    - Возможность создать аккаунт с указанием имени, email и пароля.

2. Аутентификация
    - Логин через email и пароль.
    - Генерация Access и Refresh токенов.

3. Управление профилем
    - Обновление информации о пользователе (навыки, интересы).
    - Удаление аккаунта.

4. Поиск пользователей
    - Фильтрация пользователей по навыкам и интересам.
    - Пагинация при получении списка пользователей.

5. Защищенные маршруты
    - Доступ к закрытым маршрутам только при наличии валидного Access токена.

---

## Основные маршруты

### Пользователи
- GET `users` — Получение списка пользователей с пагинацией.
- POST `users` — Создание нового пользователя.
- PUT `users{user_id}` — Обновление профиля пользователя.
- DELETE `users{user_id}` — Удаление пользователя.
- GET `usersfilter` — Фильтрация пользователей по навыкам и интересам.

### Аутентификация
- POST `register` — Регистрация нового пользователя.
- POST `token` — Логин и получение токенов.
- POST `refresh` — Обновление Access токена с использованием Refresh токена.

### Прочее
- GET `/ping` — healthcheck
- GET `protected` — Защищенный маршрут, доступный только с валидным токеном.
- GET `/doc` — API Documentation
---

## Технологии

- Backend actix-web, sea-orm
- Database PostgreSQL
- Auth JWT (JSON Web Tokens)

---

## Локальный запуск

1. Поднимите PostgreSQL
2. Установите Rust
3. Установите sea-orm-cli
   ```bash
   cargo install sea-orm-cli
4. Задайте переменные окружения:
   ```bash
   cp env_example .env
5. Запустите миграции:
   ```bash
   sea-orm-cli migrate up
6. Запустите сервер разработки:
   ```bash
   cargo run
7. Откройте в браузере
   [http://127.0.0.1:8080](http://127.0.0.1:8080)


## Тестирование

## Entity generation example

```bash
sea-orm-cli generate entity -u postgresql://frank:ben@localhost/franklinclub -o src/internal/entity
```

## Generate openapi.json (needed for /doc) from swagger.yml

```bash
swagger-codegen generate -i swagger.yml -l openapi -o docs
```
