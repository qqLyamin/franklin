const API_BASE_URL = "http://127.0.0.1:8000"; // URL бэкенда

/**
 * Регистрирует нового пользователя.
 * @param {string} name - Имя пользователя.
 * @param {string} email - Email пользователя.
 * @param {string} password - Пароль пользователя.
 * @returns {Promise<Object>} - Данные зарегистрированного пользователя.
 */
export async function registerUser(name, email, password) {
    try {
        const response = await fetch(`${API_BASE_URL}register/`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ name, email, password }),
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || "Ошибка регистрации");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка регистрации:", error);
        throw error;
    }
}

/**
 * Получает список пользователей.
 * @returns {Promise<Array>} - Массив пользователей.
 */
export async function getUsers() {
    try {
        const response = await fetch(`${API_BASE_URL}users/`, {
            method: "GET",
        });

        if (!response.ok) {
            throw new Error("Ошибка получения пользователей");
        }

        const data = await response.json();
        console.log("Список пользователей:", data); // Лог для отладки
        return data;
    } catch (error) {
        console.error("Ошибка получения пользователей:", error);
        throw error;
    }
}

export async function updateUserProfile(data) {
    try {
        const accessToken = localStorage.getItem("access_token");
        if (!accessToken) {
            throw new Error("Пользователь не авторизован");
        }

        const response = await fetch(`${API_BASE_URL}users/update`, {
            method: "PUT",
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${accessToken}`,
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || "Ошибка обновления профиля");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка обновления профиля:", error);
        throw error;
    }
}


/**
 * Получает данные пользователя по ID.
 * @param {number} userId - ID пользователя.
 * @returns {Promise<Object>} - Данные пользователя.
 */
export async function getUserById(userId) {
    try {
        const accessToken = localStorage.getItem("access_token");
        if (!accessToken) {
            throw new Error("Пользователь не авторизован");
        }

        const response = await fetch(`${API_BASE_URL}users/${userId}`, {
            method: "GET",
            headers: {
                Authorization: `Bearer ${accessToken}`,
            },
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || "Ошибка получения данных пользователя");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка в getUserById:", error);
        throw error;
    }
}

export async function getUserByEmail(email) {
    try {
        const accessToken = localStorage.getItem("access_token");
        if (!accessToken) {
            throw new Error("Пользователь не авторизован");
        }

        const response = await fetch(`${API_BASE_URL}users/email/${encodeURIComponent(email)}`, {
            method: "GET",
            headers: {
                Authorization: `Bearer ${accessToken}`,
                "Content-Type": "application/json",
            },
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || "Ошибка получения данных пользователя");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка в getUserByEmail:", error);
        throw error;
    }
}


export async function login(email, password) {
    try {
        const formData = new URLSearchParams();
        formData.append("grant_type", "password");
        formData.append("username", email);
        formData.append("password", password);

        const response = await fetch(`${API_BASE_URL}token`, {
            method: "POST",
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
            },
            body: formData,
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.detail || "Ошибка авторизации");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка авторизации:", error);
        throw error;
    }
}