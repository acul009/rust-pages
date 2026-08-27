document.addEventListener("click", (event) => {
    document.querySelectorAll("nav details[open]").forEach((details) => {
        if (!details.contains(event.target)) {
            details.removeAttribute("open");
        }
    });
});
