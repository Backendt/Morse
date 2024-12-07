function displayUsername() {
    let element = document.getElementById("my-name");
    element.innerText = getUsername();
}

async function onPageLoad() {
    addMessageHandler("room", async event => {
        onRoomMessage(event);
        let room_list = getRooms();
        fillSidebar(room_list);
    });
    
    document.addEventListener("DOMContentLoaded", displayUsername);
}