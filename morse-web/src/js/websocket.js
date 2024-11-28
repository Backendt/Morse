const WEBSOCKET_ENDPOINT = "/stream";

var api_socket = null;
tryEstablishingWebsocket();

function getWebsocketUrl() {
    return "ws://api." + window.location.host + "/stream";
}

async function tryEstablishingWebsocket() {
    let token = getToken();
    if(token == null) {
        console.warn("Tried establishing websocket connection without being logged-in");
        window.location.href = "/login";
        return;
    }
    establishWebsocket(token);
}

function establishWebsocket(token) {
    if(api_socket != null) {
        console.error("[ERROR] Tried establishing websocket connection multiple times");
        return;
    };

    let socket_url = getWebsocketUrl();
    api_socket = new WebSocket(socket_url); // Looks like there is not way to send Authorization header... time to change server side implementation

    api_socket.onopen = () => {
        console.log("Connected to websocket");
    };

    api_socket.onerror = (event) => {
        console.error("[ERROR] The connection with the websocket has been closed because of an error: ", event);
    };

    api_socket.onclose = () => {
        console.log("Disconnected from websocket");
        api_socket = null;
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