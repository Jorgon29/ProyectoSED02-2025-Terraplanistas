const tags = [
    "Conectar con API", "Adventure", "Romance", "Mystery", "Fantasy", "Science Fiction",
    "Horror", "Thriller", "Historical", "Biography", "Self-Help",
    "Cookbook", "Travel", "Children's", "Young Adult", "Graphic Novel"
];

const multiSelect = document.getElementById("multiSelect");
const input = multiSelect.querySelector(".tag-input");
const dropdown = multiSelect.querySelector(".dropdown-list");
const selectedContainer = multiSelect.querySelector(".selected-tags");
let selected = [];

function renderDropdown(filter = "") {
    dropdown.innerHTML = "";
    const filtered = tags.filter(
        c => c.toLowerCase().includes(filter.toLowerCase()) && !selected.includes(c)
    );
    if (filtered.length === 0) {
        dropdown.style.display = "none";
        return;
    }
    filtered.forEach(country => {
        const div = document.createElement("div");
        div.className = "item";
        div.textContent = country;
        div.addEventListener("click", () => selectTag(country));
        dropdown.appendChild(div);
    });
    dropdown.style.display = "block";
}

function selectTag(value) {
    if (!selected.includes(value)) {
        selected.push(value);
        renderTags();
        input.value = "";
        renderDropdown();
    }
}

function renderTags() {
    const existingTags = selectedContainer.querySelectorAll(".badge");
    existingTags.forEach(b => b.remove());

    selected.forEach(tag => {
        const badge = document.createElement("span");
        badge.className = "badge bg-primary";
        badge.textContent = tag;

        const close = document.createElement("button");
        close.type = "button";
        close.className = "btn-close btn-close-white ms-2";
        close.style.fontSize = "0.6rem";
        close.addEventListener("click", () => {
            selected = selected.filter(t => t !== tag);
            renderTags();
        });

        badge.appendChild(close);
        selectedContainer.insertBefore(badge, input);
    });
}

input.addEventListener("input", e => renderDropdown(e.target.value));

input.addEventListener("focus", () => renderDropdown());

document.addEventListener("click", e => {
    if (!multiSelect.contains(e.target)) dropdown.style.display = "none";
});