const SUPRESSED_ERROR_CODES = ["parse_error"];

function notify(response) {
    // TODO
    alert(response.message);
}

function displayResponse(response) {
    let status_code = response.code;
    if(status_code in SUPRESSED_ERROR_CODES) {
        console.log(`Received response for suppressed error message '${status_code}': ${response.message}`);
        return;
    }

    if(response.status === "error") {
        notify(response);
    }
}