const NAME_ELEMENT = document.getElementById("my-name");
const NO_ROOM_ELEMENT = document.getElementById("no-room");
const ROOM_ELEMENT = document.getElementById("room");
const ROOM_USERS_ELEMENT = document.getElementById("room-users");
const ROOM_NAME_ELEMENT = document.getElementById("room-name");
const MESSAGE_INPUT_ELEMENT = document.getElementById("message-input");
const MESSAGE_LIST_ELEMENT = document.getElementById("messages");

let current_room = null;

async function displayUsername() {
    NAME_ELEMENT.innerText = getUsername();
}

// Room display

async function displayRoom(room_id) {
    current_room = room_id;
    let room_users = getUsersInRoom(room_id);

    let all_users = room_users.join(", ");
    ROOM_USERS_ELEMENT.innerText = all_users;
    ROOM_NAME_ELEMENT.innerText = room_id;
    displayRoomMessages(room_id);
}

async function updateRoomsDisplay(room_list) {
    fillSidebar(room_list);
    let has_no_room = room_list.length == 0;
    NO_ROOM_ELEMENT.hidden = !has_no_room;
    ROOM_ELEMENT.hidden = has_no_room;
}

// Message display

async function addMessageToDisplay(message) {
    let message_element = document.createElement("p");
    message_element.innerText = `${message.sender}: ${message.content}`;
    MESSAGE_LIST_ELEMENT.appendChild(message_element);
}

function displayRoomMessages(room_id) {
    MESSAGE_LIST_ELEMENT.innerText = ''; // Clear current messages display
    let message_list = getRoomMessages(room_id);
    for(let message of message_list) {
        addMessageToDisplay(message);
    }
}

async function onMessageSubmit(event) {
    event.preventDefault();
    let message = MESSAGE_INPUT_ELEMENT.value;
    MESSAGE_INPUT_ELEMENT.value = ''; // Clear input field
    
    if(message && message.trim().length > 0) {
        sendMessage(current_room, message);
    }
}

async function registerMessageHandlers() {
    addMessageHandler("room", async room_message => {
        let room = room_message.room;
        onRoomMessage(room_message);
        if(!current_room || room === current_room) {
            displayRoom(room);
        }
        let room_list = getRooms();
        updateRoomsDisplay(room_list);
    });
    addMessageHandler("chat", async chat_message => {
        let added_message = onChatMessage(chat_message);
        if(current_room === chat_message.room) {
            addMessageToDisplay(added_message);
        }
    });
}

async function onPageLoad() {
    registerMessageHandlers();
    establishWebsocket();
    document.addEventListener("DOMContentLoaded", displayUsername);
    document.getElementById("message-form").onsubmit = onMessageSubmit;
}