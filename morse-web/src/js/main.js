const NO_ROOM_ELEMENT = document.getElementById("no-room");
const ROOM_ELEMENT = document.getElementById("room");

function displayUsername() {
    let element = document.getElementById("my-name");
    element.innerText = getUsername();
}

async function updateRoomDisplay(room_list) {
    fillSidebar(room_list);    
    let has_no_room = room_list.length == 0;
    NO_ROOM_ELEMENT.hidden = !has_no_room;
    ROOM_ELEMENT.hidden = has_no_room;
}

async function onPageLoad() {
    addMessageHandler("room", async event => {
        onRoomMessage(event);
        let room_list = getRooms();
        updateRoomDisplay(room_list);
    });

    document.addEventListener("DOMContentLoaded", displayUsername);
}