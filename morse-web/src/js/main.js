function displayUsername() {
    let element = document.getElementById("my-name");
    element.innerText = getUsername();
}

async function toggleSidebar() {
    let sidebar = document.getElementsByTagName("nav")[0];
    let display_class = "nav-open";
    let is_displayed = sidebar.classList.contains(display_class);
    if(is_displayed) {
        sidebar.classList.remove(display_class);
    } else {
        sidebar.classList.add(display_class);
    }
}

async function onPageLoad() {
    addMessageHandler("room", onRoomMessage);
    document.addEventListener("DOMContentLoaded", displayUsername);
}