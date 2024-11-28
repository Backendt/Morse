const WEBSOCKET_ENDPOINT = "/chat";
const RECONNECTION_DELAY_SECONDS = 10;

var api_socket = null;
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

        console.log("Disconnected from websocket for reason: '" + event.reason + "' and code: " + event.code);
        connectToWebsocket(token, true);
    };

    api_socket.onmessage = onWsMessage;
}

async function onWsMessage(event) {
    // TODO
    alert("Received from ws: " + event.data);
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