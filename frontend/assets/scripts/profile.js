async function isTokenValid() {
  const token = localStorage.getItem('jwt_token');
  if (!token) return showModal();

  const response = await fetch(`${API_BASE_URL}/users/`, {
    headers: {
      'Authorization': token
    }
  });

  response.ok ? hideModal(await response.json()) : showModal();
}

function showModal() {
  document.getElementById('auth-modal').style.display = 'flex';
}

function hideModal(user) {
  document.getElementById('auth-modal').style.display = 'none';
  const date = new Date(user.created_at);

  const year = date.getFullYear();
  const month = (date.getMonth() + 1).toString().padStart(2, '0');
  const day = date.getDate().toString().padStart(2, '0');

  const format = `${year}-${month}-${day}`
  const new_date = new Date(format);
  const options = { day: '2-digit', month: 'short', year: 'numeric' };
  const createdAt = new_date.toLocaleDateString('en-GB', options);

  const info = `Email: ${user.email}
Username: ${user.username}
Created the ${createdAt}
`
  document.getElementById('user-info').innerText = info;
}

async function handleRegisterSubmit(event) {
  event.preventDefault();
  const email = document.getElementById('email-register').value.trim();
  const password = document.getElementById('password-register').value.trim();
  const username = document.getElementById('username').value.trim();

  const body = { email, password, username };

  const response = await fetch(`${API_BASE_URL}/auth/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });

  if (response.status == 200) {
    const data = await response.json();
    localStorage.setItem('jwt_token', `Bearer ${data.token}`);
    await fetch(`${API_BASE_URL}/streaks/`, {
      method: 'POST',
      headers: { 'Authorization': data.token }
    });
    hideModal(data);
    location.reload();
  } else {
    if (response.status == 401) {
      alert("Authentication failed. Invalid Credentials");
    } else {
      alert(`Authentication failed. Error: ${response.status}`);
    }
  }
}

async function handleLoginSubmit(event) {
  event.preventDefault();
  const email = document.getElementById('email-login').value.trim();
  const password = document.getElementById('password-login').value.trim();

  const body = { email, password };

  const response = await fetch(`${API_BASE_URL}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body)
  });

  if (response.status == 200) {
    const data = await response.json();
    localStorage.setItem('jwt_token', `Bearer ${data.token}`);
    hideModal(data);
    location.reload();
  } else {
    if (response.status == 401) {
      alert("Authentication failed. Invalid Credentials");
    } else {
      alert(`Authentication failed. Error: ${response.status}`);
    }
  }
}

async function remove_token() {
  localStorage.removeItem('jwt_token');
  location.reload();
}

document.addEventListener('DOMContentLoaded', () => {
  isTokenValid();

  document.getElementById('login-form').addEventListener('submit', handleLoginSubmit);
  document.getElementById('register-form').addEventListener('submit', handleRegisterSubmit);
  const container = document.getElementById("container");
  const loginBtn = document.getElementById("login");
  const registerBtn = document.getElementById("register");

  registerBtn.addEventListener("click", () => {
    container.classList.remove("active");
  });

  loginBtn.addEventListener("click", () => {
    container.classList.add("active");
  });
});
