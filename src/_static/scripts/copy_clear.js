document.querySelectorAll(".clear_btn").forEach((elm) => {
  elm.addEventListener("click", () => {
    const target = elm.getAttribute("clear-target");
    document.querySelector(target).value = "";
  });
});

document.querySelectorAll(".copy_btn").forEach((elm) => {
  elm.addEventListener("click", () => {
    const target = elm.getAttribute("copy-target");
    const text = document.querySelector(target).value;
    navigator.clipboard.writeText(text);
  });
});
