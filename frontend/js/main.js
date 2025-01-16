import { getUsers, getUserById } from "./api.js";
import { setupCloseButtons, showModal, closeModal } from "./modals.js";
import { setupFormHandlers } from "./forms.js";
const API_BASE_URL = "http://127.0.0.1:8000"; // URL бэкенда

document.addEventListener("DOMContentLoaded", async () => {
    // Логика отображения кнопок в зависимости от авторизации
    Event.log("Определяемся какие кнопки показывать")
    await handleNavigationButtons();

    setupFormHandlers(); // Настройка форм логина и регистрации
    setupCloseButtons(); // Настройка кнопок закрытия

    const userCarousel = document.getElementById("user-carousel");
    const showMoreBtn = document.querySelector(".show-more-btn");

    if (userCarousel) {
        await populateHorizontalCarousel(userCarousel); // Загрузка пользователей в карусель
        addCarouselScrollButtons(); // Добавляем кнопки прокрутки
    }

    if (showMoreBtn) {
        showMoreBtn.addEventListener("click", async () => {
            try {
                const userContainer = document.querySelector(".popular-users .container");
                await populateUserCards(userContainer, true);
            } catch (error) {
                console.error("Ошибка подгрузки пользователей:", error);
                showNotification("Не удалось загрузить дополнительные данные.");
            }
        });
    }

    const loginForm = document.querySelector("#login-form");

    if (loginForm) {
        loginForm.addEventListener("submit", async (event) => {
            event.preventDefault(); // Предотвращаем перезагрузку страницы
            const email = document.querySelector("#email").value;
            const password = document.querySelector("#password").value;

            try {
                // Вызываем функцию для авторизации
                const data = await login(email, password);
                console.log("Авторизация успешна:", data);

                // Сохраняем токены в localStorage
                localStorage.setItem("access_token", data.access_token);
                localStorage.setItem("refresh_token", data.refresh_token);

                // Перенаправление после успешного входа
                window.location.href = "/profile.html";
            } catch (error) {
                console.error("Ошибка авторизации:", error);
                alert("Неправильный email или пароль");
            }
        });
    }
});

/**
 * Управляет отображением кнопок "Войти", "Профиль", "Разлогиниться" и "Зарегистрироваться".
 */
async function handleNavigationButtons() {
    const loginLink = document.querySelector("#login-link");
    const profileLink = document.querySelector("#profile-link");
    const registerButton = document.querySelector(".register-btn");
    const logoutButton = document.querySelector("#logout-btn");
    const userCarousel = document.getElementById("user-carousel");

    const accessToken = localStorage.getItem("access_token");

    // Функция для установки видимости элемента
    function setElementVisibility(element, isVisible) {
        if (element) {
            element.style.display = isVisible ? "inline-block" : "none";
            console.log(`Visibility set for ${element.id || element.className}: ${isVisible ? "visible" : "hidden"}`);
        }
    }

    // Логика авторизации
    if (accessToken) {
        try {
            const payload = JSON.parse(atob(accessToken.split(".")[1])); // Декодирование токена
            console.log("Payload токена:", payload);
        } catch (error) {
            console.error("Ошибка декодирования токена:", error);
        }

        // Пользователь авторизован
        setElementVisibility(loginLink, false);
        setElementVisibility(profileLink, true);
        setElementVisibility(logoutButton, true);
        setElementVisibility(registerButton, false);

        profileLink?.addEventListener("click", () => {
            window.location.href = "/profile.html";
        });

        logoutButton?.addEventListener("click", () => {
            localStorage.removeItem("access_token");
            localStorage.removeItem("refresh_token");
            showNotification("Вы вышли из системы!");
            setTimeout(() => (window.location.href = "/index.html"), 1000);
        });
    } else {
        // Пользователь не авторизован
        setElementVisibility(loginLink, true);
        setElementVisibility(profileLink, false);
        setElementVisibility(logoutButton, false);
        setElementVisibility(registerButton, true);
    }

    // Логика загрузки пользователей (не связана с кнопками)
    if (userCarousel) {
        try {
            const users = await fetchInitialUsers(userCarousel);

            if (users.length === 0) {
                displayMessage(userCarousel, "Нет пользователей в базе.");
            } else if (users.length < 5) {
                displayMessage(userCarousel, `В базе найдено только ${users.length} пользователей.`);
            } else {
                populateUserCarousel(userCarousel, users);
            }
        } catch (error) {
            console.error("Ошибка загрузки пользователей:", error);
            displayMessage(userCarousel, "Ошибка загрузки пользователей.");
        }
    }
}


/**
 * Добавляет кнопки для прокрутки горизонтальной карусели.
 */
function addCarouselScrollButtons() {
    const userCarousel = document.getElementById("user-carousel");
    if (!userCarousel) return;

    // Создаем кнопки
    const leftButton = document.createElement("button");
    leftButton.className = "carousel-button left";
    leftButton.innerHTML = "&#10094;"; // Стрелка влево
    leftButton.onclick = () => scrollCarousel(-200);

    const rightButton = document.createElement("button");
    rightButton.className = "carousel-button right";
    rightButton.innerHTML = "&#10095;"; // Стрелка вправо
    rightButton.onclick = () => scrollCarousel(200);

    // Добавляем кнопки в родительский контейнер карусели
    userCarousel.parentElement.appendChild(leftButton);
    userCarousel.parentElement.appendChild(rightButton);
}

/**
 * Функция для прокрутки карусели.
 */
function scrollCarousel(offset) {
    const carousel = document.getElementById("user-carousel");
    if (carousel) {
        carousel.scrollBy({ left: offset, behavior: "smooth" });
    }
}

