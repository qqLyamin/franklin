export function showModal(user) {
    const modal = document.getElementById("user-modal");
    if (!modal) return;

    modal.querySelector(".modal-title").textContent = user.name;
    modal.querySelector(".modal-skills").textContent = `Навыки: ${user.skills || "Не указаны"}`;
    modal.querySelector(".modal-interests").textContent = `Интересы: ${user.interests || "Не указаны"}`;

    modal.classList.remove("hidden");
}

export function hideModal(modalId) {
    const modal = document.getElementById(modalId);
    modal.style.display = "none";
}

export function setupCloseButtons() {
    const modal = document.getElementById("user-modal");
    const closeButton = modal.querySelector(".close-btn");

    if (closeButton) {
        closeButton.addEventListener("click", () => {
            modal.classList.add("hidden");
            modal.style.display = "none";
        });
    }
}

document.querySelectorAll(".close-btn").forEach((btn) => {
    btn.addEventListener("click", (event) => {
        const modal = event.target.closest(".modal");
        modal.style.display = "none";
    });
});

export function closeModal() {
    const modal = document.getElementById("user-modal");
    if (modal) {
        modal.classList.add("hidden");
    }
}

document.addEventListener("click", (event) => {
    const modal = document.getElementById("user-modal");
    if (modal && !modal.contains(event.target) && !event.target.classList.contains("details-btn")) {
        closeModal();
    }
});