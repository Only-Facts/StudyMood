function showMessage(message, type = 'info') {
  const messageBox = document.createElement('div');
  messageBox.className = 'message-box';
  messageBox.textContent = message;

  if (type === 'error') {
    messageBox.style.backgroundColor = 'hsl(0, 70%, 50%)';
  } else if (type === 'success') {
    messageBox.style.backgroundColor = 'hsl(120, 60%, 40%)';
  }

  document.body.appendChild(messageBox);
  setTimeout(() => {
    messageBox.remove();
  }, 3000);
}

function showConfirmation(message) {
  return new Promise((resolve) => {
    const modal = document.getElementById('confirmation-modal');
    const modalMessage = document.getElementById('modal-message');
    const confirmYes = document.getElementById('confirm-yes');
    const confirmNo = document.getElementById('confirm-no');

    modalMessage.textContent = message;
    modal.classList.add('visible');

    const handleConfirmation = (result) => {
      modal.classList.remove('visible');

      confirmYes.removeEventListener('click', onYesClick);
      confirmNo.removeEventListener('click', onNoClick);
      resolve(result);
    };

    const onYesClick = () => handleConfirmation(true);
    const onNoClick = () => handleConfirmation(false);

    confirmYes.addEventListener('click', onYesClick);
    confirmNo.addEventListener('click', onNoClick);
  });
}

