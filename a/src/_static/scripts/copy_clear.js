document.querySelectorAll(".clear_btn").forEach((elm) => {
  elm.addEventListener("click", () => {
    const target = document.querySelector(elm.getAttribute("clear-target"));
    if (!target) return;
    target.innerText = "";
  });
});

document.querySelectorAll(".copy_btn").forEach((elm) => {
  elm.addEventListener("click", () => {
    const target = document.querySelector(elm.getAttribute("copy-target"));
    if (!target) return;
    const text = target.value;
    navigator.clipboard.writeText(text);
  });
});
