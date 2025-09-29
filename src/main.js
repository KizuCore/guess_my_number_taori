const invoke = window.__TAURI__.core.invoke;

const input = document.querySelector("#guess");
const button = document.querySelector("#try");
const feedback = document.querySelector("#feedback");

async function tryGuess() {
  const value = parseInt(input.value, 10);
  if (isNaN(value)) {
    feedback.textContent = "Entrez un nombre valide.";
    return;
  }
  try {
    const res = await invoke("check_guess", { guess: value });
    feedback.textContent = res;
  } catch (e) {
    console.error(e);
    feedback.textContent = "Erreur Tauri : " + e;
  }
}

button.addEventListener("click", tryGuess);
input.addEventListener("keydown", (e) => e.key === "Enter" && tryGuess());
