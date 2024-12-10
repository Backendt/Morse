function getApiUrl() {
    return window.location.protocol + "//api." + window.location.host;
}

function postToApi(path, body) {
    let url = getApiUrl() + path;
    let headers = {"Content-Type": "application/json"};

    return fetch(url, {
        method: "POST",
        headers,
        body: JSON.stringify(body)
    });
}

async function handleAuthResponse(response) {
    let message = await response.json();
    let type = message.type;
    if(!type) {
        console.warn(`Received unexpected json as authentication response: ${JSON.stringify(message)}`);
        return;
    }
    
    if(type == "token") {
        let remember_me = document.getElementById("remember").checked;
        setToken(message.body.token, remember_me);
        window.location.href = "/" + window.location.hash; // Redirect to main page
    } else if(type === "status") {
        displayStatus(message.body);
    } else {
        console.error(`Received unexpected message for authentication response. ${JSON.stringify(message)}`);
    }
}

function getUserInput() {
    let username = document.getElementById("username").value;
    let password = document.getElementById("password").value;
    return {username, password};
}

function submitLogin() {
    let user = getUserInput();
    postToApi("/login", user).then(handleAuthResponse);
}

function submitRegister() {
    let user = getUserInput();
    postToApi("/register", user).then(handleAuthResponse);
}

function anonymousLogin() {
    let url = getApiUrl() + "/anonymous";
    fetch(url).then(handleAuthResponse);
}

function setupButtons() {
    document.getElementById("register-button").onclick = submitRegister;
    document.getElementById("anonymous-button").onclick = anonymousLogin;
    document.getElementById("user-form").onsubmit = (event) => {
        event.preventDefault();
        submitLogin();
    };
}

async function onAuthPageLoad() {
    document.addEventListener("DOMContentLoaded", setupButtons);
}
