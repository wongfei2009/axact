//set body text content after dom loaded
document.addEventListener("DOMContentLoaded", function (event) {
    setInterval(async function () {
        let response = await fetch("/api/cpu");
        let data = await response.json();
        document.body.textContent = JSON.stringify(data);
    }, 1000);
});