async function deleteTodo(id) {
  const token = localStorage.getItem('jwt_token');
  if (!token) {
    showMessage("No Token Found. You may login first.", 'error');
    setTimeout(() => { window.location.href = "/profile"; }, 1500);
    return;
  }

  const confirmed = await showConfirmation("Do you really want to delete this todo?");
  if (!confirmed) {
    showMessage("Todo deletion cancelled.", 'info');
    return;
  }

  try {
    const response = await fetch(`${API_BASE_URL}/todos/${id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': token
      }
    });

    if (response.ok) {
      showMessage("Todo deleted successfully!", 'success');
      loadTodos();
    } else {
      const errorText = await response.text();
      showMessage(`Failed to delete todo: ${response.status} - ${errorText}`, 'error');
    }
  } catch (err) {
    console.error("Error while deleting :", err);
    showMessage(`Error deleting todo: ${err.message}`, 'error');
  }
}

function formatForDatetimeLocal(isoString) {
  if (!isoString) return '';
  const date = new Date(isoString);
  const year = date.getFullYear();
  const month = (date.getMonth() + 1).toString().padStart(2, '0');
  const day = date.getDate().toString().padStart(2, '0');
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${year}-${month}-${day}T${hours}:${minutes}`;
}

const todosTableWrapper = document.querySelector('.table-wrapper');
const todosTable = document.getElementById('todos');
let isOpen = true;
let currentTodosData = [];

async function loadTodos() {
  const btn = document.getElementById('todos-btn');
  const tbody = document.getElementById('todo-body');

  if (isOpen) {
    tbody.innerHTML = '';
    todosTableWrapper.classList.remove('visible');
    btn.innerText = 'Load Todos';
    isOpen = false;
  } else {
    btn.innerText = 'Loading Todos...';
    try {
      const token = localStorage.getItem('jwt_token');

      if (!token) {
        showMessage("No Token Found. You may login first.", 'error');
        setTimeout(() => { window.location.href = "/profile"; }, 1500);
        btn.innerText = 'Load Todos';
        isOpen = true;
        return;
      }

      const response = await fetch(`${API_BASE_URL}/todos/`, {
        method: 'GET',
        headers: {
          'Authorization': token
        }
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP Error: ${response.status} - ${errorText}`);
      }

      const todos = await response.json();
      currentTodosData = todos;
      tbody.innerHTML = '';

      todos.forEach(todo => {
        const row = document.createElement('tr');
        const date = new Date(todo.dtime);

        const formattedDate = date.toLocaleDateString('en-GB', {
          day: '2-digit',
          month: 'short',
          year: 'numeric'
        });
        const formattedTime = date.toLocaleTimeString('en-GB', {
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit'
        });

        const dueTime = `${formattedDate} at ${formattedTime}`;

        let statusClass = '';
        switch (todo.status.toLowerCase()) {
          case 'completed':
            statusClass = 'status-completed';
            break;
          case 'in progress':
            statusClass = 'status-in-progress';
            break;
          case 'pending':
            statusClass = 'status-pending';
            break;
          default:
            statusClass = 'status-default';
        }

        row.innerHTML = `
          <td>${todo.title}</td>
          <td>${todo.descr}</td>
          <td>${dueTime}</td>
          <td><span class="status-badge ${statusClass}">${todo.status}</span></td>
          <td>
            <button onclick="openUpdateModal(${todo.id})" class="update-btn">Update</button>
            <button onclick="deleteTodo(${todo.id})" class="delete-btn">Delete</button>
          </td>
      `;
        tbody.appendChild(row);
      });
      todosTableWrapper.classList.add('visible');
      btn.innerText = 'Unload Todos';
      isOpen = true;

    } catch (err) {
      console.error('Error while loading todos :', err);
      showMessage(`Error loading todos: ${err.message}`, 'error');
      todosTable.style.display = 'none';
      btn.innerText = 'Load Todos';
      isOpen = false;
    }
  }
}

function openAddModal() {
  const addModal = document.getElementById('add-todo-modal');
  document.getElementById('add-title').value = '';
  document.getElementById('add-description').value = '';
  document.getElementById('add-dtime').value = '';
  document.getElementById('add-status').value = 'Not Started';
  addModal.classList.add('visible');
}

function closeAddModal() {
  const addModal = document.getElementById('add-todo-modal');
  addModal.classList.remove('visible');
}

function openUpdateModal(id) {
  const todoToUpdate = currentTodosData.find(todo => todo.id === id);

  if (!todoToUpdate) {
    showMessage("Todo not found for update.", 'error');
    return;
  }

  const updateModal = document.getElementById('update-todo-modal');
  document.getElementById('update-todo-id').value = todoToUpdate.id;
  document.getElementById('update-title').value = todoToUpdate.title;
  document.getElementById('update-description').value = todoToUpdate.descr;
  document.getElementById('update-dtime').value = formatForDatetimeLocal(todoToUpdate.dtime);
  document.getElementById('update-status').value = todoToUpdate.status;

  updateModal.classList.add('visible');
}

function closeUpdateModal() {
  const updateModal = document.getElementById('update-todo-modal');
  updateModal.classList.remove('visible');
}

async function updateTodo(event) {
  event.preventDefault();

  const todoId = document.getElementById('update-todo-id').value;
  const token = localStorage.getItem('jwt_token');

  if (!token) {
    showMessage("No Token Found. You may login first.", 'error');
    setTimeout(() => { window.location.href = "/profile"; }, 1500);
    return;
  }

  const title = document.getElementById('update-title').value.trim();
  const description = document.getElementById('update-description').value.trim();
  const dtimeValue = document.getElementById('update-dtime').value.trim();
  const status = document.getElementById('update-status').value;

  const localDate = new Date(dtimeValue);
  const dtime = localDate.toISOString();

  const body = { title, description, dtime, status };

  try {
    const response = await fetch(`${API_BASE_URL}/todos/${todoId}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': token,
      },
      body: JSON.stringify(body)
    });

    if (response.ok) {
      showMessage("Todo updated successfully!", 'success');
      closeUpdateModal();
      loadTodos();
    } else {
      const errorText = await response.text();
      showMessage(`Failed to update todo: ${response.status} - ${errorText}`, 'error');
    }
  } catch (err) {
    console.error('Error while updating todo :', err);
    showMessage(`Error updating todo: ${err.message}`, 'error');
  }
}

async function addTodo(event) {
  event.preventDefault();

  const token = localStorage.getItem('jwt_token');

  if (!token) {
    showMessage("No Token Found. You may login first.", 'error');
    setTimeout(() => { window.location.href = "/profile"; }, 1500);
    return;
  }

  const titleInput = document.getElementById('add-title'); // <--- CHANGED: Target new modal input ID
  const descriptionInput = document.getElementById('add-description'); // <--- CHANGED
  const dtimeInput = document.getElementById('add-dtime'); // <--- CHANGED
  const statusSelect = document.getElementById('add-status');

  const title = titleInput.value.trim();
  const description = descriptionInput.value.trim();
  const dtimeValue = dtimeInput.value.trim();
  let status = statusSelect.value;

  if (!title || !description || !dtimeValue) {
    showMessage("Please fill in all required fields (Title, Description, Due Time).", 'error');
    return;
  }

  const localDate = new Date(dtimeValue);
  const dtime = localDate.toISOString();

  status = status === "" ? "Not Started" : status;

  const body = { title, description, dtime, status };

  try {
    const response = await fetch(`${API_BASE_URL}/todos/`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': token,
      },
      body: JSON.stringify(body)
    });

    if (response.ok) {
      showMessage("Todo added successfully!", 'success');
      titleInput.value = '';
      description.value = '';
      dtimeInput.value = '';
      statusSelect.value = '';
      closeAddModal();
      loadTodos();
    } else {
      const errorText = await response.text();
      showMessage(`Failed to add todo: ${response.status} - ${errorText}`, 'error');
    }
  } catch (err) {
    console.error('Error while adding todo :', err);
    showMessage(`Error adding todo: ${err.message}`, 'error');
  }
}

document.addEventListener('DOMContentLoaded', () => {
  todosTableWrapper.classList.remove('visible');
  document.getElementById('todos-btn').innerText = 'Load Todos';
  isOpen = false;

  document.getElementById('add-todo-form').addEventListener('submit', addTodo);
  document.getElementById('update-todo-form').addEventListener('submit', updateTodo);
});
