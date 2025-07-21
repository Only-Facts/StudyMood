async function updateStreak() {
  const token = localStorage.getItem('jwt_token');
  if (!token) return;

  await fetch(`${API_BASE_URL}/streaks/`, {
    method: 'PUT',
    headers: {
      'Authorization': token
    }
  });
}

async function getStreak() {
  await updateStreak();
  const token = localStorage.getItem('jwt_token');
  if (!token) return;

  const response = await fetch(`${API_BASE_URL}/streaks/`, {
    headers: {
      'Authorization': token
    }
  });

  const streak = await response.json();

  const date = new Date(streak.last_active);
  const options = { day: '2-digit', month: 'short', year: 'numeric' };
  const last_active = date.toLocaleDateString('en-GB', options);

  document.getElementById('streak-text').innerHTML = `<p>Streak: ${streak.current_streak}</p>
  <p>Longest: ${streak.longest_streak}</p>
  <p>Last Active Day: ${last_active}</p>`
}

document.addEventListener('DOMContentLoaded', () => {
  getStreak();
})
