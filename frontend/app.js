const API_URL = "http://127.0.0.1:8080";

async function loadTasks() {
    // 1. On récupère le nom de l'utilisateur actuel
    const user = document.getElementById("username").value;

    // 2. Sécurité : si le champ est vide, on ne peut pas charger de liste
    if (!user) {
        console.warn("Veuillez saisir un nom d'utilisateur pour charger les tâches.");
        return;
    }

    // 3. Appel à ton API Rust (Route GET)
    // On utilise l'URL exacte de ton main.rs : /users/{user}/tasks
    const response = await fetch(`http://localhost:8080/users/${user}/tasks`);

    if (response.ok) {
        // 4. On extrait le JSON (ton Vec<Task> côté Rust)
        const tasks = await response.json();

        // 5. On cible la zone d'affichage
        const ul = document.getElementById("tasksList");

        // 6. TRÈS IMPORTANT : On vide la liste existante avant de la reconstruire
        ul.innerHTML = "";

        // 7. On crée un élément pour chaque tâche
        tasks.forEach(task => {
            const li = document.createElement("li");

            // On affiche le statut [X] ou [ ] + le titre
            const status = task.done ? "[X]" : "[ ]";
            li.textContent = `${status} ${task.title}`;

            // 8. On ajoute le <li> dans le <ul>
            ul.appendChild(li);
        });
        console.log("Liste mise à jour pour :", user);
    } else {
        console.error("Erreur lors du chargement :", response.status);
    }
}

async function addTask() {
    const title = document.getElementById("title").value;
    const user = document.getElementById("username").value;
    const startInput = document.getElementById("start").value;
    const endInput = document.getElementById("end").value;
    const taskInput = {
        title: title,
        start: startInput ? `${startInput}:00`:null,
        end: endInput ? `${endInput}:00`:null,
    }
    const response = await fetch(`http://127.0.0.1:8080/users/${user}/tasks`, {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(taskInput)//Convertit le javascript en json
    });
    // 2. On vérifie si l'ajout a fonctionné (Statut 201 Created ou 200 OK)
    if (response.ok) {
        console.log("Tâche ajoutée avec succès !");

        // 3. SEULEMENT ICI on rafraîchit la liste
        // Comme on a attendu le "ok" ci-dessus, Rust a fini d'écrire dans le HashMap
        await loadTasks();

        // 4. On vide le champ pour la prochaine tâche
        document.getElementById("title").value = "";
    } else {
        console.error("L'ajout a échoué. Statut :", response.status);
    }
}

async function deleteTask(user,id) {
    await fetch(`${API_URL}/users/${user}/tasks/${id}`, {
        method: "DELETE",
    })
    loadTasks();
}
// Dès que l'utilisateur finit de taper son nom et change de champ, on charge ses tâches
document.getElementById("username").addEventListener("change", loadTasks);
