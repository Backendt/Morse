async function inviteToRoom(room_id, username) {
    sendWsMessage({
        action: "invite",
        target: username,
        body: room_id
    });
}

async function sendMessage(room_id, message) {
    sendWsMessage({
        action: "message",
        target: room_id,
        body: message.trim()
    });
}

function handleChatMessage(message) {
    // TODO
    alert(`[${message.sender}@${message.room}]: ${message.content}`);
}

function handleRequest(request) {
    switch(request.action) {
        case "invite": // Invitation received
            // TODO
            alert(`Invited to room: ${request.body}`);
            break;
        case "join": // User joined room
            addUserToRoom(request.body, request.target);
            break;
        case "leave": // User left room
            removeUserFromRoom(request.body, request.target);
            break;
    }
}

function handleResponse(response) {
    let is_error = response.status === "error";
    let is_room_creation = response.code === "room_creation";

    if(!is_error && is_room_creation) {
        onCreatedRoom(response.message);
    } else {
        displayResponse(response);
    }
}

async function onWsMessage(event) {
    let message;
    try {
        message = JSON.parse(event.data);
    } catch(error) {
        console.warn("Could not parse websocket message. Tried parsing: " + event.data, error);
        return;
    }

    let is_request = "action" in message;
    let is_response = "status" in message;
    let is_chat_message = "sender" in message;
    if(is_request) {
        handleRequest(message);
    } else if(is_response) {
        handleResponse(message);
    } else if(is_chat_message) {
        handleChatMessage(message);
    } else {
        console.error("Received unknown message from websocket: " + event.data);
    }
}

function displayUsername() {
    let nameElement = document.getElementById("my-name");
    nameElement.innerText = getUsername();
}

async function onPageLoad() {
    addMessageHandler(onWsMessage);
    document.addEventListener("DOMContentLoaded", displayUsername);
}