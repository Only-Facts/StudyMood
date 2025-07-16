async function deleteTodo(id) {
  const token = localStorage.getItem('jwt_token');
  if (!token) {
    alert("No Token Found. You may login first.");
    window.location.href = "/profile";
  }

  const confirmed = confirm("Do you really want to delete this todo ?");
  if (!confirmed) return;

  try {
    const response = await fetch(`${API_BASE_URL}/todos/${id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': token
      }
    });

    if (response.ok) {
      alert("Todo deleted !");
      location.reload();
    } else {
      alert("Failed to delete todo (code " + response.status + ")");
    }
  } catch (err) {
    console.error("Error while deleting :", err)
  }
}

async function loadTodos() {
  try {
    const token = localStorage.getItem('jwt_token');

    if (!token) {
      alert("No Token Found. You may login first.");
      window.location.href = "/profile";
    }
    const response = await fetch(`${API_BASE_URL}/todos/`, {
      method: 'GET',
      headers: {
        'Authorization': token
      }
    });

    if (!response.ok) {
      throw new Error('HTTP Error: ' + response.status);
    }

    const todos = await response.json();
    const tbody = document.getElementById('todo-body');

    todos.forEach(todo => {
      const row = document.createElement('tr');

      row.innerHTML = `
            <td>${todo.title}</td>
            <td>${todo.descr}</td>
            <td>${todo.dtime}</td>
            <td>${todo.status}</td>
          `;
      const button = document.createElement('div');
      button.innerHTML = `
            <button onclick="deleteTodo(${todo.id})">Delete</button>
          `;

      tbody.appendChild(row);
      tbody.appendChild(button);
    });
  } catch (err) {
    console.error('Error while loading todos :', err);
    document.body.innerHTML += `<p style="color:red;">Erreur : ${err.message}</p>`;
  }
}
