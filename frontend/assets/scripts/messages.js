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
