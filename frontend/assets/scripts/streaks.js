async function updateStreak(token) {
  await fetch(`${API_BASE_URL}/streaks/`, {
    method: 'PUT',
    headers: {
      'Authorization': token
    }
  });
}

async function getStreak() {
  const token = localStorage.getItem('jwt_token');
  if (!token) return;
  await updateStreak(token);

  const response = await fetch(`${API_BASE_URL}/streaks/`, {
    headers: {
      'Authorization': token
    }
  });

  const streak = await response.json();

  const date = new Date(streak.last_active);
  const options = { day: '2-digit', month: 'short', year: 'numeric' };
  const last_active = date.toLocaleDateString('en-GB', options);

  document.getElementById('streak-text').innerText = `Streak: ${streak.current_streak}
  Longest: ${streak.longest_streak}
  Last Active Day: ${last_active}`
}

document.addEventListener('DOMContentLoaded', () => {
  getStreak();
})
