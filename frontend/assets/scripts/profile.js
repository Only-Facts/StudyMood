const API_BASE_URL = "http://localhost:8080/api";
let isLogin = true;

async function isTokenValid() {
  const token = localStorage.getItem('jwt_token');
  if (!token) return showModal();

  const response = await fetch(`${API_BASE_URL}/users/`, {
    headers: {
      'Authorization': token
    }
  });

  if (response.ok) { return hideModal() } else { return showModal() };
}

function showModal() {
  document.getElementById('auth-modal').style.display = 'flex';
}

function hideModal() {
  document.getElementById('auth-modal').style.display = 'none';
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

  const data = await response.json();

  if (response.ok && data.token) {
    localStorage.setItem('jwt_token', `Bearer ${data.token}`);
    hideModal();
    location.reload();
  } else {
    alert(data.message || 'Authentication failed.');
  }
}

document.addEventListener('DOMContentLoaded', () => {
  isTokenValid();

  document.getElementById('auth-form').addEventListener('submit', handleAuthSubmit);
  document.getElementById('toggle-auth').addEventListener('click', toggleAuthMode);
});

