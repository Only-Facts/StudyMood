const API_BASE_URL = "https://becomes-colon-vinyl-luggage.trycloudflare.com/api";
let isLogin = true;

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

  const info = `Email: ${user.email}
First Name: ${user.first_name}
Last Name: ${user.last_name}
Created At: ${user.created_at}
`
  document.getElementById('user-info').innerText = info;
}

function toggleAuthMode() {
  isLogin = !isLogin;
  document.getElementById('auth-title').innerText = isLogin ? 'Login' : 'Register';
  document.getElementById('toggle-auth').innerText = isLogin
    ? "Don't have an account? Register"
    : "Already have an account? Login";

  document.getElementById('fname').style.display = isLogin ? 'none' : 'block';
  document.getElementById('lname').style.display = isLogin ? 'none' : 'block';
}

async function handleAuthSubmit(event) {
  event.preventDefault();
  const email = document.getElementById('email').value.trim();
  const password = document.getElementById('password').value.trim();

  const body = { email, password };

  if (!isLogin) {
    body.fname = document.getElementById('fname').value.trim();
    body.lname = document.getElementById('lname').value.trim();
  }

  const endpoint = isLogin ? '/auth/login' : '/auth/register';

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
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

  document.getElementById('auth-form').addEventListener('submit', handleAuthSubmit);
  document.getElementById('toggle-auth').addEventListener('click', toggleAuthMode);
});

