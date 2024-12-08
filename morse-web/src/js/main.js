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

async function onMessageSubmit(event) {
    event.preventDefault();
    let message_element = document.getElementById("message-input");
    let message = message_element.value;
    if(!message || message.trim().length == 0) {
        return;
    }
    sendMessage(current_room, message);
    message_element.value = '';
}

async function onPageLoad() {
    addMessageHandler("room", async event => {
        onRoomMessage(event);
        let room_list = getRooms();
        updateRoomDisplay(room_list);
    });
    addMessageHandler("chat", async event => {
        let added_message = onChatMessage(event);
        addMessageToDisplay(added_message);
    });

    document.addEventListener("DOMContentLoaded", displayUsername);
    document.getElementById("message-form").onsubmit = onMessageSubmit;
}