/**
 * Загрузка пользователей в горизонтальную карусель.
 */
async function populateHorizontalCarousel(container) {
    if (container.id !== "user-carousel") {
        console.warn("Функция populateHorizontalCarousel предназначена только для user-carousel.");
        return;
    }

    try {
        const users = await fetchInitialUsers(container);
        if (users.length > 0) {
            populateUserCarousel(container, users);
        } else {
            displayMessage(container, "Нет пользователей в базе.");
        }
    } catch (error) {
        console.error("Ошибка загрузки пользователей для карусели:", error);
        displayMessage(container, "Ошибка загрузки пользователей.");
    }
}

/**
 * Загрузка списка пользователей в основной контейнер.
 */
async function populateUserCards(container, loadMore = false) {
    if (container.id === "user-carousel") {
        console.warn("Функция populateUserCards не должна заполнять user-carousel.");
        return;
    }

    try {
        const users = await getUsers();
        if (!users || users.length === 0) {
            console.warn("Нет данных для отображения.");
            return;
        }

        if (!loadMore) {
            container.innerHTML = ""; // Очистка контейнера, если не подгружаем больше
        }

        users.forEach((user) => {
            const userCard = createUserCard(user);
            container.appendChild(userCard);
        });

        attachDetailsButtons();
    } catch (error) {
        console.error("Ошибка загрузки пользователей:", error);
    }
}

/**
 * Создание карточки пользователя.
 */
function createUserCard(user) {
    const userCard = document.createElement("div");
    userCard.className = "user-card";
    userCard.innerHTML = `
        <img src="assets/user-icon.svg" alt="User Avatar">
        <h3>${user.name}</h3>
        <p>Навыки: ${user.skills || "Не указаны"}</p>
        <p>Интересы: ${user.interests || "Не указаны"}</p>
        <button class="details-btn" data-name="${user.name}">Подробнее</button>
    `;
    return userCard;
}

/**
 * Привязка событий к кнопкам "Подробнее".
 */
function attachDetailsButtons() {
    document.querySelectorAll(".details-btn").forEach((button) => {
        button.addEventListener("click", async (event) => {
            const accessToken = localStorage.getItem("access_token");
            if (!accessToken) {
                showNotification("Пожалуйста, войдите в систему, чтобы посмотреть подробности о пользователе.");
                return;
            }

            const name = event.target.dataset.name; // Получаем имя пользователя из кнопки
            try {
                const user = await getUserByName(name);

                // Заполнение данных модального окна
                const modal = document.getElementById("user-modal");
                modal.querySelector(".modal-title").innerText = user.name;
                modal.querySelector(".modal-skills").innerText = `Навыки: ${user.skills || "Не указаны"}`;
                modal.querySelector(".modal-interests").innerText = `Интересы: ${user.interests || "Не указаны"}`;

                // Показываем модальное окно
                modal.classList.remove("hidden");
                modal.style.display = "flex";
            } catch (error) {
                console.error("Ошибка получения данных пользователя:", error);
                showNotification("Не удалось загрузить данные пользователя.", 5000);
            }
        });
    });
}

async function getUserByName(name) {
    try {
        const encodedName = encodeURIComponent(name.trim()); // Убираем лишние пробелы и кодируем
        const response = await fetch(`${API_BASE_URL}/users/by-name/${encodedName}`, {
            headers: {
                Authorization: `Bearer ${localStorage.getItem("access_token")}`,
                "Content-Type": "application/json",
            },
        });

        if (!response.ok) {
            throw new Error("Не удалось получить данные пользователя");
        }

        return await response.json();
    } catch (error) {
        console.error("Ошибка загрузки данных пользователя:", error);
        throw error;
    }
}

/**
 * Уведомления.
 */
function showNotification(message, duration = 3000) {
    const notification = document.getElementById("notification");
    const notificationMessage = document.getElementById("notification-message");

    if (notification && notificationMessage) {
        notificationMessage.textContent = message;
        notification.classList.remove("hidden");
        notification.classList.add("show");

        setTimeout(() => {
            notification.classList.remove("show");
            notification.classList.add("hidden");
        }, duration);
    }
}

/**
 * Отображение сообщения в ленте.
 */
function displayMessage(container, message) {
    container.innerHTML = `<p style="padding: 20px; text-align: center;">${message}</p>`;
}

/**
 * Заполняет карусель карточками пользователей.
 */
function populateUserCarousel(container, users) {
    container.innerHTML = ""; // Очищаем контейнер перед добавлением новых карточек
    users.forEach((user) => {
        const userCard = document.createElement("div");
        userCard.className = "user-card";
        userCard.innerHTML = `
            <img src="assets/user-icon.svg" alt="User Avatar" style="width: 100px;">
            <h3>${user.name}</h3>
            <p>Навыки: ${user.skills || "Не указаны"}</p>
            <p>Интересы: ${user.interests || "Не указаны"}</p>
            <button class="details-btn" data-name="${user.name}">Подробнее</button>
        `;
        container.appendChild(userCard);
    });

    // Привязываем обработчики к кнопкам "Подробнее"
    attachDetailsButtons();
}

/**
 * Получение первых 5 пользователей из базы.
 */
async function fetchInitialUsers(container) {
    try {
        const users = await getUsers(); // Получение списка пользователей из API
        return users.slice(0, 5); // Возвращаем первых 5 пользователей
    } catch (error) {
        console.error("Ошибка загрузки пользователей:", error);
        displayMessage(container, "Ошибка загрузки пользователей.");
        return [];
    }
}
