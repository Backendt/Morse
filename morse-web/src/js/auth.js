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
    let json = await response.json();
    let is_response = "status" in json;
    if(is_response) {
        displayResponse(json);
        return;
    }

    let is_token = "token" in json;
    if(!is_token) {
        console.error("[ERROR] Received unexpected response for API");
        return;
    }

    let remember_me = document.getElementById("remember").checked;
    setToken(json["token"], remember_me);
    window.location.href = "/"; // Redirect to main page
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
    let user = getUserInput();
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
