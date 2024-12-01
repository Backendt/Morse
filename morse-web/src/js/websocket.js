const WEBSOCKET_ENDPOINT = "/chat";
const RECONNECTION_DELAY_SECONDS = 10;

let api_socket = null;
let message_handlers = []; // List of functions called to handle the websocket onMessage event
establishWebsocket();

function getWebsocketUrl() {
    return "ws://api." + window.location.host + WEBSOCKET_ENDPOINT;
}

async function establishWebsocket() {
    let token = getToken();
    if(token == null) {
        console.warn("Tried establishing websocket connection without being logged-in");
        window.location.href = "/login";
        return;
    }
    addMessageHandler(handleAuthError)
    connectToWebsocket(token);
}

function connectToWebsocket(token, is_retrying=false) {
    if(api_socket != null) {
        console.error("[ERROR] Tried connecting to websocket multiple times");
        return;
    };

    let socket_url = getWebsocketUrl();
    api_socket = new WebSocket(socket_url);

    api_socket.onopen = () => {
        is_retrying = false;
        console.log("Connected to websocket.");
        api_socket.send(token);
    };

    api_socket.onerror = (event) => {
        console.error("[ERROR] The connection with the websocket has been closed because of an error: ", event);
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
    for(let handler of message_handlers) {
        handler(event);
    }
}

function addMessageHandler(handler) {
    message_handlers.push(handler);
} 

function handleAuthError(event) {
    let json = JSON.parse(event.data);
    let error_code_key = "code";
    if(!(error_code_key in json)) {
        return;
    }

    let auth_error_code = "invalid_auth";
    let is_auth_error = json[error_code_key] === auth_error_code;
    if(is_auth_error) {
        console.warn("Received an authentication error from API. Redirecting to login page.");
        removeTokens();
        window.location.href = "/login";
    }
}

function sendWsMessage(message) {
    if(api_socket == null) {
        console.error("[ERROR] Tried sending websocket message before establishing connection.");
        return;
    }
    if(api_socket.readyState != WebSocket.OPEN) {
        console.error("[ERROR] Tried sending websocket message but connection isn't open. Current status: ", api_socket.readyState);
        return;
    }

    let json = JSON.stringify(message);
    api_socket.send(json);
}
