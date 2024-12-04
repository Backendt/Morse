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

function displayUsername() {
    let nameElement = document.getElementById("my-name");
    nameElement.innerText = getUsername();
}

async function onPageLoad() {
    addMessageHandler("room", onRoomMessage);
    document.addEventListener("DOMContentLoaded", displayUsername);
}