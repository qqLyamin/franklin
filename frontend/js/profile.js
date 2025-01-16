import { getUserByEmail, updateUserProfile } from "./api.js";

document.addEventListener("DOMContentLoaded", async () => {
    const logoutButton = document.querySelector("#logout-btn");
    const profileData = document.querySelector("#profile-data");
    const form = document.querySelector("#update-profile-form");
    
    if (logoutButton) {
        logoutButton.addEventListener("click", () => {
            localStorage.removeItem("access_token");
            localStorage.removeItem("refresh_token");
            alert("Вы вышли из системы!");
            window.location.href = "/index.html"; // Перенаправление на главную страницу
        });
    }

    // Проверяем наличие токена авторизации
    const accessToken = localStorage.getItem("access_token");
    if (!accessToken) {
        alert("Вы не авторизованы!");
        window.location.href = "/login.html";
        return;
    }

    // Декодируем токен и извлекаем email
    let email;
    try {
        const payload = JSON.parse(atob(accessToken.split(".")[1]));
        email = payload.sub;
        console.log("Email из токена:", email);
    } catch (error) {
        console.error("Ошибка декодирования токена:", error);
        alert("Ошибка авторизации. Пожалуйста, войдите заново.");
        window.location.href = "/login.html";
        return;
    }

    // Получение данных пользователя
    try {
        const user = await getUserByEmail(email);
        console.log("Данные пользователя:", user);
        profileData.innerHTML = `
            <p><strong>Имя:</strong> ${user.name}</p>
            <p><strong>Email:</strong> ${user.email}</p>
            <p><strong>Навыки:</strong> ${user.skills || "не указаны"}</p>
            <p><strong>Интересы:</strong> ${user.interests || "не указаны"}</p>
        `;
        form.skills.value = user.skills || "";
        form.interests.value = user.interests || "";
    } catch (error) {
        profileData.innerHTML = "Ошибка загрузки профиля.";
        console.error("Ошибка:", error);
    }

    // Обработка формы обновления профиля
    form.addEventListener("submit", async (event) => {
        event.preventDefault();
        const skills = form.skills.value;
        const interests = form.interests.value;

        try {
            const updatedUser = await updateUserProfile({ skills, interests });
            alert("Профиль успешно обновлен!");
            profileData.innerHTML = `
                <p><strong>Имя:</strong> ${updatedUser.name}</p>
                <p><strong>Email:</strong> ${updatedUser.email}</p>
                <p><strong>Навыки:</strong> ${updatedUser.skills || "не указаны"}</p>
                <p><strong>Интересы:</strong> ${updatedUser.interests || "не указаны"}</p>
            `;
        } catch (error) {
            alert("Ошибка обновления профиля.");
            console.error("Ошибка:", error);
        }
    });
});

// Утилита для декодирования JWT
function parseJwt(token) {
    try {
        const base64Url = token.split(".")[1];
        const base64 = decodeURIComponent(
            atob(base64Url)
                .split("")
                .map((c) => "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2))
                .join("")
        );
        return JSON.parse(base64);
    } catch (error) {
        console.error("Ошибка декодирования токена:", error);
        return null;
    }
}
