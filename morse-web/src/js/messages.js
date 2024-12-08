const MESSAGE_LIST_ELEMENT = document.getElementById("messages");
let room_messages = new Map();

function getRoomMessages(room_id) {
    return room_messages.get(room_id) || [];
}

function removeRoomMessages(room_id) {
    room_messages.delete(room_id);
}

function onChatMessage(message) {
    let room = message.room;
    let stored_message = {
        time: Date.now(),
        sender: message.sender,
        content: message.content
    };
    let messages = getRoomMessages(room);
    messages.push(stored_message);
    room_messages.set(room, messages);
    return stored_message;
}

async function addMessageToDisplay(message) {
    let message_element = document.createElement("p");
    message_element.innerText = `${message.sender}: ${message.content}`;
    MESSAGE_LIST_ELEMENT.appendChild(message_element);
}

function updateMessageDisplay(message_list) {
    for(let message of message_list) {
        addMessageToDisplay(message);
    }
}