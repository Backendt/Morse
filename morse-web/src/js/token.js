const TOKEN_STORAGE_KEY = "token";

function getUsername() {
    let token = getToken();
    if(token == null) {
        return null;
    }
    let jwt_payload = token.split('.')[1]; // JWT: header.payload.signature
    let decoded_payload = atob(jwt_payload);
    let jwt_claim = JSON.parse(decoded_payload);
    return jwt_claim["sub"];
}

function setToken(token, persist) {
    if(persist) {
        localStorage.setItem(TOKEN_STORAGE_KEY, token);
    } else {
        sessionStorage.setItem(TOKEN_STORAGE_KEY, token);        
    }
}

function hasToken() {
    return getToken() != null;
}

function getToken() {
    let token = localStorage.getItem(TOKEN_STORAGE_KEY);
    if(token == null) {
        token = sessionStorage.getItem(TOKEN_STORAGE_KEY);
    }
    return token;
}