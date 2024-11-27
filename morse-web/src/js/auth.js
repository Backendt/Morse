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

async function handleResponse(response) {
    let json = await response.json();
    if(isMessage(json)) {
        displayResponse(json);
        return;
    }

    let is_token = "token" in json;
    if(!is_token) {
        console.log("[ERROR] Received unexpected response for API");
        return;
    }

    let remember_me = document.getElementById("remember").checked;
    setToken(json["token"], remember_me);
    window.location.href = "/chat"; // Redirect to /chat
}

function getUserInput() {
    let username = document.getElementById("username").value;
    let password = document.getElementById("password").value;
    return {username, password};
}

function submitLogin() {
    let user = getUserInput();
    postToApi("/login", user).then(handleResponse);
}

function submitRegister() {
    let user = getUserInput();
    postToApi("/register", user).then(handleResponse);
}

function anonymousLogin() {
    let user = getUserInput();
    let url = getApiUrl() + "/anonymous";
    fetch(url).then(handleResponse);
}

function setupButtons() {
    document.getElementById("register-button").onclick = submitRegister;
    document.getElementById("anonymous-button").onclick = anonymousLogin;
    document.getElementById("user-form").onsubmit = (event) => {
        event.preventDefault();
        submitLogin();
    };
}

document.addEventListener("DOMContentLoaded", setupButtons);
