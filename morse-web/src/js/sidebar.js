const SIDEBAR_OPEN_CLASS = "nav-open";
const SIDEBAR_ROOM_LIST_ID = "room-list";

async function fillSidebar(room_list) {
    let rooms_element = document.getElementById(SIDEBAR_ROOM_LIST_ID);
    if(!rooms_element) {
        console.warn("Could not fill sidebar. Room list element does not exist");
        return;
    }

    rooms_element.textContent = ''; // Removes all current room buttons
    for(let room_id of room_list) {
        let room_button = document.createElement("button");
        room_button.innerText = room_id;
        room_button.onclick = () => {displayRoom(room_id); toggleSidebar()}; // TODO Decouple ?
        rooms_element.appendChild(room_button);
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