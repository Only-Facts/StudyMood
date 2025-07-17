document.addEventListener("DOMContentLoaded", () => {
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

