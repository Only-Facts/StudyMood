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

let isOpen = true;

async function loadTodos() {
  isOpen = !isOpen;
  const todos = document.getElementById('todos');
  if (isOpen) {
    const tbody = document.getElementById('todo-body');
    tbody.innerHTML = '';
    todos.style.display = 'none';
  } else {
    todos.style.display = 'block';
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
      tbody.innerHTML = '';

      todos.forEach(todo => {
        const row = document.createElement('tr');
        const date = new Date(todo.dtime);

        const year = date.getFullYear();
        const month = (date.getMonth() + 1).toString().padStart(2, '0');
        const day = date.getDate().toString().padStart(2, '0');

        const hour = date.getHours().toString().padStart(2, '0');
        const min = date.getMinutes().toString().padStart(2, '0');
        const sec = date.getSeconds().toString().padStart(2, '0');

        const dueTime = `${day}/${month}/${year} at ${hour}:${min}:${sec}`

        row.innerHTML = `
            <td>${todo.title}</td>
            <td>${todo.descr}</td>
            <td>${dueTime}</td>
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
}
