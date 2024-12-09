const SIDEBAR_OPEN_CLASS = "nav-open";
const SIDEBAR_ROOMS_ELEMENT = document.getElementById("room-list");

async function fillSidebar(room_list) {
    SIDEBAR_ROOMS_ELEMENT.textContent = ''; // Removes all current room buttons
    for(let room_id of room_list) {
        let room_button = document.createElement("button");
        room_button.innerText = room_id;
        room_button.onclick = () => {displayRoom(room_id); toggleSidebar()}; // TODO Decouple ?
        SIDEBAR_ROOMS_ELEMENT.appendChild(room_button);
    }
}

async function toggleSidebar() {
    let sidebar = document.getElementsByTagName("nav")[0];
    if(!sidebar) {
        console.warn("Tried toggling sidebar but none was found.");
        return;
    }

    let is_displayed = sidebar.classList.contains(SIDEBAR_OPEN_CLASS);
    if(is_displayed) {
        sidebar.classList.remove(SIDEBAR_OPEN_CLASS);
    } else {
        sidebar.classList.add(SIDEBAR_OPEN_CLASS);
    }
}