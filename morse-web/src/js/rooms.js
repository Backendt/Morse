let current_room = null;
const rooms = new Map(); // room_id -> users

async function displayRoom(room_id) {
    current_room = room_id;
    let room_users = getUsersInRoom(room_id);
    let room = document.getElementById("room");
    let room_list = document.getElementById("room-list");

    // TODO
    let all_users = "Users in room: " + room_users.join(", ");
    room.innerText = all_users;
}

function getRooms() {
    return rooms.keys();
}

function getCurrentRoom() {
    return current_room;
}

function getUsersInRoom(room_id) {
    return rooms.get(room_id) || [];
}

function setUsersInRoom(room_id, users) {
    if(users.length == 0) { // Delete room if empty
        rooms.delete(room_id);
    } else {
        rooms.set(room_id, users); 
    }
}

function onRoomMessage(message) {
    let room = message.room;

    let event_verb = message.event === "leave" ? "left" : "joined";
    console.info(`${message.event_user} ${event_verb} the room ${room}`);
    setUsersInRoom(room, message.users);
    if(!current_room || room === current_room) {
        displayRoom(room);
    }
}