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

