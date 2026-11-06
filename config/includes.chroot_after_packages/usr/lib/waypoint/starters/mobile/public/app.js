const count = document.getElementById("count");

function read() {
  try {
    return Number(localStorage.getItem("tally")) || 0;
  } catch {
    return 0;
  }
}

function show(value) {
  count.textContent = value;
  try {
    localStorage.setItem("tally", value);
  } catch {
    // The tally still works if the browser blocks storage. It just won't be saved.
  }
}

document.getElementById("add").addEventListener("click", () => show(read() + 1));
document.getElementById("reset").addEventListener("click", () => show(0));
show(read());

if ("serviceWorker" in navigator) {
  navigator.serviceWorker.register("sw.js");
}
