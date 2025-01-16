import { login, registerUser } from "./api.js";

export function setupFormHandlers() {
    // Логин
    const loginForm = document.querySelector("#loginForm");
    if (loginForm) {
        loginForm.addEventListener("submit", async (event) => {
            event.preventDefault();
            const email = document.querySelector("#email").value;
            const password = document.querySelector("#password").value;

            try {
                // Отправка запроса на логин
                const tokens = await login(email, password);
                console.log("Успешная авторизация:", tokens);

                // Сохраняем токены в localStorage
                localStorage.setItem("access_token", tokens.access_token);
                localStorage.setItem("refresh_token", tokens.refresh_token);

                // Сообщение об успешном входе
                alert("Вы успешно вошли в систему!");

                // Редирект на страницу профиля
                window.location.href = "/profile.html";
            } catch (error) {
                console.error("Ошибка входа:", error);
                alert("Ошибка входа: " + error.message);
            }
        });
    }

    // Регистрация
    const registerForm = document.querySelector("#register-form");
    if (registerForm) {
        registerForm.addEventListener("submit", async (event) => {
            event.preventDefault();
            const name = document.querySelector("#name").value;
            const email = document.querySelector("#email").value;
            const password = document.querySelector("#password").value;

            try {
                // Отправка запроса на регистрацию
                await registerUser(name, email, password);
                alert("Регистрация прошла успешно!");

                // Редирект на страницу входа
                window.location.href = "/login.html";
            } catch (error) {
                console.error("Ошибка регистрации:", error);
                alert("Ошибка регистрации: " + error.message);
            }
        });
    }
}
