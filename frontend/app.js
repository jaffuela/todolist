const USER = "alice";
const BASE_URL = "http://localhost:8080/users/alice/tasks";
async function loadTasks() {
    const response = await fetch(BASE_URL);
    const tasks = await response.json();
    const ul = document.getElementById("tasks");
    ul.innerHTML = "";
    for (const task of tasks) {
        const li = document.createElement("li"); //Crée élément html en mémoire
        li.textContent = task.title;
        ul.appendChild(li); //Ajoute le li dans le ul, l'élément devient visible
    }
}

async function addTask() {
    const input = document.getElementById("title");
    const title = input.value;
    await fetch(BASE_URL, {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({title : title})//Convertit le javascript en json
    });
    input.value = "";
    loadTasks();
}