const ROOMS = new Map(); // room_id -> users

function getRooms() {
    return ROOMS.keys();
}

function getUsersInRoom(room_id) {
    return ROOMS.get(room_id) || [];
}

function setUsersInRoom(room_id, users) {
    if(users.length == 0) { // Delete room if empty
        ROOMS.delete(room_id);
    } else {
        ROOMS.set(room_id, users); 
    }
}

function onRoomMessage(message) {
    let room = message.room;
    let event_verb = message.event === "leave" ? "left" : "joined";
    console.info(`${message.event_user} ${event_verb} the room ${room}`);
    
    setUsersInRoom(room, message.users);
}