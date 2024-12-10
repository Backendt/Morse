const WEBSOCKET_ENDPOINT = "/chat";
const RECONNECTION_DELAY_SECONDS = 10;

let api_socket = null;
let message_handlers = new Map(); // Response type -> Handler function list
let message_queue = []; // Messages that tried to be sent when websocket wasn't established

function getWebsocketUrl() {
    return "ws://api." + window.location.host + WEBSOCKET_ENDPOINT;
}

async function establishWebsocket() {
    let token = getToken();
    if(token == null) {
        console.warn("Tried establishing websocket connection without being logged-in");
        window.location.href = "/login" + window.location.hash;
        return;
    }
    addMessageHandler("status", handleAuthError)
    connectToWebsocket(token);
}

function connectToWebsocket(token, is_retrying=false) {
    if(api_socket != null) {
        console.error("Tried connecting to websocket multiple times");
        return;
    };

    let socket_url = getWebsocketUrl();
    api_socket = new WebSocket(socket_url);

    api_socket.onopen = () => {
        is_retrying = false;
        console.log("Connected to websocket.");
        api_socket.send(token);

        let queue = Array.from(message_queue);
        message_queue = [];
        for(let message of queue) {
            sendWsMessage(message);
        }
    };

    api_socket.onerror = (event) => {
        console.error("The connection with the websocket has been closed because of an error: ", event);
    };

    api_socket.onclose = (event) => {
        api_socket = null;
        if(is_retrying) {
            setTimeout(() => connectToWebsocket(token, true), RECONNECTION_DELAY_SECONDS * 1000);
            return;
        }

        console.log("Disconnected from websocket with code: " + event.code + " (reason: '" + event.reason + "')");
        connectToWebsocket(token, true);
    };

    api_socket.onmessage = handleMessageEvent;
}

async function handleMessageEvent(event) {
    let message = {};
    try {
        message = JSON.parse(event.data);
    } catch {} // Ignore the parse error
    let type = message.type;
    if(!type) {
        console.warn(`Received unknown message from api: ${event.data}`);
        return;
    }

    let handlers = message_handlers.get(type) || [];
    if(handlers.length == 0) {
        console.warn(`No handler found for message of type ${type}. Message: ${event.data}`);
        return;
    }

    handlers.forEach(handler => handler(message.body));
}

function addMessageHandler(response_type, handler) {
    let handlers = message_handlers.get(response_type) || [];
    handlers.push(handler);
    message_handlers.set(response_type, handlers);
} 

function handleAuthError(status) {
    let auth_status_code = "invalid_token";
    let is_auth_error = status.status_code === auth_status_code;
    if(is_auth_error) {
        console.warn("Received an authentication error from API. Redirecting to login page.");
        removeTokens();
        window.location.href = "/login" + window.location.hash;
    }
}

function sendWsMessage(message) {
    if(!api_socket || api_socket.readyState != WebSocket.OPEN) {
        message_queue.push(message);
        return;
    }

    let json = JSON.stringify(message);
    api_socket.send(json);
}

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