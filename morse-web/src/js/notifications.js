const SUPRESSED_ERROR_CODES = ["parse_error"];

function notify(response) {
    // TODO
    alert(response.message);
}

function displayStatus(response) {
    let status_code = response.status_code;
    if(status_code in SUPRESSED_ERROR_CODES) {
        console.log(`Received response for suppressed error message '${status_code}': ${response.message}`);
        return;
    }

    notify(response);
}