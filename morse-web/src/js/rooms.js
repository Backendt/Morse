let current_room = null;
const rooms = new Map();

async function leaveRoom(room_id) {
    sendWsMessage({
        action: "leave",
        target: room_id
    });
}

async function createRoom() {
    sendWsMessage({
        action: "create_room"
    });
}

async function joinRoom(room_id) {
    sendWsMessage({
        action: "join",
        target: room_id
    });
}

async function displayRoom(room_id) {
    current_room = room_id;
    let room_users = getUsersInRoom(room_id);
    let room = document.getElementById("room");

    // TODO
    let all_users = "Users in room: " + room_users.join(", ");
    room.innerText = all_users;
}

function onCreatedRoom(room_id) {
    addUserToRoom(getUsername(), room_id);
    displayRoom(room_id);
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

function addUserToRoom(username, room_id) {
    let room_users = getUsersInRoom(room_id);
    room_users.push(username);
    rooms.set(room_id, room_users);
    // Update display if its currently displayed
    if(room_id === current_room) {
        displayRoom(room_id);
    }
}

function removeUserFromRoom(username, room_id) {
    let room_users = getUsersInRoom(room_id);
    let new_room_users = room_users.filter(name => name !== username);
    rooms.set(room_id, new_room_users);
    // Update display if its currently displayed
    if(room_id === current_room) {
        displayRoom(room_id);
    }
}