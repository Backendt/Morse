const MESSAGES = new Map(); // room_id -> messages

function getRoomMessages(room_id) {
    return MESSAGES.get(room_id) || [];
}

function removeRoomMessages(room_id) { // TODO Use on room deletion
    MESSAGES.delete(room_id);
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
    MESSAGES.set(room, messages);
    return stored_message;
